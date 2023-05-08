use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::{env, fs::File, process::Command};
use warp::{body, Filter};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Deserialize, Serialize)]
struct KotlinSrc {
    src: String,
}

#[derive(Deserialize, Serialize)]
struct CompressionRatio {
    chars: f32,
    lines: f32,
}

#[derive(Deserialize, Serialize)]
struct JavaResponse {
    src: String,
    compression_ratio: CompressionRatio,
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() {
    let compile = warp::path("compile")
        .and(warp::post())
        .and(body::json())
        .map(|src: KotlinSrc| match compile_file(&src.src) {
            Ok(result) => warp::reply::json(&JavaResponse {
                src: result.clone(),
                compression_ratio: CompressionRatio {
                    chars: char_compression_ratio(&result, &src.src),
                    lines: line_compression_ratio(&result, &src.src),
                },
            }),
            Err(err) => warp::reply::json(&ErrorResponse { error: err }),
        });

    let socket_addr = if env::args().any(|arg| arg == "--production") {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80)
    } else {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    };
    let static_dir = env::args()
        .find(|arg| arg.contains("--static-path="))
        .map(|arg| arg.replace("--static-path=", ""))
        .unwrap_or_else(|| "build".to_string());

    warp::serve(
        warp::path::end()
            .and(warp::fs::file(static_dir.clone() + "/index.html"))
            .or(warp::path("_app").and(warp::fs::dir(static_dir.clone() + "/_app")))
            .or(compile),
    )
    .run(socket_addr)
    .await;
}

fn line_compression_ratio(val1: &String, val2: &String) -> f32 {
    val1.lines().count() as f32 / val2.lines().count() as f32
}

fn char_compression_ratio(val1: &String, val2: &String) -> f32 {
    val1.chars().filter(|c| !c.is_whitespace()).count() as f32
        / val2.chars().filter(|c| !c.is_whitespace()).count() as f32
}

fn compile_file(kotlin_src: &String) -> Result<String, String> {
    // setup temp dir
    let temp_dir = env::temp_dir();
    if !temp_dir.exists() {
        match fs::create_dir(temp_dir.clone()) {
            Ok(_) => println!("temp dir does not exist, created it"),
            Err(err) => println!("temp dir was missting and creating it failed: {}", err),
        };
    }

    // write kotlin file and compile
    let src_file_path = temp_dir.join("in.kt");
    let mut src_file = File::create(src_file_path.clone()).map_err(|err| err.to_string())?;
    write!(src_file, "{}", kotlin_src).map_err(|err| err.to_string())?;
    let out = Command::new("kotlinc")
        .arg(src_file_path.as_os_str())
        .arg("-d")
        .arg(temp_dir.as_os_str())
        .output()
        .map_err(|err| err.to_string())?;
    if !out.status.success() {
        let stderr = std::str::from_utf8(out.stderr.as_slice()).map_err(|err| err.to_string())?;
        return Err(stderr.to_string());
    }

    // decompile class file to java
    let out = Command::new("jd-cli")
        .arg("-g")
        .arg("OFF")
        .arg("-oc")
        .arg(temp_dir)
        .output()
        .map_err(|err| err.to_string())?;
    if !out.status.success() {
        let stderr = std::str::from_utf8(out.stdout.as_slice()).map_err(|err| err.to_string())?;
        return Err(format!(
            "decompilation failed(status {}): {}",
            out.status,
            stderr.to_string()
        ));
    }
    let stdout = std::str::from_utf8(out.stdout.as_slice()).map_err(|err| err.to_string())?;
    Ok(stdout.to_string())
}
