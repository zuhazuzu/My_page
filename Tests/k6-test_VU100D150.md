PS C:\vuosi 3\My_page\Tests> k6 run k6-test.js

         /\      Grafana   /‾‾/  
    /\  /  \     |\  __   /  /   
   /  \/    \    | |/ /  /   ‾‾\ 
  /          \   |   (  |  (‾)  |
 / __________ \  |_|\_\  \_____/ 

     execution: local
        script: k6-test.js
        output: -

     scenarios: (100.00%) 1 scenario, 100 max VUs, 3m0s max duration (incl. graceful stop):
              * default: 100 looping VUs for 2m30s (gracefulStop: 30s)

WARN[0058] The test has generated metrics with 100054 unique time series, which is higher than the suggested limit of 100000 and could cause high memory usage. Consider not using high-cardinality values like unique IDs as metric tags or, if you need them in the URL, use the name metric tag or URL grouping. See https://grafana.com/docs/k6/latest/using-k6/tags-and-groups/ for details.  component=metrics-engine-ingester
WARN[0115] The test has generated metrics with 200035 unique time series, which is higher than the suggested limit of 100000 and could cause high memory usage. Consider not using high-cardinality values like unique IDs as metric tags or, if you need them in the URL, use the name metric tag or URL grouping. See https://grafana.com/docs/k6/latest/using-k6/tags-and-groups/ for details.  component=metrics-engine-ingester

     ✓ POST status is 200
     ✓ GET status is 200
     ✓ PUT status is 200
     ✓ DELETE status is 204

     checks.........................: 100.00% 57428 out of 57428
     data_received..................: 52 MB   341 kB/s
     data_sent......................: 8.7 MB  58 kB/s
     http_req_blocked...............: avg=12.77µs min=0s med=0s     max=40.19ms  p(90)=0s      p(95)=0s
     http_req_connecting............: avg=6.2µs   min=0s med=0s     max=17.27ms  p(90)=0s      p(95)=0s
   ✓ http_req_duration..............: avg=11.37ms min=0s med=8.42ms max=116.39ms p(90)=21.85ms p(95)=28.98ms
       { expected_response:true }...: avg=11.37ms min=0s med=8.42ms max=116.39ms p(90)=21.85ms p(95)=28.98ms
   ✓ http_req_failed................: 0.00%   0 out of 57428
     http_req_receiving.............: avg=295.7µs min=0s med=0s     max=50.03ms  p(90)=1ms     p(95)=1.51ms
     http_req_sending...............: avg=21.72µs min=0s med=0s     max=5.53ms   p(90)=0s      p(95)=0s
     http_req_tls_handshaking.......: avg=0s      min=0s med=0s     max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=11.05ms min=0s med=8.15ms max=115.83ms p(90)=21.54ms p(95)=28.74ms
     http_reqs......................: 57428   380.2728/s
     iteration_duration.............: avg=1.04s   min=1s med=1.03s  max=1.26s    p(90)=1.08s   p(95)=1.1s
     iterations.....................: 14357   95.0682/s
     vus............................: 2       min=2              max=100
     vus_max........................: 100     min=100            max=100

                                                                                                                      
running (2m31.0s), 000/100 VUs, 14357 complete and 0 interrupted iterations                                           
default ✓ [======================================] 100 VUs  2m30s

**Tässä testissä luotuijen iteraatioiden määrä ylitti suositellun rajan(10000kpl)**