use rust_challange::RequestMonitor;

struct ConsoleLogger;

// 1. Log for Monitor
impl RequestMonitor for ConsoleLogger {
    fn on_start(&self, symbol: &str) {
        println!("[Log] Starting fetch request for: '{}'", symbol);
    }

    fn on_finish(&self, symbol: &str, success: bool) {
        let status = if success { "Success" } else { "❌ Failed" };
        println!(
            "[Log] Finished fetch request for: '{}' -> {}",
            symbol, status
        );
    }
}

#[tokio::main]
async fn main() {
    // using this url for call api
    // a_provider
    // let get_a_provider = get_url_a_provider().await;
    // let url_a_provider = get_a_provider.url;

    // b_provider
    // let url_b_provider = get_url_b_provider().await.url;
    // let url_b_provider = get_b_provider.url;
}
