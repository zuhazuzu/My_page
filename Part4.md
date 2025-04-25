# Käyttötapaukset.

1. Tiedon lisääminen.
	1. Tämä onnistuu. Ongelmana joka liittyy tiedon lisäämiseen on se, että uusi tieto lisätään aina uutena ID:nä vaikka aiemmat ID:t olisi poistettu. Tämä tarkoittaa että tiedon lisääntyessä luodaan vain suurempia ja suurempia ID:tä. Tämä ongelma korostuu kuormitustestauksessa, jossa lisätään suuria määriä dataa tietokantaan kerralla.
	2. Tiedon lisääminen aiheuttaa vastauksen sivstolta käyttäjälle onnistuneesta lisäyksestä.
	3. Parannusehdotuksena olisi luoda testit joka tarkastaa onnistuneen ja epäonnistuneen tilan vastauksen.

2. Tiedon tarkastelu.
	1. Tämä toteutuu projektissa. Tähän liittyy puolestaan sellainen parannusehdotus, että nykymuodossa tietojen tarkastelu toteutuu tietoa lisätessä, muokatessa, poistaessa taikka tietoa erikseen pyytämällä. Parannuksena olisi näyttää tieto käyttäjälle välittömästi hänen saapuessa sivustolle eikä, että hänen täytyy vuorovaikuttaa sivuston kanssa saadakseen tiedon.
	2. Toinen parannusehdotus olisi lisätä käyttäjälle responsiivinen vastaus, jos tietokanta on tyhjä. Tämä ei nykymuodossa toteudu.

3. Tiedon muokkaaminen.
	1. Tietojen muokkaaminen onnistuu. Käyttäjä voi muokata tietoja tietokannasta haluamastaan sijainnista.
	2. Käyttäjä voi myös poistaa tietoa tarvittaessa haluamastaan tietokannan sijainnista.
	3. Yllämainituissa tapauksissa käyttäjä saa toimintoa vastaavan vastuksen.

# Projektin erityisominaisuudet. 

1. Rust ohjelmointikieli projektissa. 
	1. Projektin ohjelmointikielen valinta aiheutti haasteita, jonka vuoksi projektissa käytettävät arvioinnit sovelletaan lopulliseen arvosanaan. 
	2. Ohjelma toimii kokonaisuudessaan Rust- ohjelmointikielellä ja sen tukena toimii PostgreSQL tietokanta. 
	3. Rust-ohjelmointikieli mahdollistaa muistiturvallisen ohjelman joka on kevyt ja nopea käyttää. 
	4. Rust kääntäjä on erityisen tarkka tunnistamaan virheet ja niiden täsmällinen sijainti. Tämä nopeuttaa ohjelmistokehittämistä virheiden paikallistamisessa ja tunnistamisessa.
2. Rocket framework backendissä.
	1. Tämän avulla saadaan RESTful API- rajapintojen rakentaminen tehokkaasti. 
	2. Rocket framework tarjoaa syntaksin HTTP- pyyntöjen käsittelyyn ja JSON datan käsittelyyn.
	3. Asyncroniset toiminnot mahdollistaa rinnakkaisajon pyyntöjen avulla, jolloin tietojen käsittely on nopeaa ja ei estä muiden samanaikaisten pyyntöjen suorittamista.
	4. TOKIO-postgres- ajurin avulla yhdistetään Rocket ja PostgresSQL tietokanta keskenään. Tämä Tokio-postgres pystyttää itse tietokannan, jos se ei sellaista löydä.
3. Yew- framework frontendissä.
	1. React tyylinen frontend- kirjasto jonka avulla sovellus rakennetaan komponenteista.
	2. Tämän avulla sovellus rakennetaan WebAssemblyn avulla.
	3. HTML Rustin sisällä mahdollistaa kääntäjän tunnistaa HTML osion virheet jo käännösvaiheessa
4. GLOO -kirjastot
	1. Tämäna vulla REST API pyynnöt saadaan toimimaan Backendin kanssa.
5. Tailwind CSS
	1. Tällä saadaan mielestäni selkeyttä ulkoasun määrittämiseen siten, että CSS tyylittelyt on asetettu HTML elementtien sisään ja täten niiden assosiaatiot ovat selkeämpiä kuin erillisessä CSS tiedostossa.

# Kehitysprosessi.

Ohjelmaa luodessa olin ongelmissa useita kertoja ja aloitin kahdesti alusta. Kolmannella kerralla päätin valita tuekseni varmemman tietolähteen tulosten takaamiseksi. Tämä päätös vaikutti siihen millaista tietokantaa ohjelma tulee sisältämään. Vaikutus aiheutti suppeamman sisällön aiempaan nähden. 

Loin ohjelmalle oman tietokannan Docker konttiin, mutta en tähän raporttiin saanut selvitettyä, miksi ohjelma ei onnistunut yhdistämään kyseiseen docker konttiin. Nykyisessä muodossa TOKIO-postgre luo tietokannan jota käytän ohjelman kanssa. 

Kolmos vaiheessa halusin lisätä projektiin kuormitustestauksia. 
Testauksia olisi voinut luoda vaikka mitä, mutta Rust ohjelmointikielen erityispiirteisiin kuuluu sen nopeus, joten halusin korostaa kieleen liittyviä ominaisuuksia tekemällä kuormitustestejä.

# Jatkokehitys.

Jatkokehityksenä haluaisin muuttaa tietokannan rakennetta paremmaksi omiin tarkoituksiini esmierkiksi sarakkeet voisivat olla päiväkirjan pitämiseen sopivampia.

Projektin pitkäaikaisen käytettävyyden kannalta on välttämätöntä muuttaa uuden tiedon tuottamista siten, että uusi data asetetaan pienimpään käyttämättömään ID:seen. 

Lisäksi kattavammat testaukset olisivat tärkeitä ohjelman kestävyyden kannalta. 

Jos tälläinen projekti siirtyisi kuluttajien käytettäväksi, olisi autentikointi välttämätön.

# Reflektointi

Rust ohjelmointikielen käyttö aiheutti oppimista kyseisessä ohjelmointikielessä.

Lähde jota käytin apuna ohjelman luomisessa antoi uusia näkökulmia kuinka ohjelman voi toteuttaa ja erityisesti uutena asiana korostan, kuinka tietokanta rakennetaan ohjelman sisällä, eikä rakennetta tehdä valmiiksi tietokantaan. 

Haasteita oli alusta asti, mutta olin tietoinen ohjelmistokielen haastavuudesta. Siitä huolimatta odotin nopeampaa edistymistä projektin kanssa ja projektissa oli paljon haasteita.

Tässävaiheessa olen tyytyväinen, että valitsin itseäni kiinnostavan ohjelmointikielen tätä projektia varten. Olisin kuitenkin toivonut edistyväni sen kanssa nopeammin, jotta projektin rakenne olisi vastannut paremmin aiempaa näkemystäni sen sisällöstä.

# Työaika.

| Päivä          | Käytetty aika(h) |              Tehdyn työn kuvaus                      |
| :------------- | :--------------: | :--------------------------------------------------: |
| 18.3.2025      |        2         |              ideointi                                |
| 23.3.2025      |        3         |              Määrittely                              |
| 31.3.2025      |        3         |             Rust depencyjen asennus                  |
| 3.4.2025       |        2         |              postgreSQL asennus                      |
| 8.4.2025       |        4         |        tietokannan asennusta ja kontin tekoa         |
| 9.4.2025       |        3         |         Uudelleen aloitus ohjeen mukaisesti          |
| 10.4.2025      |        3         |        Rust kielen analysointia ja muistelua         |
| 11.4.2025      |        3         |               tietokannan "etsimistä"                |
| 11.4.2025      |        3         |                    Frontti läjään                    |
| 12.4.2025      |        2         |                     Code review                      |
| 13.4.2025      |        4         |        koodin kommentointi + part 2 raportti         |
| 16.4.2025      |        1         |                raportin hienosäätöä.                 |
| 23.4.2025      |        5         |                 k6 Testien tekemistä                 |
| 24.4.2025      |        3         | Testien viimeistely. Testien ajo. Raportin tekeminen |
| 25.4.2025      |        4         |             Raportin ja esityksen teko.              |
YHT 45h