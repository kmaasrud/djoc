+++
title = "Installing"

[menu.main]
name = "Installing"
weight = 1
+++

MDoc depends on [Pandoc](https://pandoc.org/) for parsing the Markdown content, so you have to make sure it is installed first. Have a look at [Installing Pandoc](https://pandoc.org/installing.html) and make sure Pandoc is available in your `PATH` by running `pandoc -v`.

## Pre-built binaries

There are a range of pre-built binaries available on the [GitHub releases page](https://github.com/kmaasrud/mdoc/releases). Simply download the correct archive for your platform, extract it and move the binary to somewhere in your `PATH`. The binary is self-contained, so you don't have to do anything else.

## Package managers

MDoc can be installed with the following package managers:

| Package manager | Install command |
| :--- | :--- |
| [Cargo](https://crates.io/crates/mdoc) | `cargo install mdoc` |

## Building from source

If you want to compile from upstream, clone [the repo](https://github.com/kmaasrud/mdoc) and install from path with these commands:

```
$ git clone https://github.com/kmaasrud/mdoc.git
$ cd mdoc
$ cargo install --path .
```

Tectonic needs a few [third party C dependencies](https://tectonic-typesetting.github.io/book/latest/howto/build-tectonic/index.html#third-party-dependencies) when building. Depending on your platform, you might not have them available. You can either install them manually, or you can try one of the methods explained below.

### Using Nix

For Unix-based systems (or Windows with WSL,) this method is definitely recommended.

1. Install Nix by following the instructions for your platform [here](https://nixos.org/download.html).
2. Clone the MDoc repo.

    ```
    $ git clone https://github.com/kmaasrud/mdoc.git
    $ cd mdoc
    ```

3. Run the following command:

    ```
    $ nix --experimental-features "nix-command flakes" develop
    ```

You will now get placed in a shell with all the required dependencies installed. Running `cargo build --release` should compile MDoc for you, and you can find the binary in `target/release`.

> 📖 All dependencies are compiled statically into the binary with the exception of Pandoc. This means you can safely exit the Nix shell and run your compiled MDoc, as long as you have Pandoc installed on your system.

### Using VCPKG

A fully cross-platform solution is to use VCPKG to handle the dependencies. First install `cargo-vcpkg` by running

```
$ cargo install cargo-vcpkg
```

Then obtain the dependencies with

```
$ cargo vcpkg build
```

Following this, you need to set a couple of environment variables that makes sure you use vcpkg when compiling. These are the following:

```
$ export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
$ export TECTONIC_DEP_BACKEND=vcpkg
```

If you are building on Windows, you have to run with the following environment variable exported as well:

```
$ export RUSTFLAGS='-Ctarget-feature=+crt-static'
```

After all this, you should be ready to compile normally.
