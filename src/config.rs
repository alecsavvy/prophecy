#[derive(Debug, Clone)]
pub struct Config {
    pub web_server_addr: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web_server_addr: "127.0.0.1:0".to_string(),
        }
    }
}
