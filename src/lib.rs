pub use libic::*;

pub async fn get_minecraft_versions() -> MinecraftVersionResult {
    let resp =
        util::get("https://launchermeta.mojang.com/mc/game/version_manifest.json".to_string());

    return serde_json::from_str(&resp).unwrap();
}
