{ pkgs ? import <nixpkgs> {} }:

let
  rustOverlay = import (builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/refs/heads/master.tar.gz";
  });
  overlayedPkgs = import pkgs.path {
    overlays = [ rustOverlay ];
  };
  mzoonRepo = pkgs.fetchFromGitHub {
    owner = "JoyOfHardware";
    repo = "mzoon_nixos";
    rev = "632f5f7f4eb8517884d32a57d080479a2f0afc4e";
    sha256 = "sha256-1P4kS4FknRf8mgyzaI80jfCyT6+jeWJATfHw4F7D3t0=";
  };
  mzoon = import "${mzoonRepo}/default.nix" {
    inherit pkgs;
    src = mzoonRepo;
  };
  nightly = overlayedPkgs.rust-bin.nightly."2024-06-24".default.override {
      extensions = [ "rust-src" ];
      targets = [ 
      "wasm32-unknown-unknown"
      "x86_64-unknown-linux-gnu"
      ];
  };
in
overlayedPkgs.mkShell {
  buildInputs = [
    nightly
    pkgs.wasm-bindgen-cli
    pkgs.binaryen
    mzoon

  ];

  # shellHook = ''
  #   #export PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1
  #   # Source .env file if it exists
  #   if [ -f .env ]; then
  #     export $(cat .env | xargs)
  #     echo "Environment file loaded."
  #   else
  #     echo "No .env file found. Default environment variables will be used."
  #   fi
  # '';

  
}