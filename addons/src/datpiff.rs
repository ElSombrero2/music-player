use anyhow::Error;

use super::{Addon, StreamingAddon};

pub struct DatPiff {
    name: String,
    version: String,
    author: String,
    description: String,
    enabled: bool,
}

impl DatPiff {
    pub fn new() -> Self {
        Self {
            name: "DatPiff".to_string(),
            version: "0.1.0".to_string(),
            author: "Tsiry Sandratraina".to_string(),
            description: "DatPiff addon".to_string(),
            enabled: true,
        }
    }
}

impl Addon for DatPiff {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn author(&self) -> &str {
        &self.author
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl StreamingAddon for DatPiff {
    fn stream(&self, url: &str) -> Result<(), Error> {
        todo!("Implement DatPiff::stream");
    }
}
