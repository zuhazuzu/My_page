PS C:\vuosi 3\My_page\Tests> k6 run k6-test.js

         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: k6-test.js
        output: -

     scenarios: (100.00%) 1 scenario, 100 max VUs, 1m0s max duration (incl. graceful stop):
              * default: 100 looping VUs for 30s (gracefulStop: 30s)


     ✓ POST status is 200
     ✓ GET status is 200
     ✓ PUT status is 200
     ✓ DELETE status is 204

     checks.........................: 100.00% 11508 out of 11508
     data_received..................: 12 MB   398 kB/s
     data_sent......................: 1.7 MB  56 kB/s
     http_req_blocked...............: avg=130.17µs min=0s med=0s     max=21.29ms  p(90)=0s      p(95)=0s
     http_req_connecting............: avg=124.75µs min=0s med=0s     max=21.29ms  p(90)=0s      p(95)=0s
   ✓ http_req_duration..............: avg=13.56ms  min=0s med=8.97ms max=124.96ms p(90)=28.82ms p(95)=43.38ms
       { expected_response:true }...: avg=13.56ms  min=0s med=8.97ms max=124.96ms p(90)=28.82ms p(95)=43.38ms
   ✓ http_req_failed................: 0.00%   0 out of 11508
     http_req_receiving.............: avg=337.24µs min=0s med=0s     max=68.09ms  p(90)=1.11ms  p(95)=1.62ms
     http_req_sending...............: avg=22.26µs  min=0s med=0s     max=7.65ms   p(90)=0s      p(95)=0s
     http_req_tls_handshaking.......: avg=0s       min=0s med=0s     max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=13.2ms   min=0s med=8.68ms max=124.45ms p(90)=28.54ms p(95)=42.9ms
     http_reqs......................: 11508   371.120155/s
     iteration_duration.............: avg=1.05s    min=1s med=1.04s  max=1.28s    p(90)=1.1s    p(95)=1.17s
     iterations.....................: 2877    92.780039/s
     vus............................: 5       min=5              max=100
     vus_max........................: 100     min=100            max=100

                                                                                                                      
running (0m31.0s), 000/100 VUs, 2877 complete and 0 interrupted iterations                                            