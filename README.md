# AIS-Stream

Purpose of the repository is connecting to a WebSocket from AIS Stream API with the ``tokio-tungstenite`` crate, this is supposed to be one of many steps to make a public app to keep traceability in the marine traffic such as flight-radar but in the sea.

This is the expected output

```json
{
  "Message": {
    "PositionReport": {
      "Cog": 0,
      "CommunicationState": 34552,
      "Latitude": 52.516600000000004,
      "Longitude": 5.639478333333333,
      "MessageID": 1,
      "NavigationalStatus": 0,
      "PositionAccuracy": true,
      "Raim": true,
      "RateOfTurn": -128,
      "RepeatIndicator": 0,
      "Sog": 0,
      "Spare": 0,
      "SpecialManoeuvreIndicator": 0,
      "Timestamp": 47,
      "TrueHeading": 511,
      "UserID": 244040401,
      "Valid": true
    }
  },
  "MessageType": "PositionReport",
  "MetaData": {
    "MMSI": 244040401,
    "MMSI_String": 244040401,
    "ShipName": "BARELLE.",
    "latitude": 52.5166,
    "longitude": 5.63948,
    "time_utc": "2026-05-18 21:35:47.638287285 +0000 UTC"
  }
}
```

You can also try to filter by ship, just fill the ``filters_ship_mmsi: vec![]`` with the ``MMSI_String`` assigned to the ship
