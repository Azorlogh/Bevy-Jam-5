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
        blenvy = builtins.fetchurl {
          url = "https://github.com/kaosat-dev/Blenvy/releases/download/blenvy_v0.1.0_pre_alpha/blenvy.zip";
          sha256 = "sha256:0f77ps8d2cq7rryz6hsh123ka007an4gy3w6sdyr89fprkwqpyvk";
        };

        bevy-components = builtins.fetchurl {
          url = "https://github.com/kaosat-dev/Blenvy/releases/download/bevy_components_v0.4.2/bevy_components.zip";
          sha256 = "sha256:1fca24v9m1q1zw5rkjb8qan5yjk8dfwjpl1r101zhdf5fdybjgsn";
        };

        gltf-auto-export = builtins.fetchurl {
          url = "https://github.com/kaosat-dev/Blenvy/releases/download/bevy_components_v0.4.2/gltf_auto_export.zip";
          sha256 = "sha256:088nn8zp9xizk2m42wapwv9xz910fxk1xr4j3vgbz3qrmb7cdck9";
        };

        # this is a python script which installs the blenvy addon. I modernized
        # it a bit since the sources are using blender 2.x
        #
        # code taken from here:
        # - https://blender.stackexchange.com/questions/73759/install-addons-in-headless-blender
        # - https://docs.blender.org/api/current/bpy.ops.preferences.html#bpy.ops.preferences.addon_install
        install-script = pkgs.writers.writePython3Bin "install-blenvy" { } ''
          import bpy

          # bc = '${self'.packages.bevy-components}'
          # bpy.ops.preferences.addon_install(filepath=bc)
          # bpy.ops.preferences.addon_enable(module='bevy_components')
          #
          # gae = '${self'.packages.gltf-auto-export}'
          # bpy.ops.preferences.addon_install(filepath=gae)
          # bpy.ops.preferences.addon_enable(module='gltf_auto_export')

          blenvy = '${self'.packages.blenvy}'
          bpy.ops.preferences.addon_install(filepath=blenvy)
          bpy.ops.preferences.addon_enable(module='blenvy')

          bpy.ops.wm.save_userpref()
        '';

        # This is a light wrapper around blender, which basically copies nix
        # blender outputs and additionally runs the addon install script
        # beforehand
        blender = pkgs.writeShellApplication {
          name = "run-blenvy-blender";
          text = ''
            # run install script
            ${lib.getExe pkgs.blender} -b -P ${lib.getExe self'.packages.install-script}

            # regular start exe
            ${lib.getExe pkgs.blender}
          '';
        };
      };
    };
}
