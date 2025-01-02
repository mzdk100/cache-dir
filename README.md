# 跨平台缓存目录和数据目录获取库

这个Rust库提供了两个跨平台获取可读可写目录的函数。

## get_cache_dir

它能够根据不同的操作系统返回相应的缓存目录路径，并处理可能出现的错误。
缓存目录是一种非常不可控的目录类型，目录中的文件什么时候会被删除完全取决于操作系统，特别是在移动设备上，如果您的APP使用了较多的空间，当用户在使用其他APP时，可能优先删除您APP中的数据，在这种情况下可以改用[get_data_dir]
来代替。
在Windows上，此函数获取的是`%LOCALAPPDATA%\Temp`，其他平台取决于操作系统或者[dirs::data_local_dir]的实现；

## get_data_dir

它能够根据不同的操作系统返回相应的可写的数据目录路径，并处理可能出现的错误。
数据目录是一种可靠的存储目录，可随时读写文件且无需申请权限，与[get_cache_dir]的区别是，此函数获取的数据目录更加持久。
在安卓平台此目录是APP私有目录中的`files`，Windows上是`%LOCALAPPDATA%\Temp`
，其他平台取决于操作系统或者[dirs::data_local_dir]的实现；

缓存目录和数据目录用途很广，多数平台都有支持，并且使用方便且不需要任何敏感权限。
在桌面平台上也强烈推荐使用此库提供的函数，例如在Windows上，多数开发人员可能会直接使用自身APP的根目录作为数据目录，但是这会有问题，例如如果您的应用安装到需要有管理员权限才能读写的目录中(`C:\Program Files`)，此时可能无法正常写入文件。

## 功能

- 获取当前平台的缓存目录路径。
- 获取当前平台的数据目录路径。
- 处理在不同操作系统上获取目录时可能出现的错误。
- 支持的平台： Windows, Linux, MacOS, Android, iOS, WASM等。

## 安装

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
cache-dir = "0.1"
```

## 使用方法

在代码中引入`get_cache_dir`函数：

```rust
use cache_dir::get_cache_dir;
```

然后调用`get_cache_dir`函数来获取缓存目录：

```rust
match get_cache_dir() {
Ok(cache_dir) => println ! ("Cache directory: {:?}", cache_dir),
Err(e) => eprintln !("Error: {}", e),
}
```

更多细节请查看[examples](examples)文件夹中的示例。

## 贡献

如果您有任何改进意见或想要贡献代码，请随时提交Pull Request或创建Issue。

## 许可证

本项目采用MIT许可证。有关详细信息，请查看LICENSE文件。