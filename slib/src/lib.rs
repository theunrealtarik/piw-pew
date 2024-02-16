pub mod net {
    use std::time::Duration;

    pub const PROTOCOL_ID: u64 = 69;
    pub const DELTA_TIME: Duration = Duration::from_millis(12);
}

pub mod logging {
    use env_logger::{self, Env};

    pub struct Logger;
    impl Logger {
        pub fn env() -> Env<'static> {
            let env = Env::default()
                .filter_or("RUST_LOG", "trace")
                .write_style_or("RUST_STYLE_LOG", "always");
            env
        }
    }
}
