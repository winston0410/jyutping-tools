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
          #NOTE Save two files in two different derivations, as it is likely to reuse kratos.yml
          kratos-config = pkgs.writeText "kratos.yml"
            (builtins.readFile ./identity/kratos.yml);
          identity-schema = pkgs.writeText "identity.schema.json"
            (builtins.readFile ./identity/identity.schema.json);
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
              # For benchmarking
              gnuplot
              cargo-criterion
              wasm-pack
              # For starting ory kratos for development
              podman-compose
              # Investigate paperclip
              # https://github.com/paperclip-rs/paperclip
              # For wordshk_tool
              openssl
              pkgconfig
              # For testing postprocessing data with Github Action
              deno
            ];
            shellHook = ''
              # Disable pre-commit for now
              # Setting pre-commit
              # pre-commit install
              # Setting NPM
              echo "Pointing PATH to binaries in NPM..."
              export PATH=$PATH:$(npm bin)
              cargo update
              echo "Starting Ory Kratos..."
              netstat -tulpn | rg 4433 || podman-compose up --build --force-recreate
              echo "All done!"
            '';
          }) { inherit pkgs; });
      });
}
