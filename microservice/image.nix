{ pkgs, defaultPackage, pythonWithLib, ... }:
let name = "jyutping-microservice";
in (pkgs.dockerTools.buildImage {
  inherit name;
  tag = "latest";
  contents = with pkgs; [ pythonWithLib ];
  #NOTE For debug
  # contents = with pkgs; [ pythonWithLib bash coreutils ];
  config = {
    Env = [ "PYTHONPATH=${pythonWithLib}/${pythonWithLib.sitePackages}" ];
    Cmd = [ "${defaultPackage}/bin/${name}" ];
  };
})
