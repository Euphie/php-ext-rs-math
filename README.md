# PHP Rust 数学扩展

这是一个使用 Rust 编写的 PHP 扩展演示项目，提供了几个简单的数学函数。这个项目专门为 Rust 初学者设计，包含了详细的中文注释。

## 🎯 项目目标

- 学习如何使用 Rust 编写 PHP 扩展
- 理解 Rust 和 C 之间的 FFI（外部函数接口）
- 掌握 PHP 扩展开发的基本流程
- 提供完整的开发环境配置

## 📋 功能特性

这个扩展提供了以下数学函数：

1. **`rust_add(a, b)`** - 计算两个整数的和
2. **`rust_multiply(a, b)`** - 计算两个数的乘积
3. **`rust_factorial(n)`** - 计算阶乘
4. **`rust_fibonacci(n)`** - 生成斐波那契数列
5. **`rust_is_prime(n)`** - 检查一个数是否为质数

## 🏗️ 项目结构

```
php_ext_test/
├── src/
│   └── lib.rs              # Rust 核心代码（包含详细注释）
├── Cargo.toml              # Rust 项目配置
├── config.m4               # PHP 扩展构建配置
├── php_rust_math.c         # C 包装层代码
├── php_rust_math.h         # C 头文件
├── Dockerfile              # Docker 开发环境
├── docker-compose.yml      # Docker Compose 配置
├── README.md               # 项目说明文档
└── test.php                # PHP 测试文件
```

## 🚀 快速开始

### 方法一：使用 Docker（推荐）

这是最简单的方法，不会影响你的本地环境：

1. **确保已安装 Docker 和 Docker Compose**

2. **构建并启动开发环境**
   ```bash
   docker-compose up -d
   ```

3. **进入容器**
   ```bash
   docker-compose exec php-rust-dev bash
   ```

4. **构建扩展**
   ```bash
   ./build.sh
   ```

5. **测试扩展**
   ```bash
   ./test.sh
   ```

### 方法二：本地开发

如果你想要在本地环境中开发：

1. **安装依赖**
   - Rust: https://rustup.rs/
   - PHP 开发环境: `php-dev` 包
   - 构建工具: `autoconf`, `automake`, `libtool`

2. **构建 Rust 库**
   ```bash
   cargo build --release
   ```

3. **构建 PHP 扩展**
   ```bash
   phpize
   ./configure --enable-rust-math
   make
   make install
   ```

## 📖 代码说明

### Rust 代码 (`src/lib.rs`)

这个文件包含了所有的数学函数实现，每个函数都有详细的中文注释：

- **导出函数**: 使用 `#[no_mangle]` 和 `extern "C"` 导出给 C 代码调用
- **参数验证**: 检查输入参数的有效性
- **内存管理**: 正确处理 Rust 和 C 之间的内存传递
- **错误处理**: 使用错误码或返回值表示错误状态

### C 包装层 (`php_rust_math.c`)

这个文件作为 Rust 库和 PHP 之间的桥梁：

- **PHP 扩展结构**: 定义扩展的基本信息和生命周期函数
- **函数包装**: 将 PHP 函数调用转换为 Rust 函数调用
- **参数解析**: 使用 `zend_parse_parameters` 解析 PHP 参数
- **返回值处理**: 将 Rust 返回值转换为 PHP 变量

## 🧪 测试扩展

创建一个测试文件 `test.php`：

```php
<?php
// 加载扩展
if (!extension_loaded("rust_math")) {
    dl("rust_math.so");
}

echo "=== Rust 数学扩展测试 ===\n";

// 测试加法
echo "测试加法: 5 + 3 = " . rust_add(5, 3) . "\n";

// 测试乘法
echo "测试乘法: 4.5 * 2.0 = " . rust_multiply(4.5, 2.0) . "\n";

// 测试阶乘
echo "测试阶乘: 5! = " . rust_factorial(5) . "\n";

// 测试斐波那契数列
echo "测试斐波那契数列 (前8个): ";
$fib = rust_fibonacci(8);
echo implode(", ", $fib) . "\n";

// 测试质数检查
echo "测试质数检查:\n";
$numbers = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
foreach ($numbers as $num) {
    $is_prime = rust_is_prime($num) ? "是" : "否";
    echo "  $num 是质数吗？$is_prime\n";
}

echo "=== 测试完成 ===\n";
```

运行测试：
```bash
php test.php
```

## 🔧 开发指南

### 添加新函数

1. **在 Rust 中实现函数** (`src/lib.rs`)
   ```rust
   #[no_mangle]
   pub extern "C" fn rust_new_function_impl(param: c_long) -> c_long {
       // 实现逻辑
       param * 2
   }
   ```

2. **在头文件中声明** (`php_rust_math.h`)
   ```c
   long rust_new_function_impl(long param);
   ```

3. **在 C 包装层中包装** (`php_rust_math.c`)
   ```c
   PHP_FUNCTION(rust_new_function)
   {
       long param;
       
       if (zend_parse_parameters(ZEND_NUM_ARGS(), "l", &param) == FAILURE) {
           RETURN_NULL();
       }
       
       long result = rust_new_function_impl(param);
       RETURN_LONG(result);
   }
   ```

4. **重新构建扩展**
   ```bash
   cargo build --release
   make
   make install
   ```

### 调试技巧

1. **Rust 调试**: 使用 `cargo test` 运行单元测试
2. **PHP 调试**: 在 PHP 代码中添加 `var_dump()` 或 `error_log()`
3. **内存调试**: 使用 Valgrind 检查内存泄漏

## 📚 学习资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [PHP 扩展开发指南](https://www.php.net/manual/en/internals2.php)
- [Rust FFI 指南](https://doc.rust-lang.org/nomicon/ffi.html)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

## 🙏 致谢

感谢 Rust 和 PHP 社区提供的优秀工具和文档！ 