{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };
  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {

      devShell.x86_64-linux = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          rustup
          llvmPackages.bintools-unwrapped
          pkg-config
          alsa-lib
          libudev-zero
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          libxkbcommon
          vulkan-loader
        ];
        shellHook = ''
          export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:"${pkgs.lib.makeLibraryPath buildInputs}"
        '';
      };

    };
}
