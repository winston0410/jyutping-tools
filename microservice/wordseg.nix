{ pkgs, ... }:
pkgs.buildPythonPackage rec {
  pname = "wordseg";
  version = "0.0.2";

  src = pkgs.fetchPypi {
    inherit pname version;
    sha256 = "0e23afad01eaa36acb85524e1469a68f65d29bd41e1114cebfe39135e9e951d9";
  };

  # propagatedBuildInputs = [  ];

  doCheck = false;
}
