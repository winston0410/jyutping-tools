{ pkgs, defaultPackage, pythonWithLib, ... }:
let name = "jyutping-microservice";
in (pkgs.dockerTools.buildImage {
  inherit name;
  tag = "latest";
  contents = [ pythonWithLib ];
  config = {
    Cmd = [ "${defaultPackage}/bin/${name}" ];
    Env = [ "PYTHONPATH=${pythonWithLib}/${pythonWithLib.sitePackages}" ];
  };
})
