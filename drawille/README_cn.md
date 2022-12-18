# drawille (Rust)

一个 [drawille](https://github.com/asciimoo/drawille) 的复刻版，我暂时不能想到一个有创意的名字就用原来的名字了。

## drawille-rust

这个版本和原来的版本有一些区别，在代码中可以看到，我个人认为是一种改进。

这个版本还包含一个性能测试，可以通过 `cargo bench` 来获取性能测试结果。
在我的笔记本上大约比原作者的快 20 倍，没有特别优化。
Rust 本身的优化就很好，我使用了一些自认为可以减少内存申请次数而更快的手动管理 Vec 的大小的办法，最后使用 `Vec:new()` 以及 Vec 本身的方法居然会快一些。

除了 `lib.rs` 以外，还有如下二进制包:

1. basic

![basic.png](../images/drawille/basic.png)

2. turtle

![turtle.png](../images/drawille/turtle.png)

3. cube

![cube.gif](../images/drawille/cube.gif)
