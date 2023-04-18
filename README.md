# Raw glue

[![CI](https://github.com/mrkkrp/raw-glue/actions/workflows/ci.yaml/badge.svg)](https://github.com/mrkkrp/raw-glue/actions/workflows/ci.yaml)

This is a command line utility that allows us to stitch multiple RAW files
together in a completely automatic manner. It is designed to be useful for
stitching of overlapping shots of a film negative done with a digital
camera + macro lens on a copy stand. For that reason it assumes rectilinear
projection and extremely narrow field of view.

Here is an example of invocation:

``` console
$ raw-glue photo1.cr3 photo2.cr3 …
```

The source images can be in any format supported by [`libraw`][libraw].

The result will be saved in [the TIFF format][tiff] in the current working
directory named according to the current time, e.g.:

```
20230417213934004508157.tiff
```

Raw glue is written in Rust and uses [Hugin][hugin] under the hood.

[libraw]: https://www.libraw.org/
[tiff]: https://en.wikipedia.org/wiki/TIFF
[hugin]: https://hugin.sourceforge.io/

## Building

You are going to need [Nix][nix]. Once Nix is installed building is as
simple as:

```console
$ nix build
```

The resulting executable can be found in `./result/bin`.

[nix]: https://nixos.org/

## License

Copyright 2023–present Mark Karpov

Distributed under the MIT license.
