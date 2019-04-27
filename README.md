# cloud-pa
给沙雕群友刷数字用

## 使用
### 预编译可执行程序
windows 用户可以在[Release](https://github.com/chinuno-usami/cloud-pa/releases)下载预编译可执行程序
### 自行编译
1.安装rust编译器:https://www.rust-lang.org/tools/install  
2.clone本仓库
```bash
git clone https://github.com/chinuno-usami/cloud-pa.git
```  
3.编译
```bash
cd cloud-pa
cargo build --release
``` 
可执行文件在target/release下
### 使用方式
```bash
./cloud-pa -h
```

## 注意
请求间没有进行速率控制，需要的自行修改源码添加，不要给服务器加太多压力
