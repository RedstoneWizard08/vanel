pub mod format;
pub use format::*;

pub async fn get_minecraft_versions() -> MinecraftVersionResult {
    let client = reqwest::Client::new();
    let res = client
        .get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await;

    let resp = res.unwrap();
    let text = resp.text().await.unwrap();

    return serde_json::from_str(&text).unwrap();
}
