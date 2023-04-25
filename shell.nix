with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    kotlin
    jd-cli
  ];
}
