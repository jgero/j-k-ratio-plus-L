# j-k-ratio-plus-L

Kotlin to Java compression calculator

## Idea

This thing is supposed to compile Kotlin code, de-compile it to Java and then
compare the character count.

1. compile Kotlin: `kotlinc test.kt`
2. de-compile to Java: `jd-cli TestKt.class`

## Develop

The nix develop shell has all necessary tools installed. Start the Golang dev
server in it with `dev-server`.

## Container

The container listens on port 4000. You need to mount a temp directory to
"/tmp", the container does not have that by default.

