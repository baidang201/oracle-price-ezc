# 1 build docker
```
docker build --tag baidang201/oracle_price_ezc .
```


# 1 test docker
```
docker run -v /eth.keystore:/eth.keystore baidang201/oracle_price_ezc 
```