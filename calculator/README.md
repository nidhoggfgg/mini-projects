# simple calculator

[中文](./README_cn.md)

A simple calculator, but very powerful.

Operators support: +-*/()!  
Variable support: global variables, just declare them before use  
Function support: infinite levels of function nesting, deferred declaration, and some built-in functions  
Precision support: 64-bit

```
>>> 12.34 * 45.67 + 6! / 2^4
608.5678
>>> fun f(a b) = a^x + sin(b) * floor(a*b+5)
>>> x = 4
>>> f(ln(PI) E)
5.003415549553682
>>>
```

**note: As a simple project, I can't make it too complicated. I may add more features later in a separate project**
**note: As a simple project, it isn't blanzing fast, but it isn't for large scale computing, over-optimization is not appropriate**
