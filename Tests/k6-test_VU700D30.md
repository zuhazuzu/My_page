PS C:\vuosi 3\My_page\Tests> k6 run k6-test.js

         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: k6-test.js
        output: -

     scenarios: (100.00%) 1 scenario, 700 max VUs, 1m0s max duration (incl. graceful stop):
              * default: 700 looping VUs for 30s (gracefulStop: 30s)

WARN[0023] The test has generated metrics with 100135 unique time series, which is higher than the suggested limit of 100000 and could cause high memory usage. Consider not using high-cardinality values like unique IDs as metric tags or, if you need them in the URL, use the name metric tag or URL grouping. See https://grafana.com/docs/k6/latest/using-k6/tags-and-groups/ for details.  component=metrics-engine-ingester

     ✓ POST status is 200
     ✓ GET status is 200
     ✓ PUT status is 200
     ✓ DELETE status is 204

     checks.........................: 100.00% 31784 out of 31784
     data_received..................: 691 MB  22 MB/s
     data_sent......................: 4.8 MB  150 kB/s
     http_req_blocked...............: avg=1.44ms   min=0s med=0s       max=132.81ms p(90)=0s       p(95)=0s
     http_req_connecting............: avg=1.42ms   min=0s med=0s       max=118.74ms p(90)=0s       p(95)=0s
   ✗ http_req_duration..............: avg=433.69ms min=0s med=366.09ms max=1.96s    p(90)=915.27ms p(95)=1.09s        
       { expected_response:true }...: avg=433.69ms min=0s med=366.09ms max=1.96s    p(90)=915.27ms p(95)=1.09s        
   ✓ http_req_failed................: 0.00%   0 out of 31784
     http_req_receiving.............: avg=1.09ms   min=0s med=669.2µs  max=90.7ms   p(90)=2.23ms   p(95)=3.15ms       
     http_req_sending...............: avg=31.22µs  min=0s med=0s       max=14.16ms  p(90)=0s       p(95)=0s
     http_req_tls_handshaking.......: avg=0s       min=0s med=0s       max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=432.56ms min=0s med=364.98ms max=1.94s    p(90)=913.79ms p(95)=1.09s        
     http_reqs......................: 31784   994.220284/s
     iteration_duration.............: avg=2.74s    min=1s med=2.57s    max=4.91s    p(90)=3.93s    p(95)=4.72s        
     iterations.....................: 7946    248.555071/s
     vus............................: 68      min=68             max=700
     vus_max........................: 700     min=700            max=700

                                                                                                                      
running (0m32.0s), 000/700 VUs, 7946 complete and 0 interrupted iterations                                            
default ✓ [======================================] 700 VU 30s                                      
ERRO[0032] thresholds on metrics 'http_req_duration' have been crossed