# ANSI BASIC-78

An implementation of BASIC which probably meets the qualifications set forth
in ANSI X3.60-1978. I don't have this document. It's expensive to obtain.
ANSI can go fsck themselves for paywalling this important bit of history.
Fortunately, the goal here isn't to create a certified implementation.

This was created to run programs from the microcomputer editions of
BASIC Computer Games and More BASIC Computer Games.
Every program in those books will run on Microsoft BASIC 4.0.
Every program in those books will also run on BASIC-78.

The specification for BASIC-78 is Microsoft's BASIC-80 Version 5
Reference Manual. Note the 80 refers to Z80 and 8080 CPUs, not the year.

## Differences from BASIC-80

 * Supports UTF-8 in REM statements and strings.
 * String LEN returns the number of characters, not the number of graphemes or bytes.
 * String comparisons with non-ASCII characters are undefined behavior.
 * Files always save and load UTF-8. Tokenized and protected formats not available.
