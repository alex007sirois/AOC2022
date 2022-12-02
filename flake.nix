{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          (
            rust-bin.selectLatestNightlyWith (
              toolchain: toolchain.default.override {
                extensions = [ "rust-src" ];
              }
            )
          )
        ];
      };
    };
}


