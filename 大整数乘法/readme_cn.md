# 大整数乘法

If you need Readme English Version, click [here](readme.md).

## 目录

```sh
$ tree
.
├── Cargo.toml
├── datas
│   ├── data1.csv
│   └── data2.csv
├── draw.py
├── examples
│   ├── bad_case.rs
│   └── rand_case.rs
├── readme.md
├── readme_cn.md
├── results
│   ├── result1.svg
│   ├── result1_py.png
│   ├── result2.svg
│   ├── result2_py.png
│   └── result_cmp_py.png
├── src
│   ├── big_integer.rs
│   ├── bin
│   │   ├── draw.rs
│   │   └── old.rs
│   ├── lib.rs
│   └── main.rs
└── tests
    └── mult_tests.rs
```

## 环境配置

|  软件/硬件  |       版本        |
| :---------: | :---------------: |
|  操作系统   | Linux/Ubuntu20.04 |
|     RAM     |       64GB        |
|     CPU     |                   |
|     Vim     |     8.1.2269      |
| Rust(rustc) |      1.61.0       |
|    Cargo    |      1.61.0       |

## 程序运行

### 下载

```sh
git clone https://github.com/furtherun/big-integer-multiplication
```

### 编译

```sh
cd big-integer-multiplication
cargo build --release
```

### 运行一个例子

```sh
cargo run --example rand_case
```

这个例子会随机生成两个10位的大整数，
并且使用三种方式分别计算大整数乘法，并将结果打印出来。

```output
x = 2010695756, y = 3273900753
2010695756*3273900753
res1=6582818349622304268
res2=6582818349622304268
res3=6582817397422304268
```

### 运行实验

```sh
cargo run --bin big_int_mult
```

如果你在命令行窗口看到类似如下文本，说明程序正在进行模拟大整数计算。

```output
Big integer length = 10 generated.
mult takes 0.038 ms, mult_recur takes 0.124 ms, mult_recur_pro takes 0.019 ms
Big integer length = 10 finished.
Big integer length = 100 generated.
mult takes 1.909 ms, mult_recur takes 0.475 ms, mult_recur_pro takes 0.026 ms
Big integer length = 100 finished.
Big integer length = 1000 generated.
mult takes 187 ms, mult_recur takes 420 ms, mult_recur_pro takes 0.112 ms
...
```

乘法运行时间数据保存在`/datas`文件夹中。

### 绘图

```sh
cargo run --bin draw
```

绘图结果保存在`/results`文件夹中。
目前Rust的绘图效果不太理想，这里提供了Python绘图的代码，可以在得到数据后，运行`draw.py`。

## 结果展示

![result](results/result_cmp_py.png)

## 未优化的方法

```sh
cargo run --bin old
```

未优化的方法运行速度相对优化后的方法来说要慢很多。

## 可能存在的问题

1. `test_mult_recur()`测试存在问题，报告内存溢出，但我并没有找到造成错误的原因；
2. `mult_recur_pro()`方法的实验结果表现很好，但这个函数计算结果有时候会出错，可以运行
`cargo run --example bad_case`看出这种情况下中间几位似乎有些问题。
