{
  description = "Modern document writing with Djot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rust = pkgs.rust-bin.stable.latest.default;
      craneLib = (crane.mkLib pkgs).overrideToolchain rust;

      buildInputs = with pkgs;
        [
          fontconfig
          graphite2
          harfbuzz
          icu
          libpng
          perl
          pkg-config
          openssl
          zlib
        ]
        ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.ApplicationServices
          darwin.apple_sdk.frameworks.Cocoa
          libiconv
        ];

      djoc = craneLib.buildPackage {
        inherit buildInputs;
        src = craneLib.cleanCargoSource ./.;
      };
    in rec {
      packages.default = djoc;

      apps.default = flake-utils.lib.mkApp {
        drv = packages.default;
      };

      devShells.default = pkgs.mkShell {
        inherit buildInputs;
        nativeBuildInputs = with pkgs; [rust];
      };
    });
}
