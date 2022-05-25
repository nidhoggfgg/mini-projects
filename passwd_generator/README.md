# passwd generator

generator passwd and no need to remember it!

for example:
```
# every time give same auth and same target will return same passwd
$ generator -un --auth OTZ --target google
MU_:o8kNYO:lI5Lz
$ generator -un --auth OTZ --target google
MU_:o8kNYO:lI5Lz

# changed target will return different passwd
$ generator -un --auth OTZ --target facebook
aEeD_pdt9Xslh1Rp
```
so, using this tool, you can never remember the passwd!

just remember the `auth`! pls keep the `auth` safe!

## python

just see the generator.py for more infomation.