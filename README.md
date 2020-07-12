For autoloading, install with 
```
cargo install systemfd cargo-watch
```
After the first time installation, run the following command to compile, watch and run
```
systemfd -- cargo watch -x run
```

Endpoint: GET localhost:8080/API/math/prime/{number}
