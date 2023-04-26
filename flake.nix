{
  description = "Bisky deps";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain =
          pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
            targets = [ "x86_64-unknown-linux-gnu" ];
          };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            exa
            fd
            llvmPackages_latest.bintools
            llvmPackages_latest.lld
            llvmPackages_latest.llvm
            openssl
            pkg-config
            ripgrep
            toolchain
          ];

          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            alias ls=exa
            alias find=fd
            alias grep=ripgrep
          '';
        };
      }
    );
}
