fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&[
        "src/types/vendor/PreloadedUserSettings.proto",
        "src/types/vendor/FrecencyUserSettings.proto",
        "src/types/vendor/GuildSettings.proto",
    ], &["src/vendor/"])?;
    Ok(())
}
