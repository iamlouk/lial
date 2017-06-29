# Lial - Lial is a language

Lial is a programming language and interpreter. I have never used Lisp, but after reading its Wikipedia-Page, I decided to write an interpreter for my own lisp-like language. Inspired by [swgillespie/rust-lisp](https://github.com/swgillespie/rust-lisp)

```lisp
; Int:
10 0xFF 42 0b1010 0o123 0

; Real:
0.5 0.42 12.345

; Bool:
true false

; String:
"Hello world!" "\"abc\n...\txyz"

; Nil:
nil

; Map:
{:} { a: (+ 10 12 20 ) hallo: "welt" }

; Lists:
{ 1 2 3 4 "5" { 6 7 } }

; Math:
(+ 1 2 3 (+ 4 5) 6)

; Functions:
(def inc (fn {n} (+ 1 n)))
(echo "meaning of life: " (inc 41))
```

## TODO:
- `let`, `defn`, `do`, ...
- `-`, `*`, `/`, `%`, `hex`, `bin`, `>`, `<`, `=`, `>=`, `<=`, `not`, ...
- write `#[test]`s
