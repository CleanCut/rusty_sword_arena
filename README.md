# Rusty Sword Arena: A Crash Course in Rust

This is the companion repository to the half-day tutorial for OSCON 2018.

# Preparation

### Install Rust

Rust 1.27.0 or newer is required for Rusty Sword Arena!

- Go to [rust-lang.org](https://rust-lang.org) and click on the big blue `Install Rust` 
  button and follow the instructions to install Rust for your operating system.
  - Please DO NOT install rust via some other package manager!
  
You should get somewhat similar output (versions may be newer) if you run commands like the ones below.  If you get a
version older than 1.27.0, then run `rustup update` to install a newer version.
 
```shell
$ rustc --version
rustc 1.27.0 (3eda71b00 2018-06-19)

$ cargo --version
cargo 1.27.0 (1e95190e5 2018-05-27)
```

If you have any trouble with installation or running the commands above, please [contact me](mailto:nathan.stocks@gmail.com) before OSCON!!!

### Install ZeroMQ

ZeroMQ is used under-the-hood for networking.  It's abstracted away, so you won't actually deal with it other than
making sure the library portion of it is installed so Rust can find compile it in.

On macOS, using [Homebrew](https://brew.sh/) it is as easy as `brew install zmq`.

On CentOS: `yum install -y czmq-devel`

For all other operating systems, please see
[ZeroMQ's download documentation](http://zeromq.org/area:download)

