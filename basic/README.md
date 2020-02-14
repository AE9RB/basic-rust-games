# Extended BASIC 4.1+

An implementation of Altair 8800 Extended BASIC Version 4.1 in Rust.
Based on the reference manual from April 1977.
This will run all programs from the microcomputer editions of
BASIC Computer Games and More BASIC Computer Games.

BASIC was created in a time when lowercase was rare, RAM was expensive,
and CPUs were slow. Lines of code were tokenized with brute force; each
position was simply checked against keywords that would collapse to a
non-inputable character. The tokenized code was then executed by a parsing
interpreter. As you might imagine, error reporting was not very helpful.

This implementation uses modern lexical analysis and an
abstract syntax tree (AST). Lex is run when a line is entered. Parse
is done just in time. Execution is performed on the AST.
This will feel just like real BASIC except with better errors.

Supports UTF-8 in REM statements and strings.
String LEN returns the number of characters, not the number of graphemes or bytes.
Code may be input as lowercase but will LIST as uppercase.
