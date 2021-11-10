
use std::env;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //stops building if this env is available .env files are not looked at
    if let Err(_) = env::var("STOP_BUILD_PROTO") {
        tonic_build::configure()
            .build_server(true)
            .build_client(false)
            .out_dir("src/proto_build")
            .compile(&["proto/player.proto"], &["proto/"])?;
    }

    Ok(())
}
