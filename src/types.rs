//! Binance ticker data types.

use serde::Deserialize;

/// A Binance 24h ticker entry.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct CryptoTickerEntry {
    /// Trading pair symbol (e.g., "BTCUSDT")
    pub symbol: String,
    /// Current price
    pub price: Option<f64>,
    /// 24h price change percentage
    #[serde(rename = "priceChangePercent")]
    pub change_24h: Option<f64>,
    /// 24h volume in quote currency
    pub volume: Option<f64>,
    /// 24h high price
    pub high: Option<f64>,
    /// 24h low price
    pub low: Option<f64>,
}

/// Raw Binance combined stream message.
#[derive(Clone, Debug, Deserialize)]
pub struct BinanceStreamMessage {
    /// Stream name (e.g., "btcusdt@ticker")
    #[serde(rename = "stream")]
    pub stream: String,
    /// Parsed ticker data
    pub data: BinanceTickerData,
}

/// Raw Binance ticker data payload.
#[derive(Clone, Debug, Deserialize)]
pub struct BinanceTickerData {
    /// Trading pair symbol
    pub s: String,
    /// Current price (string)
    pub c: String,
    /// Price change percentage (string)
    pub P: String,
    /// 24h volume (string)
    pub q: String,
    /// 24h high (string)
    pub h: String,
    /// 24h low (string)
    pub l: String,
}

impl BinanceTickerData {
    /// Convert to typed CryptoTickerEntry.
    pub fn to_entry(&self) -> CryptoTickerEntry {
        CryptoTickerEntry {
            symbol: self.s.clone(),
            price: self.c.parse().ok(),
            change_24h: self.P.parse().ok(),
            volume: self.q.parse().ok(),
            high: self.h.parse().ok(),
            low: self.l.parse().ok(),
        }
    }
}

/// Default symbols for the combined stream.
pub const DEFAULT_SYMBOLS: &[&str] = &[
    "btcusdt", "ethusdt", "solusdt", "dogeusdt", "xrpusdt",
    "adausdt", "avaxusdt", "dotusdt", "linkusdt", "bnbusdt",
];

/// Build a Binance combined stream URL.
pub fn build_stream_url(symbols: &[&str]) -> String {
    let streams: Vec<String> = symbols
        .iter()
        .map(|s| format!("{}@ticker", s.to_lowercase()))
        .collect();
    format!("wss://stream.binance.com:9443/stream?streams={}", streams.join("/"))
}
