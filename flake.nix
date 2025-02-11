{
  description = "Template for Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix/main-0.4";

    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";

    p2p-shipyard.url = "github:darksoil-studio/p2p-shipyard/main-0.4";
    # happ-store.url = "github:darksoil-studio/happ-store";
    happ-store.url = "/home/guillem/projects/darksoil/happ-store";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames inputs.holonix.devShells;
      perSystem = { inputs', config, pkgs, system, ... }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            inputs'.p2p-shipyard.devShells.holochainTauriDev
            inputs'.holonix.devShells.default
          ];
          packages = [ pkgs.pnpm ];
        };
        devShells.androidDev = pkgs.mkShell {
          inputsFrom = [
            inputs'.p2p-shipyard.devShells.holochainTauriAndroidDev
            inputs'.holonix.devShells.default
          ];
          packages = [ pkgs.pnpm ];
        };
        packages.happ-store = inputs'.happ-store.packages.happ-store_webhapp;
        packages.file-storage-provider =
          inputs'.happ-store.packages.file_storage_provider_aon;
      };
    };
}
