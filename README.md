# Rusty Sword Arena: A Crash Course in Rust

This is the companion repository to the half-day tutorial for OSCON 2018.

# Preparation

### Install Rust

- Go to [rust-lang.org](https://rust-lang.org) and click on the big blue `Install Rust` 
  button and follow the instructions to install Rust for your operating system.
  - Please DO NOT install rust via some other package manager!
  
 You should get somewhat similar output (versions may be newer) if you run commands like these:
 
```shell
$ rustup --version
rustup 1.11.0 (e751ff9f8 2018-02-13)

$ rustc --version
rustc 1.26.1 (827013a31 2018-05-25)

$ cargo --version
cargo 1.26.0 (0e7c5a931 2018-04-06)
```

If you have any trouble with installation or running the commands above, please [contact me](mailto:nathan.stocks@gmail.com) before OSCON!!!

### Install ZeroMQ

On macOS, using [Homebrew](https://brew.sh/) it is as easy as `brew install zmq`.

For all other operating systems, please see
[ZeroMQ's download documentation](http://zeromq.org/area:download) -- you are generally looking for `libzmq`

We will be using ZeroMQ for our network communication.

### Install VulkanSDK

On Windows and Linux, the theory is that graphics will work without you having to install anything.  If that turns out 
not to be true, then [you can find the VulkanSDK downloads here](https://vulkan.lunarg.com/sdk/home).

On macOS, 
[download the macOS VulkanSDK (MoltenVK)](https://vulkan.lunarg.com/sdk/home#sdk/downloadConfirm/1.1.73.0/mac/vulkansdk-macos-1.1.73.0.tar.gz),
uncompress it, and  