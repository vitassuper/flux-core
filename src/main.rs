mod config;
mod database;

use async_tungstenite::async_std::connect_async;
use futures::stream::StreamExt;
use serde::Deserialize;
use serde_json::Value;

use crate::database::establish_connection;
use futures::prelude::*;

#[derive(Deserialize, Debug)]
struct Kline {
    start_time: u64,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
    close_time: u64,
}

async fn kline_socket(symbol: &str) -> Result<(), async_tungstenite::tungstenite::Error> {
    let uri = format!(
        "wss://fstream.binance.com/ws/{}@kline_1m",
        symbol.to_lowercase()
    );

    let (ws_stream, _) = connect_async(uri).await?;

    let (write, read) = ws_stream.split();

    let handle_messages = read.for_each(|message| async {
        if let Ok(message) = message {
            if let Ok(kline_data) = serde_json::from_str::<Value>(&message.to_string()) {
                if let Some(kline) = kline_data.get("k") {
                    if let (Some(t), Some(o), Some(h), Some(l), Some(c), Some(v)) = (
                        kline.get("t"),
                        kline.get("o"),
                        kline.get("h"),
                        kline.get("l"),
                        kline.get("c"),
                        kline.get("v"),
                    ) {
                        println!(
                            "Symbol: {}, Time: {}, Open: {}, High: {}, Low: {}, Close: {}, Volume: {}",
                            symbol,
                            t,
                            o,
                            h,
                            l,
                            c,
                            v
                        );
                    }
                }
            }
        }
    });

    handle_messages.await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let db_config = config::DbConfig::from_env()
        .expect("Failed to retrieve database configurations from environment variables");

    let connection = &mut establish_connection(db_config);

    // let symbol = "BTCUSDT";
    // if let Err(err) = kline_socket(symbol).await {
    //     eprintln!("Error: {:?}", err);
    // }
}
