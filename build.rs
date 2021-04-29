fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/")
        .compile(&["proto/heartbeat.proto"], &["proto"])
        .unwrap();

    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(serde_derive::Deserialize, serde_derive::Serialize)]",
        )
        .type_attribute(".", "#[serde(rename_all = \"kebab-case\")]")
        .out_dir("src/")
        .compile(&["proto/daemon.proto"], &["proto"])
        .unwrap();

    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(serde_derive::Serialize, serde_derive::Deserialize)]",
        )
        .type_attribute(".", "#[serde(rename_all = \"kebab-case\")]")
        .out_dir("src/")
        .compile(&["proto/raft.proto"], &["proto"])
        .unwrap();

    Ok(())
}