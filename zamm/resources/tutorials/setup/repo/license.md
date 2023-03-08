# Setting the MIT license for your project

Every open source software project needs a license before any company will legally touch it. The MIT license is a good default if you want anyone to freely use your code. So, if you are given a task such as:

> Set this project to be MIT licensed

then you can do so by following these steps:

The very first command we run is to find out who this software is supposed to be copyrighted by.

```bash
$ git config --global user.name
Amos Jun-yeung Ng
```

Make note of the name being output here -- "Amos Jun-yeung Ng" -- because we will now put it into the MIT license. Create the file `LICENSE` in the current directory (i.e. `./LICENSE`) with the contents:

```
MIT License

Copyright (c) 2023 Amos Jun-yeung Ng

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

That's all! **Don't take any more steps** because the task is now done!