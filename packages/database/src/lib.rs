pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::Vessel;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_vessel(conn: &mut PgConnection, vessel: &Vessel) -> Vessel {
  use crate::schema::vessels;

  let new_vessel = Vessel { 
    mmsi: vessel.mmsi, 
    ship_name: vessel.ship_name.clone(), 
    lat: vessel.lat, 
    lng: vessel.lng, 
    speed: vessel.speed, 
    heading: vessel.heading, 
    updated_at: vessel.updated_at
  };  

  diesel::insert_into(vessels::table)
    .values(&new_vessel)
    .returning(Vessel::as_returning())
    .get_result(conn)
    .expect("Error saving new vessel")
}
