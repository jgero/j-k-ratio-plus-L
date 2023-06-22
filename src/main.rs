use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{env, fs::File, process::Command};
use uuid::Uuid;
use warp::{body, Filter};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod scoreboard;

#[derive(Deserialize, Serialize)]
struct KotlinSrc {
    src: String,
    user: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct CompressionRatio {
    chars: f32,
    lines: f32,
}

impl Eq for CompressionRatio {}

#[derive(Deserialize, Serialize)]
struct JavaResponse {
    src: Vec<String>,
    compression_ratio: CompressionRatio,
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() {
    let scoreboard = Arc::new(Mutex::new(crate::scoreboard::Scoreboard::new()));
    let compile_endpoint_scoreboard = scoreboard.clone();
    let compile = warp::path("compile")
        .and(warp::post())
        .and(body::json())
        .map(move |src: KotlinSrc| match compile_file(&src.src) {
            Ok(result) => {
                let ratio = CompressionRatio {
                    chars: char_compression_ratio(&result, &src.src),
                    lines: line_compression_ratio(&result, &src.src),
                };
                {
                    let mut locked = compile_endpoint_scoreboard.lock().unwrap();
                    locked.new_entry(src.user, ratio.clone());
                }
                warp::reply::json(&JavaResponse {
                    src: result.clone(),
                    compression_ratio: ratio,
                })
            }
            Err(err) => warp::reply::json(&ErrorResponse { error: err }),
        });
    let scoreboard_endpoint_scoreboard = scoreboard.clone();
    let scoreboard_path = warp::path("scoreboard")
        .map(move || warp::reply::json(&scoreboard_endpoint_scoreboard.lock().unwrap().get()));

    let socket_addr = if env::args().any(|arg| arg == "--production") {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80)
    } else {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    };
    let static_dir = env::args()
        .find(|arg| arg.contains("--static-path="))
        .map(|arg| arg.replace("--static-path=", ""))
        .unwrap_or_else(|| "build".to_string());

    let (_addr, future) = warp::serve(
        warp::path::end()
            .and(warp::fs::file(static_dir.clone() + "/index.html"))
            .or(warp::path("_app").and(warp::fs::dir(static_dir.clone() + "/_app")))
            .or(compile)
            .or(scoreboard_path),
    )
    .bind_with_graceful_shutdown(socket_addr, async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen to shutdown signal");
    });
    future.await;
    println!("shutting down server");
}

fn line_compression_ratio(val1: &Vec<String>, val2: &String) -> f32 {
    let lines = val1.iter().map(|s| s.lines().count()).fold(0, |a, b| a + b);
    lines as f32 / val2.lines().count() as f32
}

fn char_compression_ratio(val1: &Vec<String>, val2: &String) -> f32 {
    let chars = val1
        .iter()
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).count())
        .fold(0, |a, b| a + b);
    chars as f32 / val2.chars().filter(|c| !c.is_whitespace()).count() as f32
}

fn compile_file(kotlin_src: &String) -> Result<Vec<String>, String> {
    // setup temp dir
    let temp_dir = env::temp_dir().join(Uuid::new_v4().to_string());
    if !temp_dir.exists() {
        match fs::create_dir_all(temp_dir.clone()) {
            Ok(_) => println!(
                "created dir {}",
                temp_dir.clone().to_str().get_or_insert("unknown")
            ),
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

    let out_dir = temp_dir.join("out");
    if let Err(err) = fs::create_dir_all(out_dir.clone()) {
        return Err(format!(
            "temp dir was missting and creating it failed: {}",
            err
        ));
    }

    // decompile class file to java
    let out = Command::new("jd-cli")
        .arg("-od")
        .arg(out_dir.as_os_str())
        .arg(temp_dir.clone())
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
    let result = match fs::read_dir(out_dir) {
        Ok(read_dir) => Ok(read_dir
            .filter(|el| {
                el.as_ref().unwrap().file_type().unwrap().is_file()
                    && el.as_ref().unwrap().path().extension().unwrap() == "java"
            })
            .map(|file| fs::read_to_string(file.unwrap().path()).unwrap())
            .collect()),
        Err(err) => Err(err.to_string()),
    };

    if let Err(err) = fs::remove_dir_all(temp_dir) {
        return Err(err.to_string());
    }
    return result;
}
