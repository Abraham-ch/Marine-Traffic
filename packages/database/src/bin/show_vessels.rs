use self::models::*;
use diesel::prelude::*;
use database::*;

fn main() {
  use self::schema::vessels::dsl::*;

  let connection = &mut establish_connection();
  let results = vessels
    .limit(5)
    .select(Vessel::as_select())
    .load(connection)
    .expect("Error loading vessels");

  println!("Displaying {} vessels", results.len());
  for vessel in results {
    println!("{}", vessel.ship_name);
  }
}
