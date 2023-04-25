with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    cargo
    kotlin
    jd-cli
  ];
}
