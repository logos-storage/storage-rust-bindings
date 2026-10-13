//! Integration tests for private downloads through Mix.

use storage_bindings::{
    fetch, upload_chunk, upload_finalize, upload_init, FetchOptions, StorageConfig, StorageNode,
    UploadOptions,
};
use tempfile::tempdir;

/// An isolated node: no bootstrap peers, which libstorage only allows with an external IP.
fn base_config(dir: &std::path::Path) -> StorageConfig {
    StorageConfig::new()
        .data_dir(dir.join("storage_data"))
        .nat("extip:127.0.0.1")
        .no_bootstrap_node(true)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_private_fetch_without_mix_fails() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let temp_dir = tempdir()?;

    let node = StorageNode::new(base_config(temp_dir.path())).await?;
    node.start().await?;

    let session_id = upload_init(&node, &UploadOptions::new().filepath("hello.txt")).await?;
    upload_chunk(&node, &session_id, b"Hello World!".to_vec()).await?;
    let cid = upload_finalize(&node, &session_id).await?;

    let err = fetch(&node, &cid, &FetchOptions::new().private(true))
        .await
        .expect_err("a private fetch without Mix configured should fail");

    assert!(
        err.to_string().contains("Mix transport is not enabled"),
        "unexpected error: {}",
        err
    );

    Ok(())
}
