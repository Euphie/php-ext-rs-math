# PHP Rust 扩展学习指南

## 项目结构

```
php_ext_test/
├── src/
│   └── lib.rs              # Rust 核心代码 - 实现数学函数
├── php_rust_math.c         # C 包装层 - 将 Rust 函数包装成 PHP 扩展
├── php_rust_math.h         # C 头文件 - 声明函数接口
├── config.m4               # PHP 扩展构建配置
├── configure.ac            # autoconf 配置文件
├── config.h.in             # 配置头文件模板
├── Cargo.toml              # Rust 项目配置
├── build.sh                # 自动化构建脚本
├── test.php                # 测试文件
├── example.php             # 使用示例
├── Dockerfile              # Docker 环境配置
├── docker-compose.yml      # 容器编排配置
├── README.md               # 项目文档
└── .gitignore              # Git 忽略文件
```

## 学习路径

### 1. 理解 Rust 核心代码 (src/lib.rs)
- 学习 `#[no_mangle]` 和 `extern "C"` 的作用
- 理解 Rust 和 C 之间的数据类型转换
- 掌握内存管理（如 `std::mem::forget` 的使用）

### 2. 理解 C 包装层 (php_rust_math.c)
- 学习 PHP 扩展的基本结构
- 理解 `ZEND_FUNCTION` 宏的使用
- 掌握 PHP 参数解析和返回值处理

### 3. 理解构建系统
- `config.m4`: 配置如何链接 Rust 静态库
- `configure.ac`: 定义构建检查
- `build.sh`: 自动化构建流程

### 4. 运行和测试
```bash
# 启动 Docker 环境
docker-compose up -d

# 构建扩展
docker exec php-rust-extension-dev bash -c "cd /app && ./build.sh"

# 运行测试
docker exec php-rust-extension-dev bash -c "cd /app && php -d extension=modules/rust_math.so test.php"

# 运行示例
docker exec php-rust-extension-dev bash -c "cd /app && php -d extension=modules/rust_math.so example.php"
```

## 核心概念

### Rust 导出函数
```rust
#[no_mangle]  // 防止 Rust 修改函数名
pub extern "C" fn rust_add_impl(a: c_long, b: c_long) -> c_long {
    a + b
}
```

### PHP 扩展函数
```c
ZEND_FUNCTION(rust_add) {
    long a, b;
    
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "ll", &a, &b) == FAILURE) {
        RETURN_NULL();
    }
    
    RETURN_LONG(rust_add_impl(a, b));
}
```

### 构建配置
```m4
# 链接 Rust 静态库
RUST_MATH_SHARED_LIBADD="${RUST_MATH_SHARED_LIBADD} ${RUST_LIB_FILE}"
```

## 扩展学习建议

1. **修改现有函数**: 尝试修改 Rust 函数的实现
2. **添加新函数**: 在 Rust 中添加新的数学函数，并在 C 层包装
3. **优化性能**: 比较 Rust 和 PHP 原生实现的性能差异
4. **错误处理**: 改进错误处理机制
5. **内存管理**: 深入理解 Rust 和 C 之间的内存传递

## 常见问题

### Q: 为什么需要 C 包装层？
A: PHP 扩展必须用 C 编写，Rust 代码需要通过 C 接口暴露给 PHP。

### Q: `#[no_mangle]` 的作用是什么？
A: 防止 Rust 编译器修改函数名，确保 C 代码能找到正确的函数。

### Q: 如何处理内存管理？
A: 使用 `std::mem::forget` 防止 Rust 自动释放内存，在 C 层手动管理。

## 下一步

1. 阅读 `src/lib.rs` 中的详细注释
2. 运行测试，观察输出结果
3. 尝试修改代码，重新构建
4. 参考 PHP 扩展开发文档深入学习 