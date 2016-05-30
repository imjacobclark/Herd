# Herd
Herd was a small side project in building a HTTP load testing application in Rust with a main focus on being easy to use and low on OS level dependencies such as the JVM.

Herd was developed on a pre-1.0 version of Rust, it is not unit tested and unlikley to compile on later versions of Rust, I intend to revisit this project as the Rust ecosystem matures.

Herd can spawn thousands of concurrent requests to HTTP endpoints by making use of unix process forking and multithreadding. 

Herd is hardware intensive. Per each 1000 virtual users, Herd will fork itself in order to overcome hard limits set by the OS on how many active threads a process can have at any one time. Due to this, it is recomended you run Herd from a disposable machine in the cloud, high levels of virtual users can crash machines if they run out of memory or CPU resources, __do not run Herd on a production machine__.

##### Download and run

As Herd is unlikley to compile out the box, it is recomended you clone this repository and fully build from source, some alterations to the source code may be needed over time as the Rust language changes, I do not intend on continuously maintaining this project.

####### Dependencies:

`gcc clang libssl rust cargo`

###### Compile from source and run:
```shell
$ git clone git@github.com:imjacobclark/Herd.git && cd Herd
$ cargo build --release
$ ./target/release/herd -t <THREADS(int)> -r <REQUESTS(int)> <HOST(str)>
```

You can define several hosts to test with a configuration file. See [example-config.yaml](example-config.yaml).

```shell
$ ./target/release/herd -c config.yaml
```

