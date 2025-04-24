PS C:\vuosi 3\My_page\Tests> k6 run k6-test.js

         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: k6-test.js
        output: -

     scenarios: (100.00%) 1 scenario, 500 max VUs, 1m0s max duration (incl. graceful stop):
              * default: 500 looping VUs for 30s (gracefulStop: 30s)


     ✓ POST status is 200
     ✓ GET status is 200
     ✓ PUT status is 200
     ✓ DELETE status is 204

     checks.........................: 100.00% 13284 out of 13284
     data_received..................: 1.1 GB  33 MB/s
     data_sent......................: 2.0 MB  62 kB/s
     http_req_blocked...............: avg=1.15ms   min=0s      med=0s       max=65.35ms p(90)=0s     p(95)=0s
     http_req_connecting............: avg=1.14ms   min=0s      med=0s       max=65.35ms p(90)=0s     p(95)=0s
   ✗ http_req_duration..............: avg=939.42ms min=515.9µs med=956.19ms max=2.48s   p(90)=1.57s  p(95)=1.78s      
       { expected_response:true }...: avg=939.42ms min=515.9µs med=956.19ms max=2.48s   p(90)=1.57s  p(95)=1.78s      
   ✓ http_req_failed................: 0.00%   0 out of 13284
     http_req_receiving.............: avg=1.52ms   min=0s      med=1.38ms   max=94.98ms p(90)=2.69ms p(95)=3.42ms     
     http_req_sending...............: avg=37.56µs  min=0s      med=0s       max=8.18ms  p(90)=0s     p(95)=458.9µs    
     http_req_tls_handshaking.......: avg=0s       min=0s      med=0s       max=0s      p(90)=0s     p(95)=0s
     http_req_waiting...............: avg=937.86ms min=515.1µs med=954.6ms  max=2.48s   p(90)=1.57s  p(95)=1.78s      
     http_reqs......................: 13284   409.217692/s
     iteration_duration.............: avg=4.76s    min=1.02s   med=5.21s    max=5.76s   p(90)=5.54s  p(95)=5.58s      
     iterations.....................: 3321    102.304423/s
     vus............................: 324     min=324            max=500
     vus_max........................: 500     min=500            max=500

                                                                                                                      
running (0m32.5s), 000/500 VUs, 3321 complete and 0 interrupted iterations                                            
default ✓ [======================================] 500 VUs  30s                                                       
ERRO[0032] thresholds on metrics 'http_req_duration' have been crossed