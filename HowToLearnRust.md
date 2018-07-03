# How To Learn Rust

Perhaps more important than a crash course tutorial in Rust is learning _how to learn Rust_.  You can sit back and
listen to the tutorial and maybe you will learn a little something.  Learning how to learn Rust will put you on a path
that will lead to eventual mastery.  It is up to you!

### IDE / Editor

Big, bloated IDE's can actually be really helpful in learning Rust.  IDE support is part of the core Rust project, and
it is already getting really good.  Much more than just syntax highlighting, an IDE like IntelliJ will integrate with
the compiler and show you type hints, compiler check errors, and all sorts of good stuff inline. 

- Google: Rust _(your favorite IDE)_
  - Install Rust support
  - Install TOML support, which is often separate from Rust support ([TOML](https://github.com/toml-lang/toml)
    is the config file format that Rust uses)
  - ...wait...
  - Be amazed at all the helpful auto-complete, etc. that turns on.  Yay!
  - Customize your editor to your liking.

### Find Answers

You are always going to have questions.  Know how to find the answers.

- If it is about something the standard library, then Google: `rust std (thing you want to find)`
  - For example, can't quite remember what that method on `Vec` was? Google `rust std Vec`
- There is a very welcoming [Rust Community](https://www.rust-lang.org/en-US/community.html) out there that you can
  communicate with.  See the link above for:
  - Forums
  - IRC channels
  - StackOverflow topics
  - News (The [weekly newsletter](https://this-week-in-rust.org/) is seriously fantastic)
  - YouTube channel
  - User Groups and Meetups
  - Where to find and communicate with all the core Rust Teams

### Play Around

Code something.  Don't just sit and watch the tutorial.  Try stuff out!

- Do the stuff in the tutorial!
- Don't be afraid to just `cargo new blah` and write a 5-line throwaway program to try something out.
- Start an interesting little project
  - If you get stuck, or the project gets boring...no worries! Just start another interesting little project...
- Find an existing project that looks interesting
  - Try it out
  - Try to contribute a bug fix or feature
- Rewrite some existing little project in Rust (in a new project)
  - Compare the results
  - What did you like better about Rust?
  - What did you like better about the other language?
  - Compare binary size, memory usage, speed, etc.
- Write a blog post about your experience!


### Tools

There are tools that help you learn as well.

- [Clippy](https://github.com/rust-lang-nursery/rust-clippy) is a super-amazing linter.  It will tell you how to change
  working code into _idiomatic_ and _high-performing_ code.
- [rustfmt](https://github.com/rust-lang-nursery/rustfmt) will format your code according to Rust style guidelines.
  There's only one set of Rust style guidelines...so there's nothing to argue about!

### Reading

Long-format reading is really interesting and informative. You will learn some things plowing through a comprehensive
book that you would never have encountered during years of reading random bits of the standard library reference.  I 
found these books _especially_ useful and high quality:

**Educational**

- [The Rust Programming Language](https://doc.rust-lang.org/book/), aka "The Book" - the official free online book 
  about the language.  I have read the first _and_ second edition, and they're both great.  At this point in time it
  probably makes more sense to go straight to the second edition.
- [Programming Rust](https://amzn.to/2KC72XV) - The O'Reilly book by Jim Blandy and Jason Orendorff.  Fantastic book
  focused on using the Rust language.  Much longer and more in depth than you can get elsewhere.

**Informational**

- [Entering the Quantum Era—How Firefox got fast again and where it’s going to get faster](https://hacks.mozilla.org/2017/11/entering-the-quantum-era-how-firefox-got-fast-again-and-where-its-going-to-get-faster/)

**Random References**
- [TOML Format](https://github.com/toml-lang/toml) - the config file format Rust uses
- [Semantic Versioning](https://semver.org/) and [Cargo's Version Field Rules](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
