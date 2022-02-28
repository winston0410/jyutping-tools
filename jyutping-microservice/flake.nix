{

  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };
        defaultPackage = naersk-lib.buildPackage {
          root = ../.;
          cargoBuildOptions = x: x ++ [ "-p" "jyutping-microservice" ];
          cargoTestOptions = x: x ++ [ "-p" "jyutping-microservice" ];
        };
        naersk-lib = pkgs.callPackage naersk { };
      in {
        inherit defaultPackage;

        packages = {
          image = pkgs.callPackage ./image.nix { inherit defaultPackage; };
        };

        defaultApp = utils.lib.mkApp { drv = self.defaultPackage."${system}"; };
      });
}
