{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  languages.rust.enable = true;

  git-hooks.hooks.typos.enable = true;
}
