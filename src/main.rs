use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{env, fs::File, process::Command};
use warp::{body, Filter};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Deserialize, Serialize)]
struct KotlinSrc {
    src: String,
}

#[tokio::main]
async fn main() {
    let compile = warp::path("compile")
        .and(warp::post())
        .and(body::json())
        .map(|src: KotlinSrc| {
            format!(
                "{}",
                match compile_file(src.src) {
                    Ok(_) => "success".to_string(),
                    Err(err) => err,
                }
            )
        });

    let socket_addr = if env::args().any(|arg| arg == "--production") {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80)
    } else {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    };

    warp::serve(compile).run(socket_addr).await;
}

fn compile_file(kotlin_src: String) -> Result<String, String> {
    let temp_dir = env::temp_dir();
    let src_file_path = temp_dir.join("in.kt");
    let mut src_file = File::create(src_file_path.clone()).map_err(|err| err.to_string())?;
    write!(src_file, "{}", kotlin_src).map_err(|err| err.to_string())?;
    let out = Command::new("kotlinc")
        .arg(src_file_path.as_os_str())
        .arg("-d")
        .arg(temp_dir.as_os_str())
        .output()
        .map_err(|err| err.to_string())?;
    let stderr = std::str::from_utf8(out.stderr.as_slice()).map_err(|err| err.to_string())?;
    if stderr.is_empty() {
        Ok("compield successfully".to_string())
    } else {
        Err(stderr.to_string())
    }
}
