# gitrev

Gitrev is a simple Windows utility written in Rust 1.4 (stable) that is
(ultimately) meant to serve as a replacement for `SubWCRev.exe` for Git-based
repositories. See
[here](http://tortoisesvn.net/docs/release/TortoiseSVN_en/tsvn-subwcrev.html)
for the documentation of `SubWCRev.exe`.

## Usage

The utility requires two positional arguments, `template-file` and
`output-file`. The former should be a template file that gitrev will
insert git build-specific data into, and the latter should be the name of the
desired output file.

For example, if the template file is called
`VersionInfo.tmpl` and the output should be stored in `VersionInfo.h`, the
utility should be invoked as follows:

```
gitrev.exe VersionInfo.tmpl VersionInfo.h
```

## Building and testing

To build the utility, run in the command line:

```
$ cargo build
```

To test it, run:

```
$ cargo test
```

## License

Standard MIT license applies.
