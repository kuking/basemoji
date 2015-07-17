# basemoji
Happy Emoji Day! - encodes and decodes files in base emoji

😍😍

## Usage

```
$ echo "Hello Emoji" | basemoji -e
😸🚑🚜🚜🚟😐😵🚝🚟🚘🚗🌗
```

then ...

```
$ echo "😸🚑🚜🚜🚟😐😵🚝🚟🚘🚗🌗" | basemoji -d
Hello Emoji
```

you can use files too:

```
$ cat very-serious-file | basemoji -e > slighly-less-serious-file
$ cat slightly-less-serious-file | basemoji -d > very-serious-file
```
