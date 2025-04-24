import http from "k6/http"; // Tuodaan k6:n HTTP-moduuli
import { check, sleep } from "k6"; // Tuodaan k6:n check- ja sleep-funktiot
// Määritellään testin asetukset
export const options = {
  vus: 10, // Samanaikaisten virtuaalikäyttäjien määrä
  duration: "30s", // Testin kesto
  thresholds: {
    http_req_duration: ["p(95)<500"], // 95% pyynnöistä alle 500ms
    http_req_failed: ["rate<0.01"], // Alle 1% pyynnöistä epäonnistuu
  },
};
// Testin pääfunktio, joka suoritetaan jokaiselle virtuaalikäyttäjälle
export default function () {
  // Lähetetään HTTP GET -pyyntö paikalliselle palvelimelle
  const response = http.get("http://127.0.0.1:8000/api/users");
  // Tarkistetaan, että vastaus on onnistunut (status 200)
  check(response, {
    "status is 200": (r) => r.status === 200,
    "response time is acceptable": (r) => r.timings.duration < 500, // Vastausaika alle 500ms
  });
  // Tulostetaan virheviesti, jos pyyntö epäonnistui
  if (response.status !== 200) {
    console.error(`Request failed with status ${response.status}`);
  }
  // Odotetaan 1 sekunti ennen seuraavaa pyyntöä

  sleep(1);
}
