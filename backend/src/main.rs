#[macro_use] // Rocket-webframeworkin makrot käyttöön (esim. #[get], #[post] toimii tämän ansiosta)
extern crate rocket;

use rocket::serde::{ Deserialize, Serialize, json::Json }; // JSON-sarjennus ja -purku käyttöön
use rocket::{ State, response::status::Custom, http::Status }; // HTTP-statukset ja tilat
use tokio_postgres::{ Client, NoTls }; // PostgreSQL-yhteys Tokion async-ajurilla
use rocket_cors::{ CorsOptions, AllowedOrigins }; // CORS-sääntöjen määritys (frontendin pääsy backendille)

// ---------------------------
// Datan malli: käyttäjä
// ---------------------------

/// Struct, joka edustaa käyttäjää tietokannassa.
/// Käytetään myös HTTP-pyynnöissä JSON-muodossa.
/// - `id` on valinnainen, koska sitä ei tarvitse antaa kun luodaan uusi käyttäjä.
/// - `name` ja `email` ovat pakollisia kenttiä.

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// ---------------------------
// Käyttäjän lisääminen
// ---------------------------

/// POST /api/users
/// Lisää uuden käyttäjän tietokantaan.
/// Ottaa vastaan JSON-muotoisen käyttäjän tiedot ja lisää ne tietokantaan.
/// Palauttaa koko päivitetyn käyttäjälistan onnistuneen lisäyksen jälkeen.

#[post("/api/users", data = "<user>")]
async fn add_user(
    conn: &State<Client>, // Rocketin hallinnoima PostgreSQL-yhteys (shared state)
    user: Json<User> // JSON-rungossa saapuva käyttäjän tieto
) -> Result<Json<Vec<User>>, Custom<String>> {
    // SQL-kysely: lisätään nimi ja sähköposti käyttäjätauluun
    execute_query(
        conn,
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email]
    ).await?;
    // Palautetaan koko käyttäjälista lisäyksen jälkeen
    get_users(conn).await
}

// ---------------------------
// Käyttäjien hakeminen
// ---------------------------

/// GET /api/users
/// Palauttaa kaikki käyttäjät tietokannasta JSON-muodossa.
/// Suorittaa SELECT-kyselyn ja muuntaa tuloksen JSON-listaksi.

#[get("/api/users")]
async fn get_users(conn: &State<Client>) -> Result<Json<Vec<User>>, Custom<String>> {
    get_users_from_db(conn).await.map(Json) // JSON-muunnos automaattisesti
}

/// Suorittaa varsinaisen SELECT-kyselyn tietokantaan.
/// Palauttaa vektorin User-rakenteita, joka vastaa `users`-taulun sisältöä.
/// Virheen sattuessa palautetaan HTTP 500 (Internal Server Error).
/// Tietokantavirheet käsitellään Custom-tyypillä.
/// Parametrit:
/// - `client`: tietokantayhteys
/// - `query`: SQL-kysely (SELECT users)
/// - `params`: kyselyn parametrit (tässä tapauksessa tyhjät, koska haetaan kaikki käyttäjät)
/// Palauttaa vektorin käyttäjistä tai HTTP-virheen.
async fn get_users_from_db(client: &Client) -> Result<Vec<User>, Custom<String>> {
    let users = client
        .query("SELECT id, name, email FROM users", &[]).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))? // Tietokantavirheiden käsittely

        // Muunnetaan jokainen rivi User-rakenteeksi
        .iter()
        .map(|row| User { 
            id: Some(row.get(0)), // id on Option<i32>, koska se voi olla NULL
            name: row.get(1), // nimi (TEXT)
            email: row.get(2) }) // sähköposti (TEXT)
        .collect::<Vec<User>>(); // Kerätään kaikki käyttäjät vektoriin

    Ok(users) // Palautetaan käyttäjät vektorina
}

// ---------------------------
// Käyttäjän muokkaaminen
// ---------------------------

/// PUT /api/users/<id>
/// Päivittää käyttäjän tiedot annetun ID:n perusteella.
/// Ottaa vastaan uuden nimen ja sähköpostin, korvaa vanhat tiedot.
/// Palauttaa päivitetyn käyttäjälistan onnistumisen jälkeen.

#[put("/api/users/<id>", data = "<user>")]
async fn update_user(
    conn: &State<Client>,
    id: i32,    // Muokattavan käyttäjän ID URL-polusta
    user: Json<User> // Uudet käyttäjätiedot JSON-muodossa
) -> Result<Json<Vec<User>>, Custom<String>> {
    execute_query(
        conn,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        &[&user.name, &user.email, &id]
    ).await?;
    get_users(conn).await // Palautetaan koko käyttäjälista päivityksen jälkeen
}

// ---------------------------
// Käyttäjän poistaminen
// ---------------------------

/// DELETE /api/users/<id>
/// Poistaa käyttäjän annetun ID:n perusteella tietokannasta.
/// Palauttaa HTTP 204 (No Content) onnistuneen poiston jälkeen.

#[delete("/api/users/<id>")]
async fn delete_user(conn: &State<Client>, id: i32) -> Result<Status, Custom<String>> {
    execute_query(conn, "DELETE FROM users WHERE id = $1", &[&id]).await?;
    Ok(Status::NoContent) // HTTP 204 = ei sisältöä, mutta onnistui
}

// ---------------------------
// Apufunktio tietokantakyselyille
// ---------------------------

/// Yleinen funktio SQL-kyselyiden suorittamiseen.
/// Parametrit:
/// - `client`: tietokantayhteys
/// - `query`: suoritettava SQL-kysely (esim. INSERT, UPDATE, DELETE)
/// - `params`: viitteet parametreihin kyselyssä ($1, $2, ...).
/// Palauttaa rivien lukumäärän tai HTTP-virheen jos kysely epäonnistuu.
/// Virheiden käsittely tapahtuu Custom-tyypillä.
/// Tietokantavirheet käsitellään Custom-tyypillä.
/// Parametrit:
/// - `client`: tietokantayhteys
/// - `query`: SQL-kysely (INSERT, UPDATE, DELETE)
/// - `params`: kyselyn parametrit (esim. &[&user.name, &user.email])
/// Palauttaa rivien lukumäärän tai HTTP-virheen.
async fn execute_query(
    client: &Client,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)]
) -> Result<u64, Custom<String>> {
    client
        .execute(query, params).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}


// ---------------------------
// Sovelluksen käynnistys
// ---------------------------

/// Rocketin `#[launch]`-makrolla merkitty käynnistysfunktio.
/// Luo tietokantayhteyden, alustaa taulun jos ei ole olemassa,
/// määrittelee CORS-säännöt ja käynnistää HTTP-palvelimen.

#[launch]
async fn rocket() -> _ {
    // Yhdistetään PostgreSQL-tietokantaan (paikallinen)
    let (client, connection) = tokio_postgres
        ::connect("host=localhost user=postgres password=Sauna321 dbname=postgres", NoTls).await
        .expect("Failed to connect to Postgres");
    // Käynnistetään tietokantayhteyden ylläpito erillisessä säikeessä
    tokio::spawn(async move { 
        if let Err(e) = connection.await {
            eprintln!("Failed to connect to Postgres: {}", e);
        }
    });

    // Luodaan 'users'-taulu, jos se ei vielä ole olemassa
    let result = client
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL
            )",
            &[]
        ).await;
        // Tulostetaan konsoliin tiedot taulun luomisesta
    match result {
        Ok(_) => println!("Table created successfully"),
        Err(e) => eprintln!("Failed to create table: {}", e),
    }
    // Määritellään CORS-säännöt: sallitaan kaikki lähteet (kehityksessä helppoa)
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error while building CORS");

    // Rakennetaan Rocket-sovellus:
    // - hallinnoidaan tietokantayhteyttä
    // - liitetään reitit (REST API)
    // - otetaan käyttöön CORS
    rocket
        ::build()
        .manage(client)
        .mount("/", routes![add_user, get_users, update_user, delete_user])
        .attach(cors)
}
