{
  description = "Development environment for the SaBA browser project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-26.05-darwin";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { fenix, nixpkgs, ... }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];

      forEachSystem = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forEachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          rustToolchain = fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-90k41wADPtnhcOpUB/L4dsQLT/N40GT5WS5B5vmYWwc=";
          };
        in
        {
          default = pkgs.mkShell {
            packages = [
              rustToolchain
              fenix.packages.${system}.rust-analyzer
              pkgs.git
              pkgs.gnumake
              pkgs.jq
              pkgs.wget
            ];

            shellHook = ''
              export CARGO_TERM_COLOR=always
              export RUST_BACKTRACE=1
            '';
          };
        }
      );
    };
}
