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
          perl
          openssl
          zlib
        ] ++ lib.optionals stdenv.isDarwin [
          ApplicationServices
          Cocoa
        ];

        runtimeDeps = with pkgs; [ pandoc ];
      in
      rec {
        # `nix build`
        packages.${pname} = rustPlatform.buildRustPackage {
          inherit nativeBuildInputs pname version;
          buildInputs = buildDeps;
          propagatedBuildInputs = runtimeDeps;
          src = ./.;
          cargoSha256 = "sha256-CGcNTIDcJLPC/XF6DDf1P6Z7tIqY9Z9rXaE8GAUxNFM=";
        };
        defaultPackage = packages.${pname};

        # `nix run`
        apps.${pname} = utils.lib.mkApp {
          drv = packages.${pname};
        };
        defaultApp = apps.${pname};

        # `nix develop`
        devShell = mkShell {
          inherit nativeBuildInputs;

          buildInputs = with pkgs; [
            # Rust toolchain
            rust-bin.nightly.latest.default

            # Handy dev tools
            convco
          ] ++ buildDeps ++ runtimeDeps;
        };
      }
    );
}
