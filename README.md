# Herd
An experimental HTTP load testing application written in Rust.

Herd is an experimental HTTP load testing application written in Rust. It's main focus is being easy to use and low on OS level depencies such as the JVM.

Herd is currently able to spawn thousands of concurrent requests to HTTP endpoints by making use of unix process forking and multithreadding.

Herd is hardware intensive. Per each 100 virtual users, Herd will fork itself in order to overcome hard limits set by the OS on how many active threads a process can have at any one time. Due to this, it is recomended you run Herd from a disposable machine in the cloud, high levels of virtual users can crash machines if they run out of memory or CPU resources, __do not run Herd on a production machine__.

###### Todo

* ~~Linear scenario~~
* Ramp up scenario
* Batched scenario
* Accept command line flags to begin a test
* Ability to create tests via a YAML config file
* Export data to influxDB or other data store
* Graph metrics

###### Download and run

Grab the latest stable **preview** release of Herd [here](https://github.com/imjacobclark/Herd/releases).

```shell
$ chmod +x Herd
$ ./Herd <THREADS(int)> <REQUESTS(int)> <HOST(str)>
```

###### Depdendencies:

`gcc clang libssl rust cargo`

###### Compile from source and run:
```shell
$ git clone git@github.com:imjacobclark/Herd.git && cd Herd
$ cargo build
$ ./target/debug/Herd <THREADS(int)> <REQUESTS(int)> <HOST(str)>
```
