{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    gcc
    openssl.dev
    libpkgconf
    pkg-config
    xorg.libxcb.dev
    wayland
    # dbus.lib
    # dbus.dev
    dbus
    glib
    gtkd
    libsoup_3
    webkitgtk_4_1
  ];

  # services.dbus.enabled = true;

  # env.RUSTFLAGS = lib.mkForce "-C link-args=-Wl,-fuse-ld=mold,-rpath,${with pkgs;
  #   lib.makeLibraryPath [
  #     libGL
  #     libxkbcommon
  #     wayland
  #     xorg.libX11
  #     xorg.libXcursor
  #     xorg.libXi
  #     xorg.libXrandr
  #     # dbus.lib
  #     # dbus.dev
  #     dbus
  #   ]}";

  env.RUST_BACKTRACE = "1";
  env.__NV_PRIME_RENDER_OFFLOAD = "1";
  env.WEBKIT_DISABLE_COMPOSITING_MODE = "1";
  # env.RUSTFLAGS = "-C link-args=-Wl,-fuse-ld=mold,-rpath,$(devenv makeLibraryPath pkgs.libGL)";

  # -C link-arg=-fuse-ld=mold

  # env.LD_LIBRARY_PATH = with pkgs;
  #   lib.makeLibraryPath [
  #     libGL
  #     libxkbcommon
  #     wayland
  #     xorg.libX11
  #     xorg.libXcursor
  #     xorg.libXi
  #     xorg.libXrandr
  #   ];
  # ...
  # LD_LIBRARY_PATH = libPath;

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  enterShell = ''
    hello
    git --version
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
