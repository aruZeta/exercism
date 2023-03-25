{
  description = "Flake with rust";

  inputs = {
    nixpkgs.url = "nixpkgs/c90dcc73273387b152e7d3cf1a50abec00c3204b";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.rustc;
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rust-analyzer
        ];
      };
    };
}
