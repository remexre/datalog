" Vim syntax file
" Language:    Datalog
" Filenames:   *.dl
" Maintainer:  Nathan Ringo <remexre@gmail.com>
" URL:         https://github.com/remexre/datalog/tree/master/misc/vim/syntax/datalog.vim
" Last Change: 2018 Jan 01 - Initial release

if exists("b:current_syntax")
  finish
endif

syn keyword datalogOperator :- , ( )

syn keyword datalogCommentTodo BUG FIXME TODO XXX contained
syn match datalogComment "%.*" contains=datalogCommentTodo

syn match datalogIdent "[a-z0-9-][a-zA-Z0-9_-]*"
syn match datalogVariable "[A-Z_][a-zA-Z0-9_-]*"

syn region datalogString start=+"+ skip=+\\\\\|\\"+ end=+"+ contains=datalogStringEsc,datalogStringHex2,datalogStringHex4,datalogStringHex8,datalogStringEscUnknown
syn match datalogStringEscUnknown "\\." contained
syn match datalogStringEsc "\\[\"\\nrt]" contained
syn match datalogStringHex2 "\\x[0-9a-fA-F]\{2\}" contained
syn match datalogStringHex4 "\\u[0-9a-fA-F]\{4\}" contained
syn match datalogStringHex8 "\\U[0-9a-fA-F]\{8\}" contained

let b:current_syntax = "datalog"

hi link datalogPunctuation Operator

hi link datalogComment Comment
hi link datalogCommentTodo Todo

hi link datalogIdent Constant
hi link datalogVariable Identifier

hi link datalogString String
hi link datalogStringEsc SpecialChar
hi link datalogStringEscUnknown Error
hi link datalogStringHex2 SpecialChar
hi link datalogStringHex4 SpecialChar
hi link datalogStringHex8 SpecialChar
