use std::env;

use dotenv::dotenv;

use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

use database::{establish_connection, models::Vessel};

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

#[derive(Deserialize)]
struct RootMessage {
  #[serde(rename = "Message")]
  message: MessageData,
  #[serde(rename = "MetaData")]
  metadata: MetaData,
}

#[derive(Deserialize)]
struct MessageData {
  #[serde(rename = "PositionReport")]
  position_report: PositionReport,
}

#[derive(Deserialize)]
struct PositionReport {
  #[serde(rename = "Sog")]
  speed: Option<f32>,
  #[serde(rename = "TrueHeading")]
  heading: Option<f32>,
}

#[derive(Deserialize)]
struct MetaData {
  #[serde(rename = "MMSI")]
  mmsi: i64,
  #[serde(rename = "ShipName")]
  ship_name: String,
  #[serde(rename = "latitude")]
  lat: f64,
  #[serde(rename = "longitude")]
  lng: f64,
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

  // This example is for demonstration purposes and only processes the first message received. In a real application, you would likely want to continuously read messages in a loop.
  // if let Some (message) = read.next().await {
  //   let message = message.expect("Failed to read message");
  //   println!("Creating the vessel for message: {}", message);
  //   let message: RootMessage = serde_json::from_str(&message.to_string()).expect("Failed to parse message");
  //   let vessel = create_vessel(connection, &Vessel {
  //     mmsi: message.metadata.mmsi,
  //     ship_name: message.metadata.ship_name,
  //     lat: message.metadata.lat,
  //     lng: message.metadata.lng,
  //     speed: message.message.position_report.speed,
  //     heading: message.message.position_report.heading,
  //     updated_at: chrono::Utc::now(),
  //   });
  //   println!("Vessel created: {:?}", vessel);
  // };

  while let Some(message) = read.next().await {
    let msg = message.unwrap();
    let message: RootMessage = match serde_json::from_str::<RootMessage>(&msg.to_string()){
      Ok(msg) => msg,
      Err(err) => {
        eprintln!("Invalid JSON: {}", err);
        continue;
      }
    };

    let upsert_vessels = database::queries::upsert_vessels::upsert_vessel(connection, &Vessel {
      mmsi      :message.metadata.mmsi,
      ship_name :message.metadata.ship_name,
      lat       :message.metadata.lat,
      lng       :message.metadata.lng,
      speed     :message.message.position_report.speed,
      heading   :message.message.position_report.heading,
      updated_at:chrono::Utc::now(),
    });
    println!("Vessel upserted: {:?}", upsert_vessels);
  }
}
