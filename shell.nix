let
  pkgs =
    import <nixpkgs> {};
  rust-toolchain = pkgs.symlinkJoin {
    name = "rust-toolchain";
    paths = with pkgs; [
      rustc
      rustPlatform.rustcSrc
      cargo
      cargo-watch
      rustfmt
    ];
  };
in with pkgs;
mkShell {
  name = "compute 15.0";
  buildInputs = [rust-toolchain];
  RUST_BACKTRACE = 1;

  nativeBuildInputs = with pkgs; [
    cargo
    cargo-watch
  ];
}
