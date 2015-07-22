# Herd
An experimental HTTP load testing application written in Rust.

Herd is an experimental HTTP load testing application written in Rust. It's main focus is being easy to use and low on OS level dependencies such as the JVM.

Herd is currently able to spawn thousands of concurrent requests to HTTP endpoints by making use of unix process forking and multithreadding.

Herd is hardware intensive. Per each 1000 virtual users, Herd will fork itself in order to overcome hard limits set by the OS on how many active threads a process can have at any one time. Due to this, it is recomended you run Herd from a disposable machine in the cloud, high levels of virtual users can crash machines if they run out of memory or CPU resources, __do not run Herd on a production machine__.

###### Todo

* ~~Linear scenario~~
* Ability to create tests via a YAML config file
  * ~~multiple hosts with different threads/requests per host~~
* Ramp up scenario
* Batched scenario
* Accept command line flags to begin a test
* Export data to influxDB or other data store
* Graph metrics
* Master/Slave mode to orchestrate a thundering herd via a message broker

###### Download and run

Grab the latest stable **preview** release of Herd [here](https://github.com/imjacobclark/Herd/releases).

```shell
$ chmod +x herd
$ ./herd -t <THREADS(int)> -r <REQUESTS(int)> <HOST(str)>
```

You can define several hosts to test with a configuration file. See [example-config.yaml](example-config.yaml).

```shell
$ ./herd -c config.yaml
```

###### Dependencies:

`gcc clang libssl rust cargo`

###### Compile from source and run:
```shell
$ git clone git@github.com:imjacobclark/Herd.git && cd Herd
$ cargo build --release
$ ./target/release/herd -t <THREADS(int)> -r <REQUESTS(int)> <HOST(str)>
```
