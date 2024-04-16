{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup pkgs.pkg-config pkgs.fontconfig
  ] ++ (if pkgs.stdenv.targetPlatform.isDarwin then [
    pkgs.libiconv
    pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
    pkgs.darwin.apple_sdk.frameworks.CoreText
  ] else []);

  OPENSSL_DEV=pkgs.openssl.dev;
}
