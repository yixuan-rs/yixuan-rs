use tracing::Level;

pub fn init_tracing(level: Level) {
    tracing_subscriber::fmt().with_max_level(level).init()
}
