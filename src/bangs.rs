use std::collections::HashMap;

use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bang {
    #[serde(default)]
    pub c: Option<String>, // Category
    pub d: String, // Direct Link
    #[serde(default)]
    pub r: Option<u32>,
    #[serde(default)]
    pub sc: Option<String>, // Secondary Category
    pub t: String,
    pub u: String, // URL template
}

pub struct BangMap {
    bangs: HashMap<String, Bang>,
}

impl BangMap {
    pub fn new(bangs: HashMap<String, Bang>) -> Self {
        BangMap { bangs }
    }

    pub fn resolve_bang(&self, tag: &String, query: Option<Vec<String>>) -> Result<String> {
        let bang = self
            .bangs
            .get(tag)
            .ok_or_else(|| anyhow!("Bang '{}' not found", tag))?;
        debug!("Bang resolved: {:?}", bang);
        if let Some(query) = query {
            let joined = query.join(" ");
            let encoded_query = urlencoding::encode(&joined);
            let template_with_query = bang.u.replace("{{{s}}}", &encoded_query);
            Ok(template_with_query)
        } else {
            Ok(bang.d.clone())
        }
    }
}
