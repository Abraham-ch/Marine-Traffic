use diesel::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::vessels)]
pub struct Vessel {
    pub mmsi: i64,
    pub ship_name: String,
    pub lat: f64,
    pub lng: f64,
    pub speed: Option<f32>,
    pub heading: Option<f32>,
    pub updated_at: DateTime<Utc>,
}
