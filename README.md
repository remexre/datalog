# datalog

[![Build Status](https://travis-ci.org/remexre/sparkly-rs.svg?branch=master)](https://travis-ci.org/remexre/sparkly-rs)

A series of interpreters for Datalog.
This is mostly an educational project for myself, although at some point I do want to build a Datalog-based database.

## IDE Support

Vim syntax highlighting is in `misc/vim`.
If you use [vim-plug](https://github.com/junegunn/vim-plug), add a line like:

```vim
Plug 'remexre/datalog', { 'rtp': 'misc/vim' }
```

## TODOs

 - Parse infix arithmetic expressions
   - `"*"(X, 2, Tmp), "+"(Tmp, 1, Y)` is already supported.
   - `(X * 2) + 1 = Y` should be, though.

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
