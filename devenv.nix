{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";
  env.ANDROID_HOME="/Users/frondeus/Library/Android/sdk";
  env.NDK_HOME="/Users/frondeus/Library/Android/sdk/ndk/28.0.12674087";



  dotenv.enable = true;

  # https://devenv.sh/packages/
  packages = with pkgs; [
    libiconv
    jdk17  ];

  # https://devenv.sh/languages/
  # languages.rust.enable = true;

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = '' 
      export PATH="/Users/frondeus/Library/Android/sdk:/Users/frondeus/Library/Android/sdk/tools:/Users/frondeus/Library/Android/sdk/tools/bin:/Users/frondeus/Library/Android/sdk/platform-tools:$PATH"
  '';

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
