use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BotConfig {
    pub homeserver_url: String,
    pub username: String,
    pub password: String,
}

impl BotConfig {
    pub fn load() -> Result<BotConfig, ConfyError> {
        let cfg: BotConfig = confy::load("matrix-tipping")?;

        Ok(cfg)
    }
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            homeserver_url: "https://matrix.org".to_owned(),
            username: "".to_owned(),
            password: "".to_owned(),
        }
    }
}
