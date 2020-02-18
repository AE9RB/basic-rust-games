# BASIC-78

An implementation of BASIC which probably meets the qualifications
set forth in ANSI X3.60-1978. I don't have this document.
Despite being superseded and revoked, it's expensive to obtain.
Fortunately, the goal here isn't to create a certified implementation.

This was created to run programs from the microcomputer editions of
BASIC Computer Games and More BASIC Computer Games.
Every program in those books will run on Microsoft BASIC 4 and 5.
Every program in those books will also run on BASIC-78.

## Quick summary of Differences from Microsoft/ANSI BASIC

 * Input is readline. To edit a line, type the line number then TAB.
 * Supports UTF-8 in REM statements and strings.
 * String LEN returns the number of characters, not the number of graphemes or bytes.
 * Files always save and load UTF-8. Tokenized and protected formats not available.
