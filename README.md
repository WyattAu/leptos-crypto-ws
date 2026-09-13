# leptos-crypto-ws

Real-time Binance WebSocket client for [Leptos](https://leptos.dev/) — provides live crypto prices as reactive signals.

## Features

- **Binance combined stream** — subscribes to multiple ticker streams
- **Automatic reconnection** — reconnects on disconnect with backoff
- **Typed data** — structured `CryptoTickerEntry` with price, change, volume
- **Reactive signals** — updates a `Signal<HashMap>` via Leptos context
- **Configurable** — custom symbols, default to top 10 by volume

## Installation

```toml
[dependencies]
leptos-crypto-ws = "0.1"
```

## Usage

```rust
use leptos::prelude::*;
use leptos_crypto_ws::{CryptoWsProvider, use_crypto_ws};

#[component]
fn App() -> impl IntoView {
    view! {
        <CryptoWsProvider>
            <TickerBar/>
        </CryptoWsProvider>
    }
}

#[component]
fn TickerBar() -> impl IntoView {
    let crypto = use_crypto_ws();
    view! {
        <div class="ticker">
            {move || {
                let data = crypto.get();
                data.get("BTCUSDT")
                    .and_then(|t| t.price)
                    .map(|p| format!("${p:.2}"))
                    .unwrap_or_else(|| "--".to_string())
            }}
        </div>
    }
}
```

## Custom Symbols

```rust
view! {
    <CryptoWsProvider symbols=vec!["btcusdt".into(), "ethusdt".into()]>
        <App/>
    </CryptoWsProvider>
}
```

## API

### `CryptoWsProvider`

Leptos component that manages the WebSocket connection.

- `symbols`: `Option<Vec<String>>` — Binance symbols (default: top 10)
- `children`: `Children` — child components

### `use_crypto_ws() -> Signal<HashMap<String, CryptoTickerEntry>>`

Returns the current crypto prices as a reactive signal.

### `CryptoTickerEntry`

```rust
pub struct CryptoTickerEntry {
    pub symbol: String,        // "BTCUSDT"
    pub price: Option<f64>,    // 42500.00
    pub change_24h: Option<f64>, // 2.35
    pub volume: Option<f64>,   // 28000000000.0
    pub high: Option<f64>,     // 43000.00
    pub low: Option<f64>,      // 41500.00
}
```

## License

MIT
