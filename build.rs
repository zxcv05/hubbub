fn main() -> Result<(), Box<dyn std::error::Error>> {
    // No clue if this is good practice
    // But hopefully it makes UX better?
    // Mostly adding this because of github actions
    std::env::set_var("PROTOC", protobuf_src::protoc());

    prost_build::compile_protos(&[
        "src/types/vendor/PreloadedUserSettings.proto",
        "src/types/vendor/FrecencyUserSettings.proto",
        "src/types/vendor/GuildSettings.proto",
    ], &["src/vendor/"])?;
    Ok(())
}
