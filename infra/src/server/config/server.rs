#[derive(Debug, Clone)]
pub struct BasicServerConfig {
    pub addr: String,
    pub port: u16,
}

impl BasicServerConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BasicServerConfig {
    fn default() -> Self {
        Self {
            addr: String::from("127.0.0.1"),
            port: 7777,
        }
    }
}
