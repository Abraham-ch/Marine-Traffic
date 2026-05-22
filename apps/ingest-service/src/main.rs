use std::env;

use dotenv::dotenv;

use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

use database::{create_vessel, establish_connection, models::Vessel};

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct VesselMessage {
  #[serde(rename = "MMSI")]
  mmsi: i64,
  ship_name: String,
  #[serde(rename = "latitude")]
  lat: f64,
  #[serde(rename = "longitude")]
  lng: f64,
  #[serde(rename = "Sog")]
  speed: Option<f32>,
  #[serde(rename = "TrueHeading")]
  heading: Option<f32>,
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
  
  let connection = &mut establish_connection();
  if let Some (message) = read.next().await {
    let message = message.expect("Failed to read message");
    println!("Creating the vessel for message: {}", message);
    let message: VesselMessage = serde_json::from_str(&message.to_string()).expect("Failed to parse message");
    let vessel = create_vessel(connection, &Vessel {
      mmsi: message.mmsi,
      ship_name: message.ship_name,
      lat: message.lat,
      lng: message.lng,
      speed: message.speed,
      heading: message.heading,
      updated_at: chrono::Utc::now(),
    });
    println!("Vessel created: {:?}", vessel);
  };

  while let Some(message) = read.next().await {
    match message {
        Ok(msg) => println!("Received message: {}", msg),
        Err(e) => eprintln!("Error receiving message: {}", e),
    }
  }
}
