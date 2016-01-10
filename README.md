which -- locate a program file in the user's path
---

BSD `which` implemented in Rust.

### Usage
*which* [-as] __program__

### Description
The which utility takes a list of command names and searches the path for each executable file that would be run had these commands actu-
ally been invoked.

The following options are available:

-a      List all instances of executables found (instead of just the first one of each).

-s      No output, just return 0 if any of the executables are found, or 1 if none are found.


### Notes

holy crap is there _any_ standardisation between implementations of `which`?
