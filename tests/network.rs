//! Integration test for the network preset.

use storage_bindings::{NetworkPreset, StorageConfig, StorageNode};
use tempfile::tempdir;

#[tokio::test(flavor = "multi_thread")]
async fn test_network() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let temp_dir = tempdir()?;

    let config = StorageConfig::new()
        .data_dir(temp_dir.path().join("storage_data"))
        .network(NetworkPreset::LogosDev)
        .nat("extip:127.0.0.1")
        .no_bootstrap_node(true);

    let node = StorageNode::new(config).await?;
    node.start().await?;

    assert_eq!(node.network().await?, NetworkPreset::LogosDev.to_string());

    Ok(())
}
