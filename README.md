# Rusty Sword Arena: A Crash Course in Rust

This is the companion repository to the half-day tutorial for OSCON 2018.

# Preparation - **_DO THIS BEFORE OSCON_**

I use macOS, and that is what I developed Rusty Sword Arena on.  Everything _ought_ to be able to work on major Linux 
distributions and Windows. Please do the following preparation _before_ OSCON so we can focus our tutorial time on 
learning Rust instead of twiddling with dependencies.  Please [contact me](mailto:nathan.stocks@gmail.com) ASAP if you
have trouble with anything on this page.

### Install Rust

Rust 1.27.0 or newer is required for Rusty Sword Arena!

- Go to [rust-lang.org](https://rust-lang.org) and click on the big blue `Install Rust` 
  button and follow the instructions to install Rust for your operating system.
  - Please DO NOT install rust via some other package manager, because it will be a version that is _too old_.
  
You should get somewhat similar output (versions may be newer) if you run commands like the ones below.  If you get a
version older than 1.27.0, then run `rustup update` to install a newer version.
 
```shell
$ rustc --version
rustc 1.27.0 (3eda71b00 2018-06-19)

$ cargo --version
cargo 1.27.0 (1e95190e5 2018-05-27)
```

If you have any trouble with installation or running the commands above, please
[contact me](mailto:nathan.stocks@gmail.com) before OSCON!!!

### Install ZeroMQ

ZeroMQ is used under-the-hood for networking.  It's abstracted away, so you won't actually deal with it other than
making sure the library portion of it is installed so Rust can find it.

On macOS, make sure you have [Homebrew](https://brew.sh/) installed and then run
```bash
brew install pkgconfig zmq
```

On CentOS, run: 
```bash
yum install -y czmq-devel
```

For all other operating systems, please see
[ZeroMQ's download documentation](http://zeromq.org/area:download)

### See if everything is working

- Clone this repository
- From inside the repository, run:
```bash
cargo run --bin server
```
- It should compile for a long time and then you should get a startup message and some stats.
  - If your firewall prompts you for whether to allow the server to use the network, choose YES
  - Leave the server running for the next step!  When you're ready to shut it down press `Ctrl-C`
  - If something crashes or goes wrong, please [contact me](mailto:nathan.stocks@gmail.com) before OSCON!!!
- In another terminal window, run
```bash
cargo run --bin client -- yourname localhost
```
  - The server should say something about a player connecting
  - This should compile and then launch a window with a circle with a stripe.
  - The stripe should point at your mouse pointer.
  - The circle can be moved around with the arrow keys or WASD.
  - Stop the client by closing the window. Stop the server by pressing `Ctrl-C` in it's terminal.
  - If something crashes or goes wrong, please [contact me](mailto:nathan.stocks@gmail.com) before OSCON!!!

If you got through all those steps without anything crashing, then you're all ready for OSCON. ✨🎉✨

# Tutorial!

For the tutorial, you are going to make your own game client far Rusty Sword Arena!

Next:
- [How To Learn Rust](https://github.com/CleanCut/rusty_sword_arena/blob/master/HowToLearnRust.md)
- [Rusty Sword Arena Crate Docs](https://agileperception.com/doc/rusty_sword_arena/)