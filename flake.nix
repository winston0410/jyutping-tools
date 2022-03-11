{
  description = "preview wasm app";

  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { nixpkgs, rust-overlay, utils, naersk, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };

        #REF https://github.com/nix-community/naersk/issues/154
        # Has to put all derivations on the top-level flake
        jyutping-microservice = naersk-lib.buildPackage {
          root = ./.;
          cargoBuildOptions = x: x ++ [ "-p" "jyutping-microservice" ];
          cargoTestOptions = x: x ++ [ "-p" "jyutping-microservice" ];
        };
        naersk-lib = pkgs.callPackage naersk { };
      in {
        packages = {
          inherit jyutping-microservice;
          jyutping-microservice-image =
            pkgs.callPackage ./jyutping-microservice/image.nix {
              inherit jyutping-microservice;
            };
          # kratos-config = pkgs.writeText "kratos.yml"
            # (builtins.readFile ./identity/kratos.yml);
          # identity-schema = pkgs.writeText "identity."
            # (builtins.readFile ./identity/kratos.yml);
        };
        devShell = (({ pkgs, ... }:
          pkgs.mkShell {
            buildInputs = with pkgs; [
              # For compatibility with Vercel, need nodejs16
              nodejs-16_x
              cargo-watch
              pre-commit
              (rust-bin.selectLatestNightlyWith (toolchain:
                toolchain.default.override {
                  targets = [ "wasm32-unknown-unknown" ];
                }))
              cargo-outdated
              # For benchmarking
              gnuplot
              cargo-criterion
              wasm-pack
            ];
            shellHook = ''
              # Disable pre-commit for now
              # Setting pre-commit
              # pre-commit install
              # Setting NPM
              echo "Pointing PATH to binaries in NPM"
              export PATH=$PATH:$(npm bin)
              # Updating cargo dep
              cargo update
            '';
          }) { inherit pkgs; });
      });
}
