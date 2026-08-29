{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    cargo-nextest
    cargo-deny
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "nightly";
    mold.enable = true;
    components = [
      "rustc"
      "cargo"
      "rust-analyzer"
      "rustfmt"
      "clippy"
    ];
  };

  git-hooks = {
    settings.rust.cargoManifestPath = "temp-rs/Cargo.toml";
    hooks = {
      typos.enable = true;
      clippy.enable = true;
    };
  };

  scripts.watcher = {
    exec = ''
      watchexec -c -e rs \
      "cargo clippy && cargo nextest run"
    '';
    packages = [ pkgs.watchexec ];
  };

  # See full reference at https://devenv.sh/reference/options/
}
