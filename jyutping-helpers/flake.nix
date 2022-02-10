{
  outputs = { self, nixpkgs }:
    let system = "x86_64-linux";
    in {
      devShell.${system} = (({ pkgs, ... }:
        pkgs.mkShell {
          buildInputs = with pkgs; [ nodejs-16_x pre-commit];

          shellHook = ''
          '';
        }) { pkgs = nixpkgs.legacyPackages.${system}; });
    };
}
