{inputs, ...}: {
  perSystem = {
    self',
    pkgs,
    system,
    ...
  }: let
    fnx = inputs.fenix.packages.${system};
  in {
    packages.rust = fnx.combine [
      fnx.stable.cargo
      fnx.stable.clippy
      fnx.stable.rust-analyzer
      fnx.stable.rust-src
      fnx.stable.rustc

      # it's generally recommended to use nightly rustfmt
      fnx.complete.rustfmt

      fnx.targets.wasm32-unknown-unknown.stable.rust-std
    ];

    devShells.rust = let
      # required for linking bevy
      extraPackages = [
        # general
        pkgs.pkg-config
        pkgs.udev
        pkgs.alsaLib
        pkgs.vulkan-loader
        pkgs.openssl

        # wayland
        pkgs.wayland
        pkgs.libxkbcommon

        # X
        pkgs.xorg.libX11
        pkgs.xorg.libXrandr
        pkgs.xorg.libXcursor
        pkgs.xorg.libXi
      ];
    in
      pkgs.mkShell {
        name = "Rust Shell";
        packages = extraPackages ++ [self'.packages.rust];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath extraPackages;
      };
  };
}
