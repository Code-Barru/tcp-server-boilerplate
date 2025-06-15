pub fn start_logger() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::TRACE)
        .compact()
        .init();
}
