let
  # mozilla rust overlay
  moz_overlay = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  );

  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in

with nixpkgs;

stdenv.mkDerivation {
  name = "bookmarket";

  nativeBuildInput = [
    binutils gcc gnumake openssl pkgconfig # common deps
  ];

  # The packages in the `buildInputs` list will be added to the PATH in our shell
  buildInputs = [
    nixpkgs.latest.rustChannels.nightly.rust # rust
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
  CARGO_HOME = "/tmp/.cargo";

  shellHook = ''
    export PATH="$PATH:$CARGO_HOME/bin:$PIP_PREFIX/bin"
  '';
}
