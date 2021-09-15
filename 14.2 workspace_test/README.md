## Cargo工作空间
整个工作空间只在根目录下有一个Cargo.lock文件，确保所有的内部包会使用完全相同的依赖版本

### 配置根目录`Cargo.toml`
```
[workspace]

members = [
  "adder",
  "add-one"
]
```

### 运行工作空间内指定包
```bash
cargo run -p adder
```

