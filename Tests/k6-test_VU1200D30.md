PS C:\vuosi 3\My_page\Tests> k6 run k6-test.js

         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: k6-test.js
        output: -

     scenarios: (100.00%) 1 scenario, 1200 max VUs, 1m0s max duration (incl. graceful stop):
              * default: 1200 looping VUs for 30s (gracefulStop: 30s)

**TÄMÄ TOISTUU n- MÄÄRÄN**                                      
WARN[0000] Request Failed                                error="Post \"http://127.0.0.1:8000/api/users\": dial tcp 127.0.0.1:8000: connectex: No connection could be made because the target machine actively refused it."                  
rce=console                                               
   
ERRO[0000] POST failed: 0 - null                         source=console                                               
**LOOPPI LOPPUU TÄHÄN**       

     ✗ POST status is 200
      ↳  99% — ✓ 5342 / ✗ 36
     ✓ GET status is 200
     ✓ PUT status is 200
     ✓ DELETE status is 204

     checks.........................: 99.83% 21368 out of 21404
     data_received..................: 1.1 GB 30 MB/s
     data_sent......................: 3.2 MB 89 kB/s
     http_req_blocked...............: avg=4.86ms   min=0s      med=0s      max=292.97ms p(90)=0s     p(95)=69ms       
     http_req_connecting............: avg=4.78ms   min=0s      med=0s      max=164.79ms p(90)=0s     p(95)=68.8ms     
   ✗ http_req_duration..............: avg=1.68s    min=0s      med=1.61s   max=5.02s    p(90)=3.21s  p(95)=3.56s      
       { expected_response:true }...: avg=1.69s    min=510.1µs med=1.61s   max=5.02s    p(90)=3.21s  p(95)=3.56s      
   ✓ http_req_failed................: 0.16%  36 out of 21404
     http_req_receiving.............: avg=1.19ms   min=0s      med=951.2µs max=74.28ms  p(90)=2.28ms p(95)=3.17ms     
     http_req_sending...............: avg=184.26µs min=0s      med=0s      max=220.54ms p(90)=0s     p(95)=505.8µs    
     http_req_tls_handshaking.......: avg=0s       min=0s      med=0s      max=0s       p(90)=0s     p(95)=0s
     http_req_waiting...............: avg=1.68s    min=0s      med=1.61s   max=5s       p(90)=3.2s   p(95)=3.56s      
     http_reqs......................: 21404  593.419923/s
     iteration_duration.............: avg=7.73s    min=79.02ms med=8.23s   max=10.72s   p(90)=10.4s  p(95)=10.54s     
     iterations.....................: 5378   149.103548/s
     vus............................: 209    min=209            max=1200
     vus_max........................: 1200   min=1200           max=1200

                                                                                                                      
running (0m36.1s), 0000/1200 VUs, 5378 complete and 0 interrupted iterations                                          
default ✓ [======================================] 1200 VUs  30s                                                      
ERRO[0036] thresholds on metrics 'http_req_duration' have been crossed


---

PS C:\vuosi 3\My_page\Tests> k6 run k6-test.js

         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: k6-test.js
        output: -

     scenarios: (100.00%) 1 scenario, 1200 max VUs, 1m0s max duration (incl. graceful stop):
              * default: 1200 looping VUs for 30s (gracefulStop: 30s)


     ✓ POST status is 200
     ✓ GET status is 200
     ✓ PUT status is 200
     ✓ DELETE status is 204

     checks.........................: 100.00% 9644 out of 9644
     data_received..................: 582 MB  14 MB/s
     data_sent......................: 1.5 MB  35 kB/s
     http_req_blocked...............: avg=20.03ms  min=0s     med=0s     max=288.43ms p(90)=58.18ms  p(95)=191.1ms    
     http_req_connecting............: avg=19.85ms  min=0s     med=0s     max=287.71ms p(90)=57.7ms   p(95)=189.94ms   
   ✗ http_req_duration..............: avg=4.7s     min=8.24ms med=4.46s  max=14.63s   p(90)=7.74s    p(95)=8.42s      
       { expected_response:true }...: avg=4.7s     min=8.24ms med=4.46s  max=14.63s   p(90)=7.74s    p(95)=8.42s      
   ✓ http_req_failed................: 0.00%   0 out of 9644
     http_req_receiving.............: avg=3.27ms   min=0s     med=2.01ms max=429.92ms p(90)=5.32ms   p(95)=7.06ms     
     http_req_sending...............: avg=653.71µs min=0s     med=0s     max=51.79ms  p(90)=424.03µs p(95)=717.58µs   
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s     max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=4.7s     min=7.71ms med=4.45s  max=14.54s   p(90)=7.74s    p(95)=8.42s      
     http_reqs......................: 9644    231.343588/s
     iteration_duration.............: avg=19.92s   min=4.53s  med=17.09s max=26.14s   p(90)=24.73s   p(95)=24.8s      
     iterations.....................: 2411    57.835897/s
     vus............................: 769     min=769          max=1200
     vus_max........................: 1200    min=1200         max=1200

                                                                                                                      
running (0m41.7s), 0000/1200 VUs, 2411 complete and 0 interrupted iterations                                          
default ✓ [======================================] 1200 VUs  30s                                                      
ERRO[0042] thresholds on metrics 'http_req_duration' have been crossed

Toinen testi ei aiheuttanut toimimattomuutta. 