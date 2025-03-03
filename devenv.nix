{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [ pkgs.git pkgs.btop ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/git-hooks/
  git-hooks.hooks.shellcheck.enable = true;
  git-hooks.hooks.clippy.enable = true;
  git-hooks.hooks.rustfmt.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
