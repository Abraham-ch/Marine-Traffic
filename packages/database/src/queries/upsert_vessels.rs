use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper};
use crate::models::Vessel;

pub fn upsert_vessel(conn: &mut PgConnection, vessel: &Vessel) -> Vessel {
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

  let changes = (
    vessels::ship_name.eq(&new_vessel.ship_name),
    vessels::lat.eq(new_vessel.lat),
    vessels::lng.eq(new_vessel.lng),
    vessels::speed.eq(new_vessel.speed),
    vessels::heading.eq(new_vessel.heading),
    vessels::updated_at.eq(new_vessel.updated_at),
  );

  diesel::insert_into(vessels::table)
    .values(&new_vessel)
    .on_conflict(vessels::mmsi)
    .do_update()
    .set(changes)
    .returning(Vessel::as_returning())
    .get_result(conn)
    .expect("Error upserting vessel")
}
