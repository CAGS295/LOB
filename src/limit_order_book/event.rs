use serde::Deserialize;

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
