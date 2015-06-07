# Herd
An `simple` but experimental HTTP load testing client written in Rust

Due to the current immaturity of async in Rust via libraries such as [mio](https://github.com/carllerche/mio), the amount of concurrent HTTP requests Herd can make is totally dependent on the hardware of the system Herd is executed on. 

Upon the maturity of such libraries and the ability to execute HTTP requests within them, it would be theoretically possible for Herd to create recursive function calls which do not block the main iterator, as such frees Herd from using an inefficient threading mechanism. Until that time, Herd will continue to be incomplete!

Feel free to contribute via PRs.

###### Depdendencies:

`gcc clang libssl`

###### Compile and Run:
```shell
$ git clone git@github.com:imjacobclark/Herd.git && cd Herd
$ cargo build
$ ./target/debug/Herd
```
