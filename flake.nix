{
  description = "Template for Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix/main-0.5";

    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";

    tauri-plugin-holochain.url =
      "github:darksoil-studio/tauri-plugin-holochain/main-0.5";
    playground.url = "github:darksoil-studio/holochain-playground/main-0.5";
    happ-store.url = "github:darksoil-studio/happ-store";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames inputs.holonix.devShells;
      perSystem = { inputs', config, pkgs, system, ... }: rec {
        devShells.default = pkgs.mkShell {
          inputsFrom = [
            inputs'.tauri-plugin-holochain.devShells.holochainTauriDev
            inputs'.holonix.devShells.default
          ];
          packages = [ pkgs.pnpm inputs'.playground.packages.hc-playground ];
        };
        devShells.androidDev = pkgs.mkShell {
          inputsFrom = [
            inputs'.tauri-plugin-holochain.devShells.holochainTauriAndroidDev
            devShells.default
          ];
          shellHook = ''
            export CARGO_TARGET_DIR=$(pwd)/src-tauri/target/android
          '';
        };
        packages.happ-store = inputs'.happ-store.packages.happ-store_webhapp;
        packages.file-storage-provider =
          inputs'.happ-store.packages.file_storage_provider_aon;
        packages.file-storage-provider_happ =
          inputs'.happ-store.packages.file_storage_provider_happ;
      };
    };
}
