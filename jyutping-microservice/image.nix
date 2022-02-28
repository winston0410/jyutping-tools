{ pkgs, defaultPackage, ... }:
let name = "jyutping-microservice";
in (pkgs.dockerTools.buildImage {
  inherit name;
  tag = "latest";
  contents = with pkgs; [  ];
  config = {
    Cmd = [ "${defaultPackage}/bin/${name}" ];
  };
})
