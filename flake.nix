{
  description = "Modern document writing with Djot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    crane,
    flake-utils,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };

      toolchain = with fenix.packages.${system};
        combine [
          default.rustc
          default.cargo
          default.clippy
          default.rustfmt
        ];

      craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

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

      filter = path: type:
        (builtins.match ".*css$" path != null) || (craneLib.filterCargoSources path type);

      djoc = craneLib.buildPackage {
        inherit buildInputs;
        src = pkgs.lib.cleanSourceWith {
          inherit filter;
          src = craneLib.path ./.;
        };
      };
    in rec {
      packages.default = djoc;

      apps.default = flake-utils.lib.mkApp {
        drv = packages.default;
      };

      devShells.default = pkgs.mkShell {
        inherit buildInputs;
        nativeBuildInputs = [toolchain];
      };
    });
}
