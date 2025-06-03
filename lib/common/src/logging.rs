use tracing::Level;

pub fn init_tracing(level: Level) {
    eprintln!(
        r#"__  ___ _  __                  ____  _____
\ \/ (_) |/ /_  ______ _____  / __ \/ ___/
 \  / /|   / / / / __ `/ __ \/ /_/ /\__ \ 
 / / //   / /_/ / /_/ / / / / _, _/___/ / 
/_/_//_/|_\__,_/\__,_/_/ /_/_/ |_|/____/  
                                          "#
    );

    tracing_subscriber::fmt().with_max_level(level).init()
}
