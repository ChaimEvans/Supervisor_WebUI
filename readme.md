# Supervisor_WebUI
A webui application for Supervisor to multiple supervisor servers

## Build
```shell
cargo build --release
```

## Usage
```shell
supervisor_webui PORT URLs
```
```shell
supervisor_webui 8080 "http://172.14.0.2:9000/RPC2" "http://172.14.0.2:9000/RPC2" "http://172.14.0.2:9000/RPC2" 
```