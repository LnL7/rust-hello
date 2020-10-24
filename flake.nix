{
  description = "Flake for hello";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
  let forAllSystems = nixpkgs.lib.genAttrs ["x86_64-linux" "x86_64-darwin"]; in
  {

    packages = forAllSystems (system: {
      hello = nixpkgs.legacyPackages."${system}".callPackage
        ({ stdenv, rustPlatform }:
         rustPlatform.buildRustPackage rec {
           pname = "rust-hello";
           version = "0.1.0";
           src = ./.;
           cargoSha256 = "sha256-aOCi6Q+JKiXU75qpEtrj1jzyPpMYRtAjoSWZLuV2c4s=";
         }) {};
    });

    defaultPackage = forAllSystems (system: self.packages."${system}".hello);

  };
}
