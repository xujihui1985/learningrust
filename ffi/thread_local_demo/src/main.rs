use std::sync::{Arc, RwLock};

fn main() {
    let config = Config::current();
    config.set_debug_mode(true);
    if config.debug_mode() {
        println!("in debug mode");
    }
}

#[derive(Default)]
struct ConfigInner {
    debug_mode: bool
}


#[derive(Default)]
struct Config {
    inner: RwLock<ConfigInner>,
}

impl Config {
    pub fn new() ->Arc<Self> {
        Arc::new(Config{inner: RwLock::new(Default::default())})
    }

    pub fn current() -> Arc<Self> {
        CURRENT_CONFIG.with(|c| c.clone())
    }

    pub fn debug_mode(&self) -> bool {
        self.inner.read().unwrap().debug_mode
    }

    pub fn set_debug_mode(&self, value: bool) {
        self.inner.write().unwrap().debug_mode = value;
    }
}

thread_local! {
    static CURRENT_CONFIG: Arc<Config> = Config::new();
}

