{ pkgs ? import (fetchTarball {
  url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/24.05.tar.gz";
}) {}, src ? ./.}:

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
  pkgs.stdenv.mkDerivation (self: {
    pname = "joyofhardware";
    version = "0.1.0";

    cargoDeps = pkgs.rustPlatform.importCargoLock {
      lockFile = ./Cargo.lock;
      outputHashes = {
       "color_macro-0.1.0" = "sha256-Um7W1JA5Jd0shegXBQ6A8SE+dbKm+sxz+HAAfduMX5o=";
     };
    };

    src = src;

    nativeBuildInputs = [
      nightly
      pkgs.wasm-bindgen-cli
      pkgs.binaryen
      mzoon
    ];

    buildPhase = ''
      mzoon build -r
    '';

    installPhase = ''
      mkdir -p $out/bin
      cp ./target/release/backend $out/bin/app
      cp -rf frontend $out/bin/
      cp -rf public $out/bin/
    '';

    buildInputs = [
      pkgs.rustPlatform.cargoSetupHook
    ];
  })