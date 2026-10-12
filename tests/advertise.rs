//! Integration tests for the advertise state of a dataset.

use storage_bindings::{
    get_advertise, set_advertise, upload_chunk, upload_finalize, upload_init, StorageConfig,
    StorageNode, UploadOptions,
};
use tempfile::tempdir;

/// An isolated node: no bootstrap peers, which libstorage only allows with an external IP.
fn base_config(dir: &std::path::Path) -> StorageConfig {
    StorageConfig::new()
        .data_dir(dir.join("storage_data"))
        .nat("extip:127.0.0.1")
        .no_bootstrap_node(true)
}

async fn upload(
    node: &StorageNode,
    options: UploadOptions,
) -> Result<String, Box<dyn std::error::Error>> {
    let session_id = upload_init(node, &options).await?;
    upload_chunk(node, &session_id, b"Hello World!".to_vec()).await?;
    Ok(upload_finalize(node, &session_id).await?)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_upload_is_advertised_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let temp_dir = tempdir()?;

    let node = StorageNode::new(base_config(temp_dir.path())).await?;
    node.start().await?;

    let cid = upload(&node, UploadOptions::new().filepath("hello.txt")).await?;

    assert!(
        get_advertise(&node, &cid).await?,
        "cid should be advertised"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_upload_without_advertise() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let temp_dir = tempdir()?;

    let node = StorageNode::new(base_config(temp_dir.path())).await?;
    node.start().await?;

    let options = UploadOptions::new().filepath("hello.txt").advertise(false);
    let cid = upload(&node, options).await?;

    assert!(
        !get_advertise(&node, &cid).await?,
        "cid should not be advertised"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_set_advertise() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::try_init();
    let temp_dir = tempdir()?;

    let node = StorageNode::new(base_config(temp_dir.path())).await?;
    node.start().await?;

    let cid = upload(&node, UploadOptions::new().filepath("hello.txt")).await?;

    set_advertise(&node, &cid, false).await?;

    assert!(
        !get_advertise(&node, &cid).await?,
        "cid should not be advertised"
    );

    Ok(())
}
