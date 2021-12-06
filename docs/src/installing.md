For the time being, you will have to compile MDoc yourself to install it. It is published on [crates.io](https://crates.io/crates/mdoc), so this is done easily by running

    $ cargo install mdoc

If you want to compile from upstream, clone [the repo](https://github.com/kmaasrud/mdoc) and install from path with these commands:

    $ git clone https://github.com/kmaasrud/mdoc.git
    $ cd mdoc
    $ cargo install --path .

MDoc depends on [Pandoc](https://pandoc.org/) for parsing the Markdown content. Have a look at [Installing Pandoc](https://pandoc.org/installing.html) and make sure it is available in your `PATH` by running `pandoc -v`.

## Notes on building

Depending on your platform, you might get some [build errors when compiling Tectonic](https://tectonic-typesetting.github.io/book/latest/howto/build-tectonic/cargo-vcpkg-dep-install.html). To solve these, you must clone the repo and build using vcpkg to handle the C dependencies. First install `cargo-vcpkg` by running

    $ cargo install cargo-vcpkg

Then obtain the dependencies with

    $ cargo vcpkg build

Following this, you need to set a couple of environment variables that makes sure you use vcpkg when compiling. These are the following:

    $ export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
    $ export TECTONIC_DEP_BACKEND=vcpkg

If you are building on Windows, you probably have to run with the following environment variable as well:

    $ export RUSTFLAGS='-Ctarget-feature=+crt-static'

After all this, you should be ready to compile normally.
