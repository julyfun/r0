知乎书签：

```
https://zhuanlan.zhihu.com/p/261342464
https://zhuanlan.zhihu.com/p/260707957
https://zhuanlan.zhihu.com/p/260157026
```

语法书：

https://gitee.com/siriusdemon/Rust-One-Piece/raw/master/book.pdf

编译原理词法分析：

https://zhuanlan.zhihu.com/p/363589423

# 知识点

## 汇编

- ❓ 见下

```
movq $42, -8(%rbp) ; 这里占用的栈地址是从 -8 到 -15 吗？
movq -8(%rbp), -16(%rbp)
movq -16(%rbp), %rax
jmp conclusion
```

- ✅ 不是，是 -0 到 -7 (ref: csapp p128)
