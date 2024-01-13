# microps-rs
Re-implementation of the TCP/IP protocol stack [microps](https://github.com/pandax381/microps) with Rust 

## Setup
### TAP Device Preparation 
```sh
$ sudo ip tuntap add mode tap user $USER name tap0
$ sudo ip addr add 192.0.2.1/24 dev tap0
$ sudo ip link set tap0 up
```

## Reference
[microps](https://github.com/pandax381/microps)

## License
[MIT](./LICENSE)