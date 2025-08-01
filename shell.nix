{ pkgs ? import <nixpkgs-unstable> { }, }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ rustc rustfmt cargo ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
