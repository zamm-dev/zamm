# Images

## Rasterization

Install ImageMagick:

```bash
$ sudo apt install install imagemagick
```

Then use the command mentioned [here](https://superuser.com/a/802627), at the resolution you want:

```bash
$ convert -density 60 input.pdf output.png
```

If you get the error

```bash
$ convert -density 60 input.pdf output.png
convert-im6.q16: attempt to perform an operation not allowed by the security policy `PDF' @ error/constitute.c/IsCoderAuthorized/421.
convert-im6.q16: no images defined `output-raster.pdf' @ error/convert.c/ConvertImageCommand/3229.
```

Then follow the instructions in [this answer](https://stackoverflow.com/a/53180170). First find out what folder you have:

```bash
$ ls -ld /etc/ImageMagick-*
drwxr-xr-x 2 root root 4096 Dec 27 17:00 /etc/ImageMagick-6
```

Then edit `/etc/ImageMagick-6/policy.xml` to add

```xml
  <policy domain="coder" rights="read | write" pattern="PDF" />
```

as recommended.
