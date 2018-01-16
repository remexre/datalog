# datalog

A series of interpreters for Datalog.

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
