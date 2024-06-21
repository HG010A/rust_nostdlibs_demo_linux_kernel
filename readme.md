## Demo rust内核模块



### 基础

用了一个rust写lkm的基础项目https://github.com/fishinabarrel/linux-kernel-module-rust.git在旧的linux内核上使用rust开发内核模块（在新的内核6.1里https://lwn.net/Articles/910762/理论上可以直接用rust开发lkm）

大概原理是用binding讲linux内核原有内核函数代理为rust函数，rust代码编译为内核二进制结构的静态链接库，最后变为lkm ko模块



### 添加的东西

#### hello-rhai

https://github.com/rhaiscript/rhai.git脚本语言

测试rhai no-std情况下可在内核运行demo

#### hello-json

测试json序列化库可在内核内使用的demo



#### 使用rust在内核里写代码的好处

1、性能和c有差距但是不大

2、安全-内存错误

3、rust依赖管理方便



一些rust库本身不需要运行时（hello-json）或者可以指定no-std的features（hello-rhai）不依赖运行时，可以直接在内核中方便的用。

rust也可以方便的嵌入其他语言里或者做为其他语言的模块，理论上如果做一个规则引擎可以在任何地方运行并统一语法。好维护、bug少。



#### 一些rust内核模块

https://lwn.net/Articles/907685/





#### 代码测试环境

ubuntu 20.04

自己编译的内核linux-5.10

##### 依赖

rustc 1.69.0-nightly (e972bc808 2023-01-29)

clang-11



### demo截图

#### hello-rhai 目录root@lkm:~/linux-kernel-module-rust/hello-rhai#

##### src/lib.rs


##### 编译

make

##### 安装lkm

insmod helloworld.ko

##### 查看输出

dmesg




#### hello-json 目录root@lkm:~/linux-kernel-module-rust/hello-json#

##### src/lib.rs


##### 编译

make

##### 安装lkm

insmod helloworld.ko

##### 查看输出

dmesg



# Hacking Group HG010A
<h1 align="center">
  <img src="https://github.com/HG010A/.github/blob/main/HG010A_logo.png" alt="starfile" width="400px">
  <br>
</h1>
来自于民间的网络安全无名小卒，专注于实战攻防与漏洞研究，以开放式的平等、互助、不带任何商业式的技术交流组织。
