# microps-rs
Re-implementation of the TCP/IP protocol stack [microps](https://github.com/pandax381/microps) with Rust 

## Usage 

1. Prepare TAP device

```bash
sudo ip tuntap add mode tap user $USER name tap0
sudo ip addr add 192.0.2.1/24 dev tap0
sudo ip link set tap0 up
```

2. Build and run

```bash 
cargo run
```

## Reference
[microps](https://github.com/pandax381/microps)

## License
[MIT](./LICENSE)
