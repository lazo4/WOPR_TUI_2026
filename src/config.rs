use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Settings {
    pub provider: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub kind: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub token_budget: u32,
    pub cache_enabled: bool,
}

// ponytail: only deserialize the slices of ~/.blumi/settings.json we need
#[derive(Deserialize, Default)]
struct BlumiConfig {
    #[serde(default)]
    llm: BlumiLlm,
    #[serde(default)]
    providers: HashMap<String, BlumiProvider>,
}

#[derive(Deserialize, Default)]
struct BlumiLlm {
    #[serde(default)]
    provider: Option<String>,
    #[serde(default)]
    model: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
struct BlumiProvider {
    api_key: Option<String>,
    base_url: Option<String>,
    #[serde(default = "default_kind")]
    kind: String,
}

fn default_kind() -> String {
    "anthropic".into()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            provider: "stub".into(),
            api_key: None,
            base_url: None,
            model: None,
            kind: "anthropic".into(),
            temperature: 0.8,
            max_tokens: 2048,
            token_budget: 100_000,
            cache_enabled: true,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let path = PathBuf::from(&home).join(".blumi").join("settings.json");

        let blumi: BlumiConfig = match std::fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => return Self::default(),
        };

        let provider_name = blumi.llm.provider.unwrap_or_else(|| "stub".into());
        let resolved = blumi
            .providers
            .get(&provider_name)
            .cloned()
            .unwrap_or_default();

        Self {
            provider: provider_name,
            api_key: resolved.api_key,
            base_url: resolved.base_url,
            model: blumi.llm.model,
            kind: resolved.kind,
            ..Self::default()
        }
    }
}
