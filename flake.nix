{
  description = "SMART data exporter for Prometheus using openSeaChest tools";

  inputs = {
    
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ { 
    self,
    nixpkgs,
    flake-parts, 
    naersk,
    fenix,
    ... 
  }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        
      ];

      systems = [ "x86_64-linux" "aarch64-linux" ];
      perSystem = { 
        config, 
        self', 
        inputs', 
        pkgs, 
        system, 
        lib, 
        ... 
      }: 
      let 
        rustToolchain = with fenix.packages.${system}; combine [
          minimal.rustc
          minimal.cargo
        ];
        naerskLib = naersk.lib.${system}.override {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {

        packages.default = naerskLib.buildPackage {
          src = ./.;
        };
      };

      flake = {
        nixosModules.default = import ./nixos {flake = self;};

      };
    };
}
