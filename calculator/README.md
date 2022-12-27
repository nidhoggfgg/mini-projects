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

and it support plot:
![plot.png](../images/calc-magic-plot2d.png)
you can use these code to see them:
```
fun f(x) = -1/(1+E^(-0.05*x+6)) * 80 + 80
%plot2d(f, 0, 240, 0.1)

fun g(x) = -sin(to_rad(10*x)) * 10 + 15
%plot2d(g, 0, 180, 0.1)

fun t(x) = -((x-25)^3-400*(x-25))+5625
fun h(x) = t(x)/100
%plot2d(h, 0, 50, 0.01)
```

**note: As a simple project, I can't make it too complicated. I may add more features later in a separate project**
**note: As a simple project, it isn't blanzing fast. it isn't for large scale computing, over-optimization is not appropriate**

