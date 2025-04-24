PS C:\vuosi 3\My_page\Tests> k6 run k6-test.js

         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: k6-test.js
        output: -

     scenarios: (100.00%) 1 scenario, 1000 max VUs, 1m0s max duration (incl. graceful stop):
              * default: 1000 looping VUs for 30s (gracefulStop: 30s)

WARN[0032] The test has generated metrics with 100072 unique time series, which is higher than the suggested limit of 100000 and could cause high memory usage. Consider not using high-cardinality values like unique IDs as metric tags or, if you need them in the URL, use the name metric tag or URL grouping. See https://grafana.com/docs/k6/latest/using-k6/tags-and-groups/ for details.  component=metrics-engine-ingester

     ✓ POST status is 200
     ✓ GET status is 200
     ✓ PUT status is 200
     ✓ DELETE status is 204

     checks.........................: 100.00% 22732 out of 22732
     data_received..................: 903 MB  27 MB/s
     data_sent......................: 3.4 MB  101 kB/s
     http_req_blocked...............: avg=4.24ms  min=0s      med=0s      max=182.52ms p(90)=0s     p(95)=540.18µs    
     http_req_connecting............: avg=4.2ms   min=0s      med=0s      max=155.58ms p(90)=0s     p(95)=0s
   ✗ http_req_duration..............: avg=1.18s   min=505.8µs med=1.06s   max=3.43s    p(90)=2.15s  p(95)=2.52s       
       { expected_response:true }...: avg=1.18s   min=505.8µs med=1.06s   max=3.43s    p(90)=2.15s  p(95)=2.52s       
   ✓ http_req_failed................: 0.00%   0 out of 22732
     http_req_receiving.............: avg=1.14ms  min=0s      med=754.2µs max=83.57ms  p(90)=2.34ms p(95)=3.27ms      
     http_req_sending...............: avg=68.28µs min=0s      med=0s      max=51.89ms  p(90)=0s     p(95)=401.83µs    
     http_req_tls_handshaking.......: avg=0s      min=0s      med=0s      max=0s       p(90)=0s     p(95)=0s
     http_req_waiting...............: avg=1.18s   min=505.8µs med=1.06s   max=3.42s    p(90)=2.15s  p(95)=2.51s       
     http_reqs......................: 22732   670.34883/s
     iteration_duration.............: avg=5.75s   min=1s      med=5.72s   max=8.44s    p(90)=8.15s  p(95)=8.23s       
     iterations.....................: 5683    167.587207/s
     vus............................: 653     min=653            max=1000
     vus_max........................: 1000    min=1000           max=1000

                                                                                                                      
running (0m33.9s), 0000/1000 VUs, 5683 complete and 0 interrupted iterations                                          
default ✓ [======================================] 1000 VUs  30s                                                      
ERRO[0034] thresholds on metrics 'http_req_duration' have been crossed