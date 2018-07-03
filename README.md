[![Build Status](https://travis-ci.org/CleanCut/rusty_sword_arena.svg?branch=master)](https://travis-ci.org/CleanCut/rusty_sword_arena)
[![](http://meritbadge.herokuapp.com/rusty_sword_arena)](https://crates.io/crates/rusty_sword_arena)


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

### Install Other Dependencies

ZeroMQ is used under-the-hood for networking.  It's abstracted away, so you won't actually deal with it other than
making sure the library portion of it is installed so Rust can find it.

On Linux, the alsa development libraries are needed for sound.

**macOS**

Make sure you have [Homebrew](https://brew.sh/) installed and then run
```bash
brew install pkgconfig zmq
```

**CentOS**

```bash
sudo yum install -y czmq-devel alsa-lib-devel
```

**Debian**
Run the following AND follow the instructions for Debian in the
[ZeroMQ's download documentation](http://zeromq.org/area:download) (you might need to create an
`/etc/apt/sources.list.d/zeromq.list` file if you don't have a `sources.list` file on Debian 9) :

```bash
sudo apt install libasound2-dev
```

**Other Operating Systems**

Follow the instructions in
[ZeroMQ's download documentation](http://zeromq.org/area:download) for your operating system.

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

# Prepare to Learn

Please do all of the following (see the 
[How To Learn Rust](https://github.com/CleanCut/rusty_sword_arena/blob/master/HowToLearnRust.md) page)
- [ ] Choose an IDE (or Editor) and configure it with Rust support and customize it to your liking
- [ ] Choose one place to "find answers" and either introduce yourself (if it's a forum, IRC, etc.) or find the answer
      to one question you have.
- [ ] Try doing something in Rust!  If you don't have a better idea, then just do this:
  - `cargo new say`
  - `cd say`
  - `cargo run`
  - Edit `src/main.rs` and change the message.
  - `cargo run` again to see your new message.
- [ ] Check out the descriptions of the tools and books. 

# Tutorial!

Now you are ready for the tutorial! You are going to make your own game client far Rusty Sword Arena!

Your resources will be:

- Presentation & walk-through by the instructor (Nathan Stocks)
- [Rusty Sword Arena Crate Docs](https://agileperception.com/doc/rusty_sword_arena/)
- [How To Learn Rust](https://github.com/CleanCut/rusty_sword_arena/blob/master/HowToLearnRust.md)
- [The Rust Standard Library](https://doc.rust-lang.org/std/)
