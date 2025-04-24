PS C:\vuosi 3\My_page\Tests> k6 run k6-test.js

         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: k6-test.js
        output: -

     scenarios: (100.00%) 1 scenario, 50 max VUs, 1m0s max duration (incl. graceful stop):
              * default: 50 looping VUs for 30s (gracefulStop: 30s)


     ✓ POST status is 200
     ✓ GET status is 200
     ✓ PUT status is 200
     ✓ DELETE status is 204

     checks.........................: 100.00% 5716 out of 5716
     data_received..................: 6.2 MB  200 kB/s
     data_sent......................: 858 kB  28 kB/s
     http_req_blocked...............: avg=44.07µs min=0s med=0s      max=6.66ms   p(90)=0s      p(95)=0s
     http_req_connecting............: avg=35.86µs min=0s med=0s      max=6.66ms   p(90)=0s      p(95)=0s
   ✓ http_req_duration..............: avg=16.33ms min=0s med=12.5ms  max=121.18ms p(90)=33.88ms p(95)=40.16ms
       { expected_response:true }...: avg=16.33ms min=0s med=12.5ms  max=121.18ms p(90)=33.88ms p(95)=40.16ms
   ✓ http_req_failed................: 0.00%   0 out of 5716
     http_req_receiving.............: avg=362.4µs min=0s med=0s      max=28.53ms  p(90)=1.18ms  p(95)=1.86ms
     http_req_sending...............: avg=35.65µs min=0s med=0s      max=9.54ms   p(90)=0s      p(95)=0s
     http_req_tls_handshaking.......: avg=0s      min=0s med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=15.93ms min=0s med=12.21ms max=121.18ms p(90)=33.51ms p(95)=39.74ms
     http_reqs......................: 5716    185.073614/s
     iteration_duration.............: avg=1.06s   min=1s med=1.05s   max=1.2s     p(90)=1.12s   p(95)=1.13s
     iterations.....................: 1429    46.268404/s
     vus............................: 50      min=50           max=50
     vus_max........................: 50      min=50           max=50


running (0m30.9s), 00/50 VUs, 1429 complete and 0 interrupted iterations
default ✓ [======================================] 50 VUs  30s