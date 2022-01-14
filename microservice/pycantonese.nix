{ pkgs, pylangacq, wordseg, ... }:
pkgs.buildPythonPackage rec {
  pname = "pycantonese";
  version = "3.4.0";

  src = pkgs.fetchPypi {
    inherit pname version;
    sha256 = "8c0768bbfbc9862b9a149525edfd24dc34f380d5d654fae3597da3f0951a0752";
  };

  propagatedBuildInputs = [ pylangacq wordseg ];

  doCheck = false;
}
