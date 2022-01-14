{ pkgs, ... }:
pkgs.buildPythonPackage rec {
  pname = "pylangacq";
  version = "0.16.0";

  src = pkgs.fetchPypi {
    inherit pname version;
    sha256 = "5e435cee8b4f30b86580666e2e8c8273d638783199f296a363bfe8c21b80d215";
  };

  propagatedBuildInputs = with pkgs; [ python-dateutil requests tabulate wcwidth];

  doCheck = false;
}
