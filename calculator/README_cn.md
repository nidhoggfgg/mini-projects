# 计算器 (Rust)

一个简单的计算器，但是很有意思，总代码量 1000 行左右（包含绘图部分）。

```
>>> 12.34 * 45.67 + 6! / 2^4 - 1.2+3*(1-2)
604.3678
>>> fun f(a b) = a^x + sin(b) * floor(a*b+5)
>>> x = 4
>>> f(ln(PI) E)
5.003415549553682
>>> fun g(x) = -sin(to_rad(10*x)) * 10 + 15
>>> %plot2d(g, 0, 72, 0.1)
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢀⡴⠒⠲⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠒⠲⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⣰⠋⠀⠀⠀⠈⢳⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠋⠀⠀⠀⠈⢳⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡴⠁⠀⠀⠀⠀⠀⠀⠱⡄⠀⠀⠀⠀⠀⠀⠀⠀⡴⠁⠀⠀⠀⠀⠀⠀⠹⡄⠀⠀⠀⠀⠀⠀⠀⠀⡀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣄⠀⠀⠀⠀⠀⢀⡼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠙⣄⠀⠀⠀⠀⠀⢀⡼⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢦⡀⠀⠀⣠⠞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢦⡀⠀⠀⣠⠞⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠒⠚⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠒⠚⠁⠀
```
## 四则运算及括号乘方阶乘

支持四则运算括号乘方阶乘，所有的运算规则及优先级和数学上的一致
```
>>> 12.34 * 45.67 + 6! / 2^4 - 1.2+3*(1-2)
604.3678
```
对于指数及底数中的负号，见如下例子：
```
>>> -9^-2 # 等价于 -(9^(-2))
-0.012345679012345678
```

## 变量

支持自定义变量，除了函数内的变量，其他变量必须在声明时赋值。  
同名变量则后声明的变量覆盖前面声明的变量。  
赋值运算可以使用任何表达式（四则运算、函数调用等等）
```
>>> x=ln(E^2)+ln(E^3)
>>> y=ln(E)+ln(E^4)
>>> y/x
1
>>> y=ln(E^3)+ln(E^7)
>>> y/x
2
```
内置的全局变量有：`E` 也就是自然对数底，`PI` 也就是圆周率。  
对于这两个变量，是可以被覆盖的，覆盖之后就是用户设置的值。

## 函数

函数是这个计算器的一大特点之一。  
可以认为，所有的用户编写的函数都是闭包，也就是可以使用外部变量。
甚至可以使用未定义的变量，只要求在调用之前定义即可。
```
>>> fun f(x y)=a^x+y
>>> f(1 2)
can't find variable named 'a'
>>> a=2
>>> f(1 2)
4
```
甚至对于函数内和参数中都可以使用函数调用，一定程度上支持了数学中的复合函数。
```
>>> fun f(x y)=a^x+y
>>> a=2
>>> fun g(x)=f(2 x)+3
>>> g(f(5 6))
45
```
不过很不幸的是，很容易就可以构造出一个无限递归的函数：
```
>>> fun g(x)=1
>>> fun f(x)=g(x)
>>> fun g(x)=f(x) # 危险！g(x)=g(x)
>>> f(1)

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
[1]    2178 IOT instruction  cargo run
```
或者使用内置函数：
```
>>> fun ln(x y)=ln(1)+ln(2)
>>> ln(1 2)

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
[1]    2337 IOT instruction  cargo run
```
最离谱的是可以通过定义自身来无限递归：
```
>>> fun f(x) = f(x)
>>> f(1)

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
[1]    962 IOT instruction  cargo run
```
修复起来并不困难，编译时和运行时都很容易检测调用链，阻止调用链闭合。  
或者 AST 内阻止子节点下绑定根节点。  
考虑到这个计算器本身已经有点复杂了，就不过多添加内容了，算是一个瑕疵吧，留给他人来修复吧🥱。

同样的，这个计算器也包含了一些内置函数。内置函数列表：  
ln, lg, sin, cos, tan, acos, asin, atan, sqrt, abs, sinh, cosh, floor, to_rad

## 终端绘图

这部分可以算是这个计算器独有的小特性，不是很实用，但是很有意思
![plot2d](./img/plot2d.png)
图中三个函数及绘图代码如下：
```
fun f(x) = -1/(1+E^(-0.05*x+6)) * 80 + 80 # 逻辑回归函数
%plot2d(f, 0, 240, 0.1)

fun g(x) = -sin(to_rad(10*x)) * 10 + 15 # sin
%plot2d(g, 0, 180, 0.1)

fun t(x) = -((x-25)^3-400*(x-25))+5625 # 三次函数
fun h(x) = t(x)/100
%plot2d(h, 0, 50, 0.01)
```
为了设计上简单，y轴会压缩8倍，x轴会压缩2倍。
同时假如终端使用等宽字体，那么一个字符所占的高等于宽的两倍，所以要自己对函数进行缩放才能和平时绘图的图像一致。  
这是这个计算器唯一依赖项，也是我个人编写的一个极小的项目 [drawille-rs](../drawille/README_cn.md)

## 其他

为了简单，REPL 环境体验可能相当糟糕，没有补全，甚至不能使用方向键等等。  
没有完整的测试过，可能存在上述的 bug 之外的 bug，本质上这类似于一个 demo。  
我还有一大堆的想法（矩阵支持，多线程支持，推理系统...）。但作为一个简单的项目，我不能让它太复杂。之后可能会单独列为一个项目添加更多的特性  

## 设计笔记

这个计算器的代码虽然不多，功能也不算非常多，但是完全可以作为一个大的强大的计算器的起始点。  
在设计上，我特意使用了最为简单但是好用的办法来实现解释器，这个项目的代码几乎都是示例性的。  
比如想要加入内置函数，只需要编写几行代码，而且在 `env.rs` 中都有例子。  
想要添加运算符？编写一个函数即可，可以参考 `parser.rs` 中的代码。  
想要添加更加强大的功能？参考代码中的 `magic function` 即可，也就是参考绘图部分的代码。
其实我已经在代码中实现了函数对象，但是为了更强大的功能，比如不定积分，最终还是选择了修改解释器。这意味着函数对象不能满足需求的时候可以参考 `magic function` 的实现。  
在最近也是最后一次更新中，所有的计算结果都返回一个枚举 `OneMore` 作为包装，这为实现多值函数、矩阵运算等等提供了便利。
