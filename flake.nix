{
  description = "A command line tool for writing scientific documents ";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    rust.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust }:
    utils.lib.eachDefaultSystem (system:
      let
        pname = "mdoc";
        version =
          (builtins.fromTOML
            (builtins.readFile ./Cargo.toml)).package.version;

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust) ];
        };
      in
      {
        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # Rust toolchain
            rust-bin.nightly.latest.default

            # Handy dev tools
            rust-analyzer
            convco

            # Tectonic dependencies
            fontconfig
            graphite2
            harfbuzz
            arcan.harfbuzz
            icu
            libpng
            pkg-config
            openssl
            zlib
          ];
        };
      }
    );
}
