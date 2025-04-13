
Frontendin ajossa saapi aina ottaa F-securen virusturvan pois päältä jotta toi homma pelaa.
## 1. Enviroment.

Tämä projekti on vain paikallisesti koneella. 
## 2. Backend.

Backendissä käytetään Rust ohjelmointikieltä ja Rocket web rankaa.

#### **Rocket (Web Framework)**

- **Toiminta**: Rocket on Rust-pohjainen web-framework, joka mahdollistaa RESTful API -rajapintojen rakentamisen helposti ja tehokkaasti. Rocket tarjoaa selkeän syntaksin HTTP-pyyntöjen käsittelyyn, reittien määrittelyyn ja JSON-datan käsittelyyn.
- Rocket tukee asynkronista koodia `async`/`await`-syntaksin avulla. Tämä mahdollistaa tehokkaan rinnakkaisuuden, jolloin pyyntöjen käsittely ei estä toisten pyyntöjen suoritusta.
- Backendin koodissa `tokio-postgres`-ajuri yhdistää Rocketin PostgreSQL-tietokantaan. Asynkroninen ajuri mahdollistaa datan haun ja käsittelyn ilman, että se estäisi muiden pyynnöiden käsittelyä.

#### **Tokio-postgres**

- **Toiminta**: `tokio-postgres` on Rustin asynkroninen ajuri PostgreSQL-tietokannalle. Se käyttää `tokio`-ajokehystä, joka tarjoaa asynkronista rinnakkaisuutta ja tehokasta resurssien hallintaa.

## 3. Frontend.

#### **Yew (Frontend Framework)**

- **Toiminta**: Yew on Rust-pohjainen React-tyylinen frontend-kirjasto, joka mahdollistaa web-sovellusten rakentamisen WebAssemblyn avulla. Yew:n rakenne perustuu komponentteihin, jotka voivat sisältää tilaa (state) ja käsitellä tapahtumia.
- **Komponentit**: Yew-komponentit määritellään funktiopohjaisesti (esim. `#[function_component(App)]`), ja ne palauttavat `Html`-tyypin, joka vastaa renderöityä HTML-koodia.

#### **Gloo (WebAssembly ja HTTP-kutsut)**

- **Toiminta**: Gloo on kirjastopaketti, joka helpottaa WebAssembly-sovellusten tekemistä.
- **HTTP-pyynnöt**: Gloo:n `Request`-moduuli mahdollistaa REST API -pyyntöjen lähettämisen. Yew-komponentissa käytetään Gloo:n `Request::get`, `Request::post`, `Request::put` ja `Request::delete` -metodeja kommunikoimaan backendin kanssa.

#### **Tailwind CSS (Tyylittely)**

- **Toiminta**: Tailwind on utility-first CSS-kehys, jossa tyylit luodaan käyttämällä valmiita luokkia suoraan HTML-elementeissä. Tailwindia käytetään frontend-sovelluksessa luomaan responsiivisia käyttöliittymiä ilman erillisiä CSS-tiedostoja. Käytetään mm. luokkia kuten `bg-blue-500`, `text-white`, `py-2`, `px-4` jne.

## 4. Tietokanta.

PostgreSQL joka tässävaiheessa nousee pystyyn `tokio-postgres` avulla.

## 5. Rakenne.

projekti/
├── backend/                       # Rocket-palvelin & PostgreSQL-tietokanta
│   ├── Cargo.toml               # Projektin workspace-tiedosto
│   ├── src/
│        └── main.rs                  # Backend-sovelluksen pääkoodi
│
├── frontend/                 # Yew-frontend WebAssemblynä
│   ├── src/
│   │   └── main.rs          # Yew-komponenttilogiikka ja HTTP-kutsut
│   ├── Cargo.toml        # Projektin workspace-tiedosto
│   └── index.html         # HTML-juurisivu joka latautuu selaimeen
│
├── Part2.md                    # Tämä dokumentti
└── README.md              # Projektin "päiväkirja"

## 6. Toiminnot.

- `get_users`: hakee käyttäjät backendistä
- `create_user`: lähettää uuden käyttäjän tiedot
- `update_user`: muokkaa olemassa olevaa käyttäjää ID:n perusteella
- `delete_user`: poistaa käyttäjän
- `edit_user`: siirtää käyttäjän tiedot lomakkeelle muokattavaksi

## 7. Kommentointi ja dokumentointi.

Tekoälyä apuna käyttäen kommentoin koodia ja olen pitänyt päiväkirjaa tekemistä asioista. Lisäksi tämän dokumentin luonti. 

### 8. Testaus ja virheen käsittely.

Annettaessa API-kutsuja tulee konsoliin lukemaan millainen pyyntö ja miten se vastasi, Esim:
```powershell
POST /api/users application/json:
   >> Matched: (add_user) POST /api/users
   >> Outcome: Success(200 OK)
   >> Response succeeded.
GET /api/users:
   >> Matched: (get_users) GET /api/users
   >> Outcome: Success(200 OK)
   >> Response succeeded.
```

Toistaiseksi ohjelmalla ei ole siihen synkronoituja testejä. Käyttäjä on testannut ohjelmiston toiminnan manuaalisesti. 

Virheenkäsittelystä Esimerkkejä:
Backend/src/main.rs
Rivit 65-74
```Rust
/// Tietokantavirheet käsitellään Custom-tyypillä.
/// Parametrit:
/// - `client`: tietokantayhteys
/// - `query`: SQL-kysely (SELECT users)
/// - `params`: kyselyn parametrit (tässä tapauksessa tyhjät, koska haetaan kaikki käyttäjät)
/// Palauttaa vektorin käyttäjistä tai HTTP-virheen.
async fn get_users_from_db(client: &Client) -> Result<Vec<User>, Custom<String>> {
    let users = client
        .query("SELECT id, name, email FROM users", &[]).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
```
Rivit 135-149
```Rust
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
```
Rivit 173-186
```Rust
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
```
Rivit 188-192
```Rust
    // Määritellään CORS-säännöt: sallitaan kaikki lähteet (kehityksessä helppoa)
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error while building CORS");
```

Frontend/src/main.rs
Rivit 12-13
```Rust
	// Luodaan muuttuja joka tarvittaessa palauttaa tilan.
    // Tila mahdolliselle käyttäjäviestille (esim. virhe tai onnistuminen)
    let message = use_state(|| "".to_string());
```
*message* muuttujaa käytetään usealla rivilla koodissa josta näkee käyttäjän tilan, esim:
```Rust
            let message = message.clone();
```
Tämä löytyy usealta riviltä, jossa siitä voisi olla hyötyä. message tila asetetaan tarvittavaan muotoon. Esim rivit 22-33
```Rust
            let users = users.clone();
            let message = message.clone();
            spawn_local(async move {
                // HTTP GET-pyyntö käyttäjien hakemiseksi
                match Request::get("http://127.0.0.1:8000/api/users").send().await {
                    Ok(resp) if resp.ok() => {
                        // Parsitaan JSON vastaus käyttäjiksi
                        let fetched_users: Vec<User> = resp.json().await.unwrap_or_default();
                        users.set(fetched_users);
                    }
                    // Jos pyyntö epäonnistuu, asetetaan virheilmoitus
                    _ => message.set("Failed to fetch users".into()),
```
Rivillä 65:
```Rust
                    _ => message.set("Failed to create user".into()),
```
Lisää virheenkäsittelyä Riveillä: 92-98
```Rust
                    match response {
                        Ok(resp) if resp.ok() => {
                            message.set("User updated successfully".into());
                            get_users.emit(()); // Päivitetään käyttäjälista
                        }
                        _ => message.set("Failed to update user".into()),
```
Tälläiset virheenkäsittelyt toistuu ohjelmassa myöhemmin muutamaan otteeseen.

## 9. Käytettävyys on tiedettä?

Tässä käyttöliittymästä kuva:
![[UI.png]]

Jokainen nappi("Create User", "Fetch User List", "Delete", "Edit") muuttaa väriään hieman kun kursoria laittaa näppäimen ylle. Nappia painaessa sivu päivittyy välittömästi. Uuden käyttäjän lisätessä, se tulee heti näkymään userlistaan. Sivun ensimmäistä kertaa avatessa käyttäjälista ei ole näkyvissä, vasta kun lisää käyttäjän, taikka hakee käyttäjälistan se tulee näkyviin.

Tähän loppuun kopioin hieman päiväkirjaa, tämä sama löytyy README.md tiedostosta.
- Kontin asenuksen kanssa ei mennyt ihan kuin tanssi. mutta luulen että se on ihan "ok" nytte.

- 8.4.2025 klo14.06
- Rustia aiemmin(3.4.2025) touhasin ja nyt se on ihan solmussa, koitan oikaista -8.4.2025 klo14.38
    - Tässä luovuin projektin nykyisestä rakenteesta, poistin sen ja päätin aloittaa alusta. ongelmat liittyi.

- backend paskaa(Jos luet tätä, olen unohtanut kirjoittaa siitä)

- 11.4.2025: Ongelma: Dockerkontin pystyttäminen onnistuu ja backendin pystyttäminen onnistuu, mutta backend ei löydä Dockerissa pyörivää tietokantaa. Backend kuitenkin ilmoittaa konsolissa "Table created successfully" ja pystyn tekemään erilaisia CRUD toimintoja, johonkin sen tiedon on mentävä. Selvittäessä asiaa ilmenee, että Docker kontti ei onnistuneesta ilmoituksesta huolimatta ole pystyssä. Mihin tieto sitten menee?: Ohjelmassa käytettävä tokio_postgres importilla on sellainen ominaisuus, että jos tämä ei löydä tietokantaa johon sen pitäisi yhdistää, niin se sellaisen luo. ELI: tietokanta on olemassa ja annetut tiedot menee jonnekkin. MUTTA, en ole selvittänyt miksi tokio_postgres toimii portissa 5432 ja Docker-tietokanta nousee samaan porttiin? spekulaatio on että tuo Docker kontti ei oikeasti nouse mihinkään.

- 11.4.2025: Tässä vaiheessa unohdetaan edellämainittu ongelma ja ryhdytään toimiin tiedon käsittelystä sivustolla. Käytetään Frontedissä apuna YEW-frameworkkia, jotta voidaan luoda frontend Rustilla(Rustin versio REACT:ista, käyttää WebAssemblyä apunaan) asennetaan riippuvuudet. Lisäksi Käytetään TRUNK rakennustyökalua joka auttaa pystyttämään YEW-framen(Kääntää Rustin WebAssemblyksi ja yhdistää assetit). Ohjelmassa käytetään myös *tailWind CSS* frameworkkia jotta saadaan ulkonäköön liittyvät toiminnot pelaamaan(Lisätään CSS suoraan HTML-koodiin tai komponentteihin).
