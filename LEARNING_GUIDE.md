# PHP Rust 扩展学习指南

## 项目结构

```
php_ext_test/
├── src/
│   ├── lib.rs              # 主模块 - 组织所有子模块和导出 C 接口
│   ├── types/
│   │   └── mod.rs          # 类型定义 - 与 C 代码交互的数据结构
│   ├── utils/
│   │   └── mod.rs          # 工具函数 - 通用辅助函数
│   └── math/
│       ├── mod.rs          # 数学运算主模块
│       ├── basic.rs        # 基本数学运算（加减乘除等）
│       └── advanced.rs     # 高级数学运算（三角函数、对数等）
├── c_src/
│   ├── php_rust_math.c     # C 包装层 - 将 Rust 函数包装成 PHP 扩展
│   └── php_rust_math.h     # C 头文件 - 声明函数接口
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

## 模块化设计

### 1. 类型定义模块 (src/types/mod.rs)
- `RustFibonacciResult`: 斐波那契数列结果结构体
- `MathError`: 数学运算错误码枚举
- `MathResult<T>`: 通用数学运算结果包装器

### 2. 工具函数模块 (src/utils/mod.rs)
- 数学工具函数：`gcd()`, `lcm()`, `sqrt_int()` 等
- 安全检查函数：`safe_add()`, `safe_multiply()` 等
- 辅助函数：`is_even()`, `is_odd()`, `is_in_range()` 等

### 3. 数学运算模块 (src/math/)
- **主模块 (mod.rs)**: 核心数学函数和模块组织
- **基本运算 (basic.rs)**: 加减乘除、绝对值、最大最小值等
- **高级运算 (advanced.rs)**: 三角函数、对数、伽马函数、贝塞尔函数等

### 4. 主模块 (src/lib.rs)
- 模块声明和导入
- C 接口函数导出（`#[no_mangle]` 函数）
- 单元测试

## 学习路径

### 1. 理解模块化结构
- 学习 Rust 模块系统：`mod`, `pub mod`, `use`
- 理解模块间的依赖关系
- 掌握 `pub use` 重新导出

### 2. 理解类型定义 (src/types/mod.rs)
- 学习 `#[repr(C)]` 的作用和内存布局
- 理解 Rust 和 C 之间的类型转换
- 掌握错误处理模式

### 3. 理解工具函数 (src/utils/mod.rs)
- 学习通用工具函数的设计
- 理解安全运算的重要性
- 掌握函数复用和模块化

### 4. 理解数学运算模块 (src/math/)
- 学习按功能分组的设计模式
- 理解基本运算和高级运算的分离
- 掌握模块间的协作

### 5. 理解 C 接口 (src/lib.rs)
- 学习 `#[no_mangle]` 和 `extern "C"` 的使用
- 理解内存管理（`std::mem::forget`）
- 掌握错误码的处理

### 6. 运行和测试
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

### Rust 模块系统
```rust
// 声明子模块
pub mod types;
pub mod utils;
pub mod math;

// 重新导出
pub use types::{RustFibonacciResult, MathError, MathResult};
pub use math::{add, multiply, factorial, fibonacci, is_prime};
```

### C 接口函数
```rust
#[no_mangle]  // 防止 Rust 修改函数名
pub extern "C" fn rust_add_impl(a: c_long, b: c_long) -> c_long {
    add(a, b)  // 调用内部函数
}
```

### 错误处理
```rust
pub fn factorial(n: c_long) -> MathResult<c_long> {
    if n < 0 {
        return MathResult::error(MathError::NegativeNumber, -1);
    }
    // ... 计算逻辑
    MathResult::success(result)
}
```

### 内存管理
```rust
// 防止 Rust 自动释放内存
std::mem::forget(numbers);

// 在 C 代码中手动释放
pub extern "C" fn rust_free_fibonacci_result(result: RustFibonacciResult) {
    // 将指针转换回 Rust 向量，自动释放
}
```

## 扩展学习建议

1. **添加新模块**: 尝试创建新的功能模块（如统计函数、矩阵运算等）
2. **改进错误处理**: 使用更丰富的错误类型和错误信息
3. **优化性能**: 使用更高效的算法和数据结构
4. **添加配置**: 实现可配置的参数（如精度、算法选择等）
5. **文档生成**: 使用 `cargo doc` 生成 API 文档

## 常见问题

### Q: 为什么要模块化？
A: 模块化提高代码的可读性、可维护性和可扩展性，便于团队协作和功能扩展。

### Q: `#[repr(C)]` 的作用是什么？
A: 确保结构体的内存布局与 C 代码兼容，使 Rust 和 C 之间可以安全地传递数据。

### Q: 如何添加新的数学函数？
A: 1. 在 `src/math/` 中添加函数实现
   2. 在 `src/lib.rs` 中添加 C 接口函数
   3. 在 `php_rust_math.c` 中添加 PHP 包装函数
   4. 更新测试和文档

## 下一步

1. 阅读各个模块的详细注释
2. 尝试添加新的数学函数
3. 学习 Rust 的高级特性（如泛型、trait 等）
4. 探索 PHP 扩展开发的最佳实践
5. 参考 Rust 和 PHP 的官方文档深入学习 