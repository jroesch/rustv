rustv[![Build Status](https://secure.travis-ci.org/jroesch/rustv.png)](http://travis-ci.org/jroesch/rustv)
======
`rustv` is about giving you a way to easily manage multiple versions of the
Rust compiler and associated tools. Currently rustv-build will install a
version of Cargo next to your compiler installation since Cargo will be
a necessary part of any serious Rust project.

This should make the lives of compiler developers, and library authors easier.
Especially when it comes to supporting multiple versions of your tool it will be
as simple as `rustv local 0.11.0`, `cargo test`, `rustv local 0.12.0`,
`cargo test`.

# Installation
To install simply clone the repository: 
```
git clone https://github.com/jroesch/rustv
```

Build the project with Cargo:
```
cargo build
```

Then run the install script:
```
./install.sh
```

Eventually we will support binary installation that doesn't require a Rust compiler to build.

# Notes
Currently there is no window support. If anyone is interested in helping out with this I know
absoutely nothing about building and installing software on Windows.
