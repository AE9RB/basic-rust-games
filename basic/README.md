# BASIC

This unfinished project is a BASIC language interpreter.
The dialect is what's needed to run programs from the microcomputer
editions of BASIC Computer Games and More BASIC Computer Games.

BASIC was created as an ASCII language in a time when lowercase was rare.
This implementation supports UTF-8 in REM statements and quoted strings.
String LEN returns the number of characters, not the number of graphemes.
Code may be input as lowercase but will LIST as uppercase.
Variables names can be any length and will not be truncated.
