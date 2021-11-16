<!-- ![](https://raw.githubusercontent.com/kmaasrud/doctor/master/docs/out/assets/card_header.png) -->

<h1 align="center">MDoc</h1>

<p align="center"><code>$ cargo install --git https://github.com/kmaasrud/mdoc.git</code></p>

> It is time for academic writing to step out of it's archaic ways and into the modern world. LaTeX - for all it's greatness - is simply too complex and unreadable to be the face of academia. We can do better.

MDoc is a document environment and command line tool for producing academic documents. It's main focus is on user-friendliness; you write your content in Markdown (which, frankly, is a whole lot more enjoyable than writing LaTeX), and MDoc handles the rest. MDoc also comes with a bunch of tools and additions that make your writing experience smoother and A LOT faster.

## Notes on building

Depending on your platform, you might get some build errors when compiling Tectonic. To solve these, you can build using vcpkg to handle the C dependencies. First install `cargo-vcpkg` by running

    cargo install cargo-vcpkg

Then obtain the dependencies with

    cargo vcpkg build

Following this, you need to set a couple of environment variables that makes sure you use vcpkg when compiling. These are the following:

    export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
    export TECTONIC_DEP_BACKEND=vcpkg

If you are building on Windows, you probably have to run with the following environment variable as well:

    export RUSTFLAGS='-Ctarget-feature=+crt-static'
