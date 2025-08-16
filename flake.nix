{
  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libPath = with pkgs; lib.makeLibraryPath [
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
          libxkbcommon libGL wayland vulkan-loader
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            alsa-lib
            pkg-config
            bacon
            wasm-bindgen-cli
            python3
            (
              rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                ];
                targets = [ "wasm32-unknown-unknown" ];
              })
            )
          ];
          LD_LIBRARY_PATH = libPath;
          RUST_BACKTRACE = 1;
          shellHook = ''
            # wasm-bindgen --target web --out-dir ./pkg target/wasm32-unknown-unknown/debug/wgpu_tutorial.wasm;
          '';
        };
      }
    );
}
