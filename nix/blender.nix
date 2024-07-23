{
  perSystem =
    {
      self',
      pkgs,
      lib,
      ...
    }:
    {
      devShells.blender = pkgs.mkShell { packages = [ self'.packages.blender ]; };
      packages = {

        # this fetches the plugin
        blenvy-raw = builtins.fetchurl {
          url = "https://github.com/kaosat-dev/Blenvy/releases/download/blenvy_v0.1.0_pre_alpha/blenvy.zip";
          sha256 = "sha256:0f77ps8d2cq7rryz6hsh123ka007an4gy3w6sdyr89fprkwqpyvk";
        };

        # this is a python script which installs the blenvy addon. I modernized
        # it a bit since the sources are using blender 2.x
        #
        # code taken from here:
        # - https://blender.stackexchange.com/questions/73759/install-addons-in-headless-blender
        # - https://docs.blender.org/api/current/bpy.ops.preferences.html#bpy.ops.preferences.addon_install
        install-script = pkgs.writers.writePython3Bin "install-blenvy" { } ''
          import bpy

          path = '${self'.packages.blenvy-raw}'

          bpy.ops.preferences.addon_install(filepath=path)
          bpy.ops.preferences.addon_enable(module='blenvy')
          bpy.ops.wm.save_userpref()
        '';

        # This is a light wrapper around blender, which basically copies nix
        # blender outputs and additionally runs the addon install script
        # beforehand
        blender = pkgs.runCommand "blender" { } ''

          # copy blender outputs
          mkdir -p $out
          cp -rf ${pkgs.blender}/* $out/

          # run install script
          $out/bin/blender -b -P ${self'.packages.install-script}
        '';
      };
    };
}
