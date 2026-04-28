use tracing::{Level, debug, error, info, instrument, span, trace, warn};

fn test_basic() {
    debug!("this is debug");
    info!("this is info");
    warn!("this is warn");
    error!("this is error");
    trace!("this is trace");
}

fn test_span_sync() {
    let root = span!(Level::INFO, "parent_storage_task");
    let _parent_guard = root.enter();
    info!("this is parent info");
    {
        let child = span!(Level::INFO, "child_storage_task");
        let _child_guard = child.enter();
        info!("this is child info");
    }
    info!("back to parent info");
}

#[instrument]
async fn test_span_async_child() {
    info!("this is child info");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    info!("back to child info");
}

#[instrument]
async fn test_span_async_parent() {
    info!("this is parent info");
    test_span_async_child().await;
    info!("back to parent info");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .init();

    test_basic();
    test_span_sync();
    test_span_async_parent().await;
}
