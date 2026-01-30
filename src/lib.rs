// use mock_api::{get_url_a_provider, get_url_b_provider};

pub mod mock_api;

// Monitor Trait
pub trait RequestMonitor {
    fn on_start(&self, symbol: &str);
    fn on_finish(&self, symbol: &str, success: bool);
}

// Configuration
pub struct Config<'a> {
    pub base_url: &'a str,
}
