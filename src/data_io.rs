use anyhow::{self, Result};
use log::info;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::bangs::Bang;

pub struct DataIO {
    data_dir: PathBuf,
    pub bin_dir: PathBuf,
}

impl DataIO {
    pub fn new() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
            .join("banger");

        Ok(Self {
            bin_dir: data_dir.join("bangs.bin"),
            data_dir,
        })
    }

    pub fn clean_data_dir(&self) -> Result<()> {
        info!(
            "Attempting to clean data directory: {}",
            self.data_dir.display()
        );
        if self.bin_dir.exists() {
            info!("Data directory found, attempting to remove...");
            fs::remove_dir_all(&self.data_dir)?;
            info!("✅ Successfully cleaned data directory.");
        } else {
            info!("Data directory not found, no cleaning needed.");
        }
        Ok(())
    }

    fn create_data_dir(&self) -> Result<()> {
        if !self.data_dir.exists() {
            fs::create_dir_all(self.data_dir.clone()).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to create data directory {}: {}",
                    self.data_dir.display(),
                    e
                )
            })
        } else {
            info!("Data directory already created");
            Ok(())
        }
    }

    async fn fetch_bangs(&self) -> Result<Vec<Bang>> {
        info!("Fetching data from duckduckgo.com/bangs.js...");
        let response = reqwest::get("https://duckduckgo.com/bang.js").await?;
        let text = response.text().await?;

        info!("Parsing JSON data...");
        let bangs: Vec<Bang> = serde_json::from_str(&text)?;
        Ok(bangs)
    }

    fn save_binary(&self, bangs: Vec<Bang>) -> Result<()> {
        let mut map: HashMap<String, Bang> = HashMap::new();
        for bang in bangs {
            map.insert(bang.t.clone(), bang);
        }

        let bytes = bincode::serialize(&map)?;
        fs::write(&self.bin_dir, bytes)?;
        info!("✅ Wrote {} bangs to {}", map.len(), self.bin_dir.display());

        Ok(())
    }

    pub async fn build_bangs(&self) -> Result<()> {
        self.create_data_dir()?;
        let bangs_json = self.fetch_bangs().await?;
        self.save_binary(bangs_json)
    }

    pub fn read_bangs_binary(&self) -> Result<HashMap<String, Bang>> {
        let bytes = fs::read(&self.bin_dir)
            .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", self.bin_dir.display(), e))?;
        let bangs = bincode::deserialize(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize bangs.bin: {}", e))?;
        Ok(bangs)
    }
}
