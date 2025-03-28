pub struct StorageService {}

impl StorageService {
    pub async fn new() -> anyhow::Result<Self> {
        println!("Creating new StorageService");
        Ok(Self {})
    }

    pub async fn write_trunsactions(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
