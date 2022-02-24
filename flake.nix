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

        inherit (pkgs) rustPlatform mkShell stdenv lib;
        inherit (pkgs.darwin.apple_sdk.frameworks) ApplicationServices Cocoa;

        nativeBuildInputs = with pkgs; [ pkg-config ];

        buildDeps = with pkgs; [
          fontconfig
          graphite2
          harfbuzz
          icu
          libpng
          openssl
          zlib
        ] ++ lib.optionals stdenv.isDarwin [
          ApplicationServices
          Cocoa
        ];
      in
      {
        # `nix build`
        defaultPackage = rustPlatform.buildRustPackage {
          inherit nativeBuildInputs pname version;

          src = ./.;
          
          cargoSha256 = "sha256-jzQKjZTB8cgmxrF4ukcZC7nOiz0EpPsiYNT1m+X6idA=";

          buildInputs = buildDeps;

          # Needed to get openssl-sys to use pkg-config
          OPENSSL_NO_VENDOR = 1;
        };

        # `nix develop`
        devShell = mkShell {
          inherit nativeBuildInputs;

          buildInputs = with pkgs; [
            # Rust toolchain
            rust-bin.nightly.latest.default

            # Handy dev tools
            rust-analyzer
            convco
          ] ++ buildDeps;
        };
      }
    );
}
