### POLKU TESTEIHIN. LOPULLISET TESTIT SEURAAVASSA OTSIKOSSA.

Mulla oli jo valmiina k6 riippuvuudet. Lisäksi Rust mainostaa omaa nopeuttaan niin halusin tehdä kuormitustestejä ohjelmalle.

Aluksi loin k6 testin jossa on GET POST PUT ja DELETE pyynnöt:
```javascript
import http from 'k6/http';
import { sleep } from 'k6';
export let options = {
  vus: 1, // virtuaalisten käyttäjien määrä
  duration: '30s', // testin kesto
};
function getRandomInt(max) {
  return Math.floor(Math.random() * max);
}
function getRandomUser() {
  const id = getRandomInt(1000);
  const name = `User${id}`;
  const email = `user${id}@example.com`;
  return { id, name, email };
}
export default function () {
  // GET-pyyntö
  http.get('http://127.0.0.1:8080');
  
  // POST-pyyntö
  let postPayload = JSON.stringify({ name: 'John Doe', email: 'john.doe@example.com' });
  let postParams = { headers: { 'Content-Type': 'application/json' } };
  http.post('http://127.0.0.1:8080/api/users', postPayload, postParams);
  
  // PUT-pyyntö satunnaisilla arvoilla
  let putUser = getRandomUser();
  let putPayload = JSON.stringify({ name: putUser.name, email: putUser.email });
  let putParams = { headers: { 'Content-Type': 'application/json' } };
  http.put(`http://127.0.0.1:8080/api/users/${putUser.id}`, putPayload, putParams);
  
  // DELETE-pyyntö
  http.del('http://127.0.0.1:8080/api/users/1');
  sleep(1);
}
```

Tämän testin tein käyttäjämäärillä 200, 500 ja 100.
Huomasin pian ongelman. kaikissa testeissä 75% epäonnistuu. oletettavasti kaikki GET pyynnöt onnistuu ja kaikki muut epäonnistuu. ID:t on ihan mitä sattuu.
Tämähän on ihan paska testi. Koitetaan tehdä parempi.

Uusi testi ja otetaan vähän maltillisemmin tuo lähestyminen. Vain get pyyntöjä ja lisätään virheenhallintaa jotta nähdään vähän missä homma kusee. vähennetään käyttäjiä.

```javascript
import http from 'k6/http'; // Tuodaan k6:n HTTP-moduuli
import { check, sleep } from 'k6'; // Tuodaan k6:n check- ja sleep-funktiot
// Määritellään testin asetukset
export const options = {
    vus: 10, // Samanaikaisten virtuaalikäyttäjien määrä
    duration: '30s', // Testin kesto
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% pyynnöistä alle 500ms
        http_req_failed: ['rate<0.01'] // Alle 1% pyynnöistä epäonnistuu
    }
};
// Testin pääfunktio, joka suoritetaan jokaiselle virtuaalikäyttäjälle
export default function () {
    // Lähetetään HTTP GET -pyyntö paikalliselle palvelimelle
    const response = http.get('http://127.0.0.1:8080/api/users');
    // Tarkistetaan, että vastaus on onnistunut (status 200)
    check(response, {
        'status is 200': (r) => r.status === 200,
        'response time is acceptable': (r) => r.timings.duration < 500 // Vastausaika alle 500ms
    });
    // Tulostetaan virheviesti, jos pyyntö epäonnistui
    if (response.status !== 200) {
        console.error(`Request failed with status ${response.status}`);
    }
    // Odotetaan 1 sekunti ennen seuraavaa pyyntöä

    sleep(1);

}
```

Jaaah. miksi GET pyynnöt onnistuu mutta POST ei?? no osoite tulee olla 8000 eikä 8080.... tässäkin meni melkein kaksi tuntia selvittää...
### LOPULLINEN TESTI:

Seuraava testi suoritti toiminnallisuudet POST GET PUT ja DELETE kyseisessä järjestyksessä.

```javascript
import http from 'k6/http';
import { check, sleep } from 'k6';
export const options = {
    vus: 1200, // Samanaikaisia virtuaalikäyttäjiä. TÄMÄ MÄÄRÄ MUUTTUU.
    duration: '30s', // Testin kesto
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% pyynnöistä alle 500ms
        http_req_failed: ['rate<0.01'], // Alle 1% pyynnöistä epäonnistuu
    },
};
export default function () {
    const baseUrl = 'http://127.0.0.1:8000/api/users';
    const name = `Test User ${__VU}-${__ITER}`; // __VU ja __ITER ovat k6:n sisäänrakennettuja muuttujia, jotka auttavat erottamaan käyttäjät toisistaan
    const email = `test${__VU}${__ITER}@example.com`; // Sähköposti on uniikki jokaiselle käyttäjälle, jotta vältetään päällekkäisyydet
    const payload = JSON.stringify({ name, email }); // Muodostetaan JSON-muotoinen data, joka lähetetään POST-pyynnössä
    const params = {
        headers: {
            'Content-Type': 'application/json', // Määritellään, että lähetämme JSON-dataa
        },
    };
    
    // 1. POST
    const postRes = http.post(baseUrl, payload, params);
    check(postRes, { 'POST status is 200': (r) => r.status === 200 }); // Tarkistetaan, että POST-pyyntö onnistui
    if (postRes.status !== 200) { // Jos POST epäonnistuu, tulostetaan virheviesti
        console.error(`POST failed: ${postRes.status} - ${postRes.body}`);
        return;
    }
    
    // 2. GET – etsitään juuri lisätty käyttäjä sähköpostin perusteella
    const getRes = http.get(baseUrl); // Haetaan kaikki käyttäjät GET-pyynnöllä
    check(getRes, { 'GET status is 200': (r) => r.status === 200 }); // Tarkistetaan, että GET-pyyntö onnistui
    const users = getRes.json(); // Muutetaan vastaus JSON-muotoon
    const latestUser = users.find((u) => u.email === email); // ← tärkein muutos. tässä etsitään juuri lisätty käyttäjä sähköpostin perusteella
    if (!latestUser || !latestUser.id) { // Jos käyttäjää ei löydy, tulostetaan virheviesti
        console.error(`User with email ${email} not found after POST`);
        return;
    }
    const userId = latestUser.id; // Tallennetaan käyttäjän ID muuttujaan, jotta voimme käyttää sitä myöhemmin
    
    // 3. PUT – päivitetään vain oma käyttäjä
    const updatePayload = JSON.stringify({
        name: `Updated ${name}`, // Päivitetään käyttäjän nimi
        email: `updated_${email}`, // Päivitetään käyttäjän sähköposti
    });
    const putRes = http.put(`${baseUrl}/${userId}`, updatePayload, params);
    check(putRes, { 'PUT status is 200': (r) => r.status === 200 }); // Tarkistetaan, että PUT-pyyntö onnistui
    if (putRes.status !== 200) { // Jos PUT epäonnistuu, tulostetaan virheviesti
        console.error(`PUT failed: ${putRes.status} - ${putRes.body}`);
    }
    
    // 4. DELETE – poistetaan oma käyttäjä
    const delRes = http.del(`${baseUrl}/${userId}`); // Poistetaan käyttäjä DELETE-pyynnöllä
    check(delRes, { 'DELETE status is 204': (r) => r.status === 204 });
    if (delRes.status !== 204) { // Jos DELETE epäonnistuu, tulostetaan virheviesti
        console.error(`DELETE failed: ${delRes.status} - ${delRes.body}`);
    }
    sleep(1);
}
```

Seuraavassa testissä muokataan käyttäjien määriä ``vus: ___`` ja testin kestoa ``duration: __``
Jos pyynnöistä 95% on ajassa <500ms on testi hyväksytty.
Jos pyynnöistä alle 1% epäonnistuu on testi hyväksytty.

Tein testin muokkaamalla yllä mainittuja arvoja:
- VU(käyttäjämäärä) muutokset Duration(sekunteina) muutokset:
	1. VU 50 D30
	2. VU 100 D30
	3. VU 500 D30
	4. VU 700 D30
	5. VU 1000 D30
	6. VU 1200 D30
	7. VU 100 D150
Ylläolevista testeistä löytyy tallennettuna data  tiedostoon 
Tests/k6-test_VU(määrä)D(aika).md

Testeistä voi tehdä seuraavia johtopäätöksiä:
1. Ohjelma luo aina uuden ID arvon jokaisessa POST pyynnössä joten ID arvo kasvaa riippumatta paljonko käyttäjiä on oikeasti tietokannassa. 
2. 500 samanaikaista käyttäjää ylitti hyväksyttävien pyyntöjen ajan. *k6-test_VU500D30.md* rivi 27.
```
✗ http_req_duration: avg=939.42ms min=515.9µs med=956.19ms max=2.48s   p(90)=1.57s  p(95)=1.78s
```
3. 1200 samanaikaista käyttäjää aiheuttaa toimimattomuutta ohjelmassa laitteella jolla testit on tehty. Testeissä laitteen muistin toiminta nousee ~97% joka mahdollisesti on pullonkaula.
   Toinen testi joka suoritettiin samoilla ehdoilla onnistui siten, että kaikki pyynnöt onnistui, mutta pyyntöjen pituus ylitti hyväksyttävän ajan.
   
   Muisti 92%, Prosessori75%!![\[\[Pasted image 20250424134701.png\]\]](Prossu.png)
   
5. Testin keston noustaessa 150sekuntiin luotujen iteraatioiden määrä nousi yli 10000 joka oli korkeampi kuin suositeltu määrä k6 testeille. *Tests\k6-test_VU100D150.md* rivi 17.
```
   WARN[0115] The test has generated metrics with 200035 unique time series, which is higher than the suggested limit of 100000 and could cause high memory usage. Consider not using high-cardinality values like unique IDs as metric tags or, if you need them in the URL, use the name metric tag or URL grouping. See https://grafana.com/docs/k6/latest/using-k6/tags-and-groups/ for details.  component=metrics-engine-ingester
``` 
Testit ja tulokset löytyy sijainnista [Tests](Tests)
