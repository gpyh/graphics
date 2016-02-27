let
  projectDirectory = "/home/gpyh/graphics";
in

with import <yarnpkgs>;
stdenv.mkDerivation {
  name = "graphics";
  buildInputs = [ 
    cargoUnstable
  ];
  shellHook = ''
    zsh
  '';
}
