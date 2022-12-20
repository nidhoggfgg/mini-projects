# passwd generator

generator passwd and no need to remember it!

for example:
```
$ generator --auth OTZ --target facebook
-p-!zvtkctp.g!vy
$ generator --auth OTZ --target google -d 24
bzkvl@i@dk!a_fi.ridyhta!

# same args will generator same passwd
$ generator --auth OTZ --target google -d 24
bzkvl@i@dk!a_fi.ridyhta!
```
so, using this tool, you can never remember the passwd!

just remember the `auth`! pls keep the `auth` safe!

use a prefix or suffix like `google:)` is not good, easy forget, the `auth` is enough.

the python version and rust version are different, the rust version is more officially, the python versino just for fun.
the rust version wouldn't changed its output.

## python

just see the generator.py for more infomation.

## rust

see the source file for more information.

