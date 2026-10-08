{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      libs = with pkgs; [ raylib libGL xorg.libX11 xorg.libXrandr xorg.libXinerama xorg.libXcursor xorg.libXi ];
    in {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = [ pkgs.pkg-config pkgs.clang pkgs.gnumake pkgs.rustc];
        buildInputs = libs;
        LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
      };
    };
}
