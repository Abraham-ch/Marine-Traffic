use std::env;

use dotenv::dotenv;

use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct SubscriptionMessage {
  #[serde(rename = "APIKey")]
  api_key: String,
  bounding_boxes: Vec<[[f64; 2]; 2]>,
  #[serde(rename = "FIltersShipMMSI")]
  filters_ship_mmsi: Vec<String>,
  filter_message_types: Vec<String>,
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let url = match env::var("AIS_STREAM_API_URL") {
    Ok(val) => val,
    Err(_) => {
      eprintln!("Error: AIS_STREAM_API_URL environment variable not set");
      return;
    }
  };

  let api_key = match env::var("AIS_STREAM_API_KEY") {
    Ok(val) => val,
    Err(_) => {
      eprintln!("Error: AIS_STREAM_API_KEY environment variable not set");
      return;
    }
  };

  let subscription_message = SubscriptionMessage {
    api_key: api_key.clone(),
    bounding_boxes: vec![[[-90.0, -180.0], [90.0, 180.0]]],
    filters_ship_mmsi: vec![],
    filter_message_types: vec!["PositionReport".to_string()],
  };

  let serialized_message = serde_json::to_string(&subscription_message).unwrap();
  
  println!("Connecting to AIS Stream API at: {}", url);
  
  let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to WebSocket");
  println!("WebSocket connection established");

  let (mut write, mut read) = ws_stream.split();

  let msg: Message = Message::Text(serialized_message.into());

  println!("Sending message: {}", msg);
  write.send(msg).await.expect("Failed to send message");
  
  if let Some(message) = read.next().await {
    let message = message.expect("Failed to read message");
    println!("Received message: {}", message);
  }  
}
