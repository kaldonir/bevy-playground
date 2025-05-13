{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };
  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {

      devShell.x86_64-linux = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustup
          llvmPackages.bintools-unwrapped
          pkg-config
          alsa-lib
          libudev-zero
          libx11
        ];
      };

    };
}
