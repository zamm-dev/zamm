# Setting up Java

The `asdf` installer for Java is not as convenient, since there is no `install latest` option. Instead, one must list out all the options and then pick one:

```bash
$ asdf plugin add java
$ asdf list-all java | grep ^openjdk-
openjdk-9
openjdk-9.0.1
openjdk-9.0.4
...
openjdk-20.0.1
openjdk-20.0.2
openjdk-21
```

Then

```bash
$ asdf install java openjdk-21
###################################################################### 100.0%
openjdk-21_linux-x64_bin.tar.gz
openjdk-21_linux-x64_bin.tar.gz: OK
$ asdf global java openjdk-21
```

Check it:

```bash
$ which java
/root/.asdf/shims/java
$ java --version
openjdk 21 2023-09-19
OpenJDK Runtime Environment (build 21+35-2513)
OpenJDK 64-Bit Server VM (build 21+35-2513, mixed mode, sharing)
```
