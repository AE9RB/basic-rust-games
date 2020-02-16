# ANSI BASIC-78

An implementation of BASIC which meets the qualifications set forth
in ANSI X3.60-1978. This was created to run programs from the microcomputer
editions of BASIC Computer Games and More BASIC Computer Games.
With that goal in mind, it makes sense to use documentation for Microsoft
BASIC-80 Version 5 as the specification instead of the actual ANSI documents.

## Differences from BASIC-80

 * Code may be input as lowercase but will LIST as uppercase.
 * Supports UTF-8 in REM statements and strings.
 * String LEN returns the number of characters, not the number of graphemes or bytes.
 * String comparisons with non-ASCII characters are undefined behavior.
 * Files always save and load UTF-8. Tokenized and protected formats not available.
