{ pkgs, jyutping-microservice, ... }:
let name = "jyutping-microservice";
in (pkgs.dockerTools.buildImage {
  inherit name;
  tag = "latest";
  contents = with pkgs; [ ];
  config = { Cmd = [ "${jyutping-microservice}/bin/${name}" ]; };
})
