# Fork of the Rust Programming Language that supports Berkley Packet Filter (BPF) targets

This fork of Rust contains changes that enable rustc to build BPF
modules.  It depends on a customized
[fork](https://github.com/solana-labs/llvm-project) of Rust's LLVM
fork.

Solana SDK does not depend directly on this repo.  Instead [bpf-tools]
builds and releases binary packages that the Solana SDK pulls in.

[bpf-tools]: https://github.com/solana-labs/bpf-tools

BPF modules are built using target triple `bpfel-unknown-unknown`
which represents the little endian version of BPF.  There is no
support for big endian at this time.

Upgrading the compiler and standard library source tree
-------------------------------------------------------

The source tree has two external dependencies
1. [compiler-builtins]
2. [llvm-project]

If any of the depencies is changed or this repository is updated to
make a new release of the bpf-tools, tag the dependencies, and this
repository with a new bpf-tools-v1.x tag, so that all components of
the released bpf-tools have the same tag, e.g. bpf-tools-v1.6. Thus,
release of every version of the bpf-tools is fully specified by the
release version.

The [llvm-project] is a submodule of this repository, therefore its
version is explicitly committed in this repository.  However,
[compiler-builtins] is pulled in as a cargo package.  Therefore, it is
necessary to update the `[patch.crates-io]` subsection of the
top-level `Cargo.toml` file, and specify which tag must be used to
pull the correct version of [compiler-builtins].

After this repository is tagged for a new release, update the
`bpf-tools/build.sh` in [bpf-tools] repository to pull the correct
version of the rust repository and make a new release tag in
[bpf-tools] repository.

[compiler-builtins]: https://github.com/solana-labs/compiler-builtins
[llvm-project]: https://github.com/solana-labs/llvm-project

---

<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg">
    <img alt="The Rust Programming Language: A language empowering everyone to build reliable and efficient software"
         src="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg"
         width="50%">
  </picture>

[Website][Rust] | [Getting started] | [Learn] | [Documentation] | [Contributing]
</div>

This is the main source code repository for [Rust]. It contains the compiler,
standard library, and documentation.

[Rust]: https://www.rust-lang.org/
[Getting Started]: https://www.rust-lang.org/learn/get-started
[Learn]: https://www.rust-lang.org/learn
[Documentation]: https://www.rust-lang.org/learn#learn-use
[Contributing]: CONTRIBUTING.md

## Why Rust?

- **Performance:** Fast and memory-efficient, suitable for critical services, embedded devices, and easily integrated with other languages.

- **Reliability:** Our rich type system and ownership model ensure memory and thread safety, reducing bugs at compile-time.

- **Productivity:** Comprehensive documentation, a compiler committed to providing great diagnostics, and advanced tooling including package manager and build tool ([Cargo]), auto-formatter ([rustfmt]), linter ([Clippy]) and editor support ([rust-analyzer]).

[Cargo]: https://github.com/rust-lang/cargo
[rustfmt]: https://github.com/rust-lang/rustfmt
[Clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer

## Quick Start

Read ["Installation"] from [The Book].

["Installation"]: https://doc.rust-lang.org/book/ch01-01-installation.html
[The Book]: https://doc.rust-lang.org/book/index.html

## Installing from Source

If you really want to install from source (though this is not recommended), see
[INSTALL.md](INSTALL.md).

## Getting Help

See https://www.rust-lang.org/community for a list of chat platforms and forums.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

## Trademark

[The Rust Foundation][rust-foundation] owns and protects the Rust and Cargo
trademarks and logos (the "Rust Trademarks").

If you want to use these names or brands, please read the
[Rust language trademark policy][trademark-policy].

Third-party logos may be subject to third-party copyrights and trademarks. See
[Licenses][policies-licenses] for details.

[rust-foundation]: https://rustfoundation.org/
[trademark-policy]: https://rustfoundation.org/policy/rust-trademark-policy/
[policies-licenses]: https://www.rust-lang.org/policies/licenses
