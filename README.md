# Herd
An __simple__ but __experimental__ HTTP load testing client written in Rust

Herd is hardware intensive. Per each 100 virtual users, Herd will fork itself in order to overcome hard limits set by the OS on how many active threads a process can have at any one time. Due to this, it is recomended you run Herd from a disposable machine in the cloud, high levels of virtual users can crash machines if they run out of memory or CPU resources, __do not run Herd on a production machine__.

###### Depdendencies:

`gcc clang libssl`

###### Compile and Run:
```shell
$ git clone git@github.com:imjacobclark/Herd.git && cd Herd
$ cargo build
$ ./target/debug/Herd <THREADS(int)> <REQUESTS(int)> <HOST(str)>
```
