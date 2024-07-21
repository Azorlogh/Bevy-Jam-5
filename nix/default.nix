{
  imports = [ ./rust.nix ];

  perSystem =
    { self', pkgs, ... }:
    {
      formatter = pkgs.nixfmt-rfc-style;
      devShells.default = self'.devShells.rust;
    };
}
