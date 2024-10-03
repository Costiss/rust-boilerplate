#[derive(Debug, Clone)]
pub struct ConfigModule {
    pub environment: String,
}

impl ConfigModule {
    pub fn new() -> Self {
        ConfigModule {
            environment: "development".to_string(),
        }
    }
}
