# j-k-ratio-plus-L

Kotlin to Java compression calculator

## Idea

This thing is supposed to compile Kotlin code, de-compile it to Java and then
compare the character count.

1. compile Kotlin: `kotlinc test.kt`
2. de-compile to Java: `jd-cli TestKt.class`

## Use

```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{"src": "fun main() { print(\"hello world\") }"}' \
  http://localhost:8080/compile
```

