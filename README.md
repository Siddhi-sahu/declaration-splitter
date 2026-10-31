# declaration-splitter

A small practice project that uses the [`cssparser`](https://crates.io/crates/cssparser) crate to split a css declaration list into `(name, value, priority)` entries

```
"padding: 20px; color: red !important"
  -> padding | 20px | (none)
  -> color   | red  | important
```

`cssparser` does the actual parsing (tokens, blocks, comments, errors). This project only implements its `DeclarationParser` trait to get the value text back for each declaration

## Why

This is practice for Servo issue [#49261](https://github.com/servo/servo/issues/49261), which will also fixes [#47097](https://github.com/servo/servo/issues/47097). Servo's devtools Rules panel currently builds its list from longhand properties. The planned fix parses the rule's text instead, like Firefox does, so the panel shows what the author wrote: `padding: 20px` stays one line, and `border: 2px solid var(--x)` isn't shown as empty longhands


## todo

- [x] print tokens
- [x] implement the parser traits and get declaration names
- [x] extract the value text
- [x] split `!important` into `priority`
