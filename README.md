# 跨平台缓存目录获取库

这个Rust库提供了一个跨平台获取缓存目录的函数。它能够根据不同的操作系统返回相应的缓存目录路径，并处理可能出现的错误。
缓存目录用途很广，多数平台都有支持，并且使用方便且不需要任何敏感权限。

## 功能

- 获取当前平台的缓存目录路径。
- 处理在不同操作系统上获取缓存目录时可能出现的错误。
- 支持的平台： Windows, Linux, MacOS, iOS, WASM等。

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
    Ok(cache_dir) => println!("Cache directory: {:?}", cache_dir),
    Err(e) => eprintln!("Error: {}", e),
}
```

更多细节请查看[examples](examples)文件夹中的示例。


## 贡献

如果您有任何改进意见或想要贡献代码，请随时提交Pull Request或创建Issue。

## 许可证

本项目采用MIT许可证。有关详细信息，请查看LICENSE文件。