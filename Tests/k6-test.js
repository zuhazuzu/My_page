import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
    vus: 100,
    duration: '30s',
    thresholds: {
        http_req_duration: ['p(95)<500'],
        http_req_failed: ['rate<0.01'],
    },
};

export default function () {
    const baseUrl = 'http://127.0.0.1:8000/api/users';

    // 🔧 Uniikki testikäyttäjä
    const payload = JSON.stringify({
        name: `Test User ${__VU}-${__ITER}`,
        email: `test${__VU}${__ITER}@example.com`
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    // ✅ 1. POST – lisää käyttäjä
    const postRes = http.post(baseUrl, payload, params);
    check(postRes, {
        'POST status is 200': (r) => r.status === 200,
    });

    if (postRes.status !== 200) {
        console.error(`POST failed: ${postRes.status} - ${postRes.body}`);
        return;
    }

    // ✅ 2. GET – hae käyttäjät ja poimi viimeinen lisätty
    const getRes = http.get(baseUrl);
    check(getRes, {
        'GET status is 200': (r) => r.status === 200,
    });

    const users = getRes.json();
    const latestUser = users[users.length - 1];
    if (!latestUser || !latestUser.id) {
        console.error('Could not retrieve latest user for PUT/DELETE');
        return;
    }

    const userId = latestUser.id;

    // ✅ 3. PUT – päivitä käyttäjä
    const updatePayload = JSON.stringify({
        name: `Updated User ${__VU}-${__ITER}`,
        email: `updated${__VU}${__ITER}@example.com`
    });

    const putRes = http.put(`${baseUrl}/${userId}`, updatePayload, params);
    check(putRes, {
        'PUT status is 200': (r) => r.status === 200,
    });

    if (putRes.status !== 200) {
        console.error(`PUT failed: ${putRes.status} - ${putRes.body}`);
    }

    // ✅ 4. DELETE – poista käyttäjä
    const delRes = http.del(`${baseUrl}/${userId}`);
    check(delRes, {
        'DELETE status is 204': (r) => r.status === 204,
    });

    if (delRes.status !== 204) {
        console.error(`DELETE failed: ${delRes.status} - ${delRes.body}`);
    }

    sleep(1);
}
