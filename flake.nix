{
  description = "Daily challenges for RustLangES";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    wrangler = {
      # Use 4.19.1
      url = "github:ryand56/wrangler/1141a859c59e05ceb901d14790f0f75a6c5de3f5";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    {
      nix.settings = {
        substituters = ["https://wrangler.cachix.org"];
        trusted-public-keys = ["wrangler.cachix.org-1:N/FIcG2qBQcolSpklb2IMDbsfjZKWg+ctxx0mSMXdSs="];
      };
    }
    // flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system:
        import ./. rec {
          inherit system;
          pkgs = nixpkgs.legacyPackages.${system};
          crane = inputs.crane.mkLib pkgs;
          fenix = inputs.fenix.packages.${system};
          wrangler-fix = inputs.wrangler.packages.${system};
        }
    );
}
