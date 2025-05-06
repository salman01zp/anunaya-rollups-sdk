/// Sets up the logger for the relayer, based on the verbosity level passed in.
///
/// Returns `Ok(())` on success, or `Err(anyhow::Error)` on failure.
///
/// # Arguments
///
/// * `verbosity` - An i32 integer representing the verbosity level.
/// * `filter` -  An &str representing filtering directive for EnvFilter
pub fn setup_logger(verbosity: i32, filter: &str) -> anyhow::Result<()> {
    use tracing::Level;
    let log_level = match verbosity {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        _ => Level::TRACE,
    };
    let directive_1 = format!("{filter}={log_level}")
        .parse()
        .expect("valid log level");
    let directive_2 = format!("sequencer_={log_level}")
        .parse()
        .expect("valid log level");
    let env_filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(directive_1)
        .add_directive(directive_2);
    let logger = tracing_subscriber::fmt()
        .with_target(true)
        .with_max_level(log_level)
        .with_env_filter(env_filter);
    let logger = logger.pretty();
    logger.init();
    Ok(())
}
