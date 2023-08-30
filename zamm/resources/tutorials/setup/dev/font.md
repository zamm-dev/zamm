# Installing a custom font

Follow the instructions [here](https://askubuntu.com/a/620104). Run this command in the directory where you have the fonts you want to install:

```bash
$ unzip "*.zip" "*.ttf" "*.otf" -d ${HOME}/.fonts
$ sudo fc-cache -f -v
```
