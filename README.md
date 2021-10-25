![](https://raw.githubusercontent.com/kmaasrud/doctor/master/docs/out/assets/card_header.png)

<h2 align="center">A blazingly fast document builder that makes scientific writing a breeze</h2>

<p align="center"><a href="https://www.kmaasrud.com/doctor/installing">Installation</a> - <a href="https://kmaasrud.com/doctor/creating-your-first-document">Getting started</a> - <a href="https://www.kmaasrud.com/doctor/docs">Documentation</a></p>

Doctor is a document builder and command line tool that uses [Pandoc](https://pandoc.org) coupled with a TeX engine to produce high-quality documents. It's main focus is on user-friendliness; you write your content in Markdown (which, frankly, is a whole lot more enjoyable than writing LaTeX), and Doctor handles the rest. Doctor also comes with a bunch of tools and additions that make your writing experience smoother and A LOT faster.

It is time for academic writing to step out of it's archaic ways and into the modern world. LaTeX - for all it's greatness - is simply too complex and unreadable to be the face of academia. We can do better.

## Notes on building

If you get build errors, you can build using vcpkg. First install `cargo-vcpkg` by running

    cargo install cargo-vcpkg

Then obtain the dependencies with

    cargo vcpkg build

Following this, you need to set a couple of environment variables that makes sure you use vcpkg when compiling. These are the following:

    export VCPKG_ROOT="${CARGO_TARGET_DIR:-$(pwd)/target}/vcpkg"
    export TECTONIC_DEP_BACKEND=vcpkg

If you are building on Windows, you probably have to run with the following environment variable as well:

    export RUSTFLAGS='-Ctarget-feature=+crt-static'
