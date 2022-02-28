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
    in {
      devShell.${system} = (({ pkgs, ... }:
        pkgs.mkShell {
          buildInputs = with pkgs; [
            # For compatibility with Vercel, need nodejs16
            nodejs-16_x
            cargo-watch
            pre-commit
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            cargo-outdated
            # For benchmarking
            gnuplot
            cargo-criterion
          ];
          shellHook = ''
            # Setting pre-commit
            pre-commit install
            # Setting NPM
            echo "Pointing PATH to binaries in NPM"
            export PATH=$PATH:$(npm bin)
            # Updating cargo dep
            cargo outdated
          '';
        }) { inherit pkgs; });
    };
}
