# 1 build docker
```
docker build --tag baidang201/oracle_price_ezc --platform=linux/arm64/v8 .
```


# 1 test docker
```
docker run -v /eth.keystore:/eth.keystore baidang201/oracle_price_ezc 
```