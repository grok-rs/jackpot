use crate::domain::models::WagerRequest;

pub struct StorageService {
    repository: String,
}

impl StorageService {
    pub async fn new() -> anyhow::Result<Self> {
        println!("Creating new StorageService");
        let repository = "SomeRepository".to_string();
        Ok(Self { repository })
    }

    pub async fn write_trunsactions(&self, request: &WagerRequest) -> anyhow::Result<()> {
        Ok(())
    }
}
