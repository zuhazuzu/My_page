#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;

use rocket::serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::fairing::AdHoc;

#[database("tehtavat_db")]
struct DbConn(diesel::PgConnection);

#[derive(Serialize, Deserialize)]
struct TehtavaInput {
    paivamaara: String,
    tyoaika: f64,  // Muutetaan tyoaika Float8-tyypiksi
    lyhyt_kuvaus: String,
    tarkempi_kuvaus: String,
}

#[post("/tehtavat", data = "<tehtava>")]
async fn luo_tehtava(conn: DbConn, tehtava: rocket::serde::json::Json<TehtavaInput>) -> String {
    use crate::schema::tehtavat::dsl::*;
    use crate::models::Tehtava;

    let uusi_tehtava = Tehtava {
        id: 0,  // Diesel asettaa automaattisesti arvon
        paivamaara: chrono::NaiveDate::parse_from_str(&tehtava.paivamaara, "%Y-%m-%d").unwrap(),
        tyoaika: tehtava.tyoaika,
        lyhyt_kuvaus: tehtava.lyhyt_kuvaus.clone(),
        tarkempi_kuvaus: tehtava.tarkempi_kuvaus.clone(),
    };

    conn.run(move |c| {
        diesel::insert_into(tehtavat)
            .values(&uusi_tehtava)
            .execute(c)
    }).await.unwrap();

    "Tehtävä luotu".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![luo_tehtava])
}
