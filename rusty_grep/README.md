# Rusty grep

A rusty version of the classic command line tool `grep` (**g**lobally search a
**r**egular **e**xpression and **p**rint). In the simplest use case, grep
searches a specified file for a specified string. To do so, grep takes as its
arguments a filename and a string. Then it reads the file, finds lines in that
file that contain the string argument, and prints those lines.

This grep project will combine Rust concepts of:
- Organizing code (using what you learned about modules)
- Using vectors and strings (collections)
- Handling errors
- Using traits and lifetimes where appropriate
- Writing tests
