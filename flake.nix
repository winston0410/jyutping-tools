{
  description = "preview wasm app";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
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
    in {
      devShell.${system} = (({ pkgs, ... }:
        pkgs.mkShell {
          buildInputs = with pkgs; [
            # For compatibility with Vercel
            nodejs-16_x
            pre-commit
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            pythonWithLib
          ];
          DEBUG_PYTHON=1;
          shellHook = ''
            # Setting pre-commit
            pre-commit install
            # Setting NPM
            PATH=$PATH:$(npm bin)
            # Setting Python
            PYTHONPATH=${pythonWithLib}/${pythonWithLib.sitePackages}
            PYO3_PRINT_CONFIG=1;
            echo "Using Nix built Python environment for this project..."
          '';
        }) { inherit pkgs; });
    };
}
