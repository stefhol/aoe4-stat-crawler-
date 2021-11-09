fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/proto_build")
        .compile(&["proto/player.proto"], &["proto/"])?;

    Ok(())
}
