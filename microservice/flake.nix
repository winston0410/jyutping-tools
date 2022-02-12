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
        python = pkgs.python310;
        wordseg = pkgs.callPackage ./wordseg.nix { pkgs = python.pkgs; };
        pylangacq = pkgs.callPackage ./pylangacq.nix { pkgs = python.pkgs; };
        pycantonese = pkgs.callPackage ./pycantonese.nix {
          pkgs = python.pkgs;
          inherit pylangacq wordseg;
        };
        pythonWithLib = python.withPackages (p: with p; [ pycantonese ]);
        defaultPackage = naersk-lib.buildPackage {
          root = ./.;
          buildInputs = with pkgs; [ pythonWithLib ];
        };
        naersk-lib = pkgs.callPackage naersk { };
      in {
        inherit defaultPackage;

        packages = {
          image = pkgs.callPackage ./image.nix {
            inherit defaultPackage pythonWithLib;
          };
        };

        defaultApp = utils.lib.mkApp { drv = self.defaultPackage."${system}"; };
      });
}
