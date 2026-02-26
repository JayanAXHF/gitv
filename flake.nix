{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    supportedSystems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = function:
      nixpkgs.lib.genAttrs
      supportedSystems
      (system: function nixpkgs.legacyPackages.${system});
  in {
    packages = forAllSystems (
      pkgs: let
        default = pkgs.callPackage ./package.nix {};
      in {
        inherit default;

        debug = default.overrideAttrs (finalAttrs: _: {
          cargoBuildType = "debug";
          cargoCheckType = finalAttrs.cargoBuildType;
        });
      }
    );

    devShells = forAllSystems (
      pkgs: {
        default = import ./shell.nix {inherit pkgs;};
      }
    );
  };
}
