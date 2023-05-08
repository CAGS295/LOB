use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Deserialize, Default)]
pub struct Event {
    #[cfg(feature = "event-id")]
    #[serde(alias = "e")]
    pub id: String,
    #[cfg(feature = "event-time")]
    #[serde(alias = "E")]
    pub time: u64,
    #[cfg(feature = "event-symbol")]
    #[serde(alias = "s")]
    pub symbol: String,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "event-id")]
        write!(f, "id: {} ", self.id)?;
        #[cfg(feature = "event-time")]
        write!(f, "time: {} ", self.time)?;
        #[cfg(feature = "event-symbol")]
        write!(f, "symbol: {} ", self.symbol)?;
        Ok(())
    }
}
