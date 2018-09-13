#### Some fixes

- in compiling shared library `xed.dll`, one may need to modify the encoding in `mbuild/mbuild/base.py:bytes2unicode` from `utf-8` to `latin-1`
