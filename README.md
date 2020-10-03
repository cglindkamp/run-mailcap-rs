run-mailcap-rs [![Build Status](https://api.travis-ci.com/cglindkamp/run-mailcap-rs.svg?branch=master)](https://travis-ci.com/cglindkamp/run-mailcap-rs)
==============
This is rewrite of run-mailcap in Rust.

Why a rewrite?
--------------
~~Run-mailcap seems to be unmaintained at least on Gentoo, so you either have an
old non-working install or non at all there.~~ Apart from that, I wanted to learn
Rust, and run-mailcap is simple enough to get it done without in-depth knowledge
of the language, but also not trivial either.

Why not using xdg-open?
-----------------------
Run-mailcap has multiple advantages over xdg-open. While xdg-open is easier to
extend for the application writers and distributions itself (just add an
application desktop file with each application), this is not so easy done by
the user. A mailcap file on the other hand is simple text file. Just add a new
line for the mime type, you want to customize, and your done.

In addition, you can have different actions with run-mailcap, e.g. open for
viewing or editing or just print the file without looking at it. In principle,
you can have the same with desktop files in a desktop environment, but there is
just one default, that xdg-open uses. You can add even more flexibility with
mailcap files by making entries context dependend ("test" command value in the
entry).

Last but not least, some command line mail client (e.g. mutt) already use
mailcap files, so if you use one of them and xdg-open, you had to maintain
configuration for both programs.

Status
------
- all command line arguments of the original run-mailcap are implemented
- the following actions are implemented: view, see (same as view), cat (same as
  view, but only handle entries with copiousoutput and don't use a pager),
  edit, change (same es edit), compose, create (same as compose) and print
- actions can also be determined by the name, the program was called; in addtion
  to the names recognized by run-mailcap, all names are also recognized with an
  "-rs" suffix, so run-mailcap and run-mailcap-rs can live side by side.
- replacement of the filename (%s) and mimetype (%t) in commands is supported;
  other replacements are not
- only a single file with optional mime type can be given to run-mailcap-rs,
  without encoding specified

Installation
------------
To install, just clone this repository, enter the cloned directory and issue
the following commands:
```
cargo install
```

The binary is installed to ~/.cargo/bin by default. The alternative symlinks
have to be created manually.

It you want to install it system wide, including the corresponding symlinks,
issue the following commands:

```
make
sudo make install
```

The binary and the symlinks are installed to /usr/local/bin by default. To get
compatibility symlinks (as if you had the original run-mailcap installed), add
"COMPAT\_LINKS=1" during install like that:
```
sudo make COMPAT_LINKS=1 install
```
