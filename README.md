# Herd
An __simple__ but __experimental__ HTTP load testing client written in Rust

Due to the current immaturity of async in Rust via libraries such as [mio](https://github.com/carllerche/mio), the amount of concurrent HTTP requests Herd can make is totally dependent on the hard limit of threads set the by OS per process (2048 in OSX) and the hardware of the system Herd is executed on. 

Currently looking at solutions to get around these problems, feel free to suggest solutions or propose changes via pull requests.

###### Depdendencies:

`gcc clang libssl`

###### Compile and Run:
```shell
$ git clone git@github.com:imjacobclark/Herd.git && cd Herd
$ cargo build
$ ./target/debug/Herd
```
