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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_raw(s: &str, c: &str, p: &str, q: &str, h: &str, l: &str) -> BinanceTickerData {
        BinanceTickerData {
            s: s.into(),
            c: c.into(),
            P: p.into(),
            q: q.into(),
            h: h.into(),
            l: l.into(),
        }
    }

    #[test]
    fn to_entry_valid_data() {
        let raw = make_raw("BTCUSDT", "42000.50", "2.5", "1000000000", "43000", "41000");
        let entry = raw.to_entry();
        assert_eq!(entry.symbol, "BTCUSDT");
        assert_eq!(entry.price, Some(42000.50));
        assert_eq!(entry.change_24h, Some(2.5));
        assert_eq!(entry.volume, Some(1000000000.0));
        assert_eq!(entry.high, Some(43000.0));
        assert_eq!(entry.low, Some(41000.0));
    }

    #[test]
    fn to_entry_invalid_number() {
        let raw = make_raw("BTCUSDT", "not-a-number", "bad", "0", "0", "0");
        let entry = raw.to_entry();
        assert_eq!(entry.symbol, "BTCUSDT");
        assert_eq!(entry.price, None);
        assert_eq!(entry.change_24h, None);
        assert_eq!(entry.volume, Some(0.0));
    }

    #[test]
    fn to_entry_negative_price() {
        let raw = make_raw("TESTUSDT", "-100.5", "-3.2", "500", "0", "-200");
        let entry = raw.to_entry();
        assert_eq!(entry.price, Some(-100.5));
        assert_eq!(entry.change_24h, Some(-3.2));
        assert_eq!(entry.low, Some(-200.0));
    }

    #[test]
    fn build_stream_url_single() {
        let url = build_stream_url(&["btcusdt"]);
        assert_eq!(
            url,
            "wss://stream.binance.com:9443/stream?streams=btcusdt@ticker"
        );
    }

    #[test]
    fn build_stream_url_multiple() {
        let url = build_stream_url(&["btcusdt", "ethusdt"]);
        assert_eq!(
            url,
            "wss://stream.binance.com:9443/stream?streams=btcusdt@ticker/ethusdt@ticker"
        );
    }

    #[test]
    fn build_stream_url_empty() {
        let url = build_stream_url(&[]);
        assert_eq!(
            url,
            "wss://stream.binance.com:9443/stream?streams="
        );
    }

    #[test]
    fn build_stream_url_uppercase_lowercased() {
        let url = build_stream_url(&["BTCUSDT"]);
        assert!(url.contains("btcusdt@ticker"));
    }

    #[test]
    fn default_symbols_count() {
        assert_eq!(DEFAULT_SYMBOLS.len(), 10);
        assert!(DEFAULT_SYMBOLS.contains(&"btcusdt"));
        assert!(DEFAULT_SYMBOLS.contains(&"ethusdt"));
    }

    #[test]
    fn deserialize_stream_message() {
        let json = r#"{
            "stream": "btcusdt@ticker",
            "data": {
                "s": "BTCUSDT",
                "c": "42000.00",
                "P": "1.5",
                "q": "500000000",
                "h": "42500",
                "l": "41500"
            }
        }"#;
        let msg: BinanceStreamMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.stream, "btcusdt@ticker");
        assert_eq!(msg.data.s, "BTCUSDT");
        assert_eq!(msg.data.c, "42000.00");
    }

    #[test]
    fn deserialize_ticker_entry() {
        let json = r#"{
            "symbol": "ETHUSDT",
            "price": 2500.0,
            "priceChangePercent": -0.5,
            "volume": 1000000,
            "high": 2600,
            "low": 2400
        }"#;
        let entry: CryptoTickerEntry = serde_json::from_str(json).unwrap();
        assert_eq!(entry.symbol, "ETHUSDT");
        assert_eq!(entry.price, Some(2500.0));
    }

    #[test]
    fn default_ticker_entry() {
        let entry = CryptoTickerEntry::default();
        assert!(entry.symbol.is_empty());
        assert!(entry.price.is_none());
    }
}
