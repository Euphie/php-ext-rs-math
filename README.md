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
├── src/                    # Rust 源代码
│   ├── lib.rs              # 主模块 - 组织所有子模块和导出 C 接口
│   ├── types/              # 类型定义模块
│   │   └── mod.rs          # 与 C 代码交互的数据结构
│   ├── utils/              # 工具函数模块
│   │   └── mod.rs          # 通用辅助函数
│   └── math/               # 数学运算模块
│       ├── mod.rs          # 数学运算主模块
│       ├── basic.rs        # 基本数学运算（加减乘除等）
│       └── advanced.rs     # 高级数学运算（三角函数、对数等）
├── c_src/                  # C 源代码
│   ├── php_rust_math.c     # C 包装层 - PHP 扩展实现
│   └── php_rust_math.h     # C 头文件 - 函数声明
├── config.m4               # PHP 扩展构建配置
├── configure.ac            # autoconf 配置文件
├── config.h.in             # 配置头文件模板
├── Cargo.toml              # Rust 项目配置
├── build.sh                # 自动化构建脚本
├── clean_all.sh            # 一键清理脚本
├── test.php                # 测试文件
├── example.php             # 使用示例
├── Dockerfile              # Docker 环境配置
├── docker-compose.yml      # 容器编排配置
├── README.md               # 项目文档
├── LEARNING_GUIDE.md       # 学习指南
└── .gitignore              # Git 忽略文件
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
   php -d extension=modules/rust_math.so example.php
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

### Rust 代码结构

项目采用模块化设计，将 Rust 代码按功能分组：

#### 主模块 (`src/lib.rs`)
- **模块组织**: 声明和导入所有子模块
- **C 接口导出**: 使用 `#[no_mangle]` 和 `extern "C"` 导出函数
- **内存管理**: 正确处理 Rust 和 C 之间的内存传递

#### 类型定义模块 (`src/types/mod.rs`)
- **数据结构**: 定义与 C 代码交互的数据结构
- **错误处理**: `MathError` 枚举和 `MathResult<T>` 包装器
- **内存布局**: 使用 `#[repr(C)]` 确保与 C 兼容

#### 工具函数模块 (`src/utils/mod.rs`)
- **通用函数**: 提供数学工具函数和安全运算
- **辅助函数**: `gcd()`, `lcm()`, `safe_add()`, `safe_multiply()` 等
- **安全检查**: 防止整数溢出和无效操作

#### 数学运算模块 (`src/math/`)
- **基本运算** (`basic.rs`): 加减乘除、绝对值、最大最小值等
- **高级运算** (`advanced.rs`): 三角函数、对数、伽马函数、贝塞尔函数等
- **核心算法** (`mod.rs`): 阶乘、斐波那契、质数检查等

### C 包装层 (`c_src/php_rust_math.c`)

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

1. **在 Rust 中实现函数**
   
   根据函数类型选择合适的位置：
   
   - **基本运算**: 添加到 `src/math/basic.rs`
   - **高级运算**: 添加到 `src/math/advanced.rs`
   - **核心算法**: 添加到 `src/math/mod.rs`
   
   ```rust
   // 在 src/math/basic.rs 中添加
   pub fn new_basic_function(a: c_long, b: c_long) -> c_long {
       // 实现逻辑
       a + b
   }
   ```

2. **在 C 接口中导出** (`src/lib.rs`)
   ```rust
   #[no_mangle]
   pub extern "C" fn rust_new_function_impl(a: c_long, b: c_long) -> c_long {
       math::basic::new_basic_function(a, b)
   }
   ```

3. **在头文件中声明** (`c_src/php_rust_math.h`)
   ```c
   long rust_new_function_impl(long a, long b);
   ```

4. **在 C 包装层中包装** (`c_src/php_rust_math.c`)
   ```c
   PHP_FUNCTION(rust_new_function)
   {
       long a, b;
       
       if (zend_parse_parameters(ZEND_NUM_ARGS(), "ll", &a, &b) == FAILURE) {
           RETURN_NULL();
       }
       
       long result = rust_new_function_impl(a, b);
       RETURN_LONG(result);
   }
   ```

5. **重新构建扩展**
   ```bash
   cargo build --release
   make
   make install
   ```

### 项目清理

项目提供了便捷的清理脚本：

```bash
# 一键清理所有构建产物
./clean_all.sh

# 或者使用 make 命令
make clean        # 清理编译产物
make distclean    # 清理编译和配置产物
```

清理脚本会删除：
- Rust 构建产物 (`target/`, `Cargo.lock`)
- PHP 扩展构建产物 (`.libs/`, `modules/`, `*.lo`, `*.la`, `*.so`)
- autoconf/automake 产物 (`config.h`, `Makefile`, `libtool` 等)
- 临时文件和日志文件

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