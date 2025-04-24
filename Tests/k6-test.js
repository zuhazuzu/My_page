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
