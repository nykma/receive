{
  description = "A minimal HTTP file upload server written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk/pull/391/head";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, flake-utils, naersk, nixpkgs, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        rustToolchain = with fenix.packages.${system};
          combine (with stable; [ clippy rustc cargo rustfmt rust-src ]);

        naersk' = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        built = naersk'.buildPackage {
          pname = "receive";
          version = "0.1.0";
          src = self;
        };
      in
      {
        packages = {
          receive = pkgs.runCommand "receive" { } ''
            mkdir -p $out/bin
            cp ${built}/bin/receive $out/bin/receive
          '';
          serve = pkgs.runCommand "serve" { } ''
            mkdir -p $out/bin
            cp ${built}/bin/serve $out/bin/serve
          '';
          default = pkgs.symlinkJoin {
            name = "receive-all";
            paths = [ self.packages.${system}.receive self.packages.${system}.serve ];
          };
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            openssl
            pkg-config
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer
            nixfmt
          ];

          env = {
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };
        };

        formatter = pkgs.nixfmt;
      }
    );
}
