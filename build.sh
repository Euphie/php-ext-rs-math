#!/bin/bash

# PHP Rust 数学扩展构建脚本
# 这个脚本自动化了扩展的构建过程

set -e  # 遇到错误时退出

echo "=== PHP Rust 数学扩展构建脚本 ==="
echo "时间: $(date)"
echo ""

# 检查必要的工具
echo "检查构建工具..."

# 检查 Rust
if ! command -v rustc &> /dev/null; then
    echo "错误: 未找到 Rust 编译器 (rustc)"
    echo "请安装 Rust: https://rustup.rs/"
    exit 1
fi

# 检查 Cargo
if ! command -v cargo &> /dev/null; then
    echo "错误: 未找到 Cargo 包管理器"
    echo "请安装 Cargo: https://rustup.rs/"
    exit 1
fi

# 检查 PHP 开发工具
if ! command -v phpize &> /dev/null; then
    echo "错误: 未找到 phpize"
    echo "请安装 PHP 开发包: sudo apt-get install php8.1-dev"
    exit 1
fi

echo "✓ 所有工具检查通过"
echo ""

# 显示版本信息
echo "工具版本信息:"
echo "  Rust: $(rustc --version)"
echo "  Cargo: $(cargo --version)"
echo "  PHP: $(php --version | head -1)"
echo ""

# 清理之前的构建
echo "清理之前的构建..."
if [ -d "target" ]; then
    rm -rf target
fi

if [ -f "Makefile" ]; then
    make clean 2>/dev/null || true
fi

echo "✓ 清理完成"
echo ""

# 构建 Rust 库
echo "构建 Rust 库..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Rust 库构建成功"
else
    echo "✗ Rust 库构建失败"
    exit 1
fi
echo ""

# 查找并复制 Rust 静态库
echo "查找 Rust 静态库..."
RUST_LIB_PATH=""
if [ -f "target/release/librust_math.a" ]; then
    RUST_LIB_PATH="target/release/librust_math.a"
    echo "找到 Rust 静态库: $RUST_LIB_PATH"
    echo "✓ Rust 静态库已就绪"
elif [ -f "target/release/deps/librust_math.a" ]; then
    RUST_LIB_PATH="target/release/deps/librust_math.a"
    echo "找到 Rust 静态库: $RUST_LIB_PATH"
    # 复制到标准位置
    cp "$RUST_LIB_PATH" "target/release/librust_math.a"
    echo "✓ Rust 静态库复制成功"
else
    # 查找带哈希后缀的静态库文件
    RUST_LIB_FILES=$(find target/release -name "librust_math*.a" -type f 2>/dev/null)
    if [ -n "$RUST_LIB_FILES" ]; then
        RUST_LIB_PATH=$(echo "$RUST_LIB_FILES" | head -1)
        echo "找到 Rust 静态库: $RUST_LIB_PATH"
        # 复制到标准位置
        cp "$RUST_LIB_PATH" "target/release/librust_math.a"
        echo "✓ Rust 静态库复制成功"
    else
        echo "✗ 未找到 Rust 静态库"
        echo "请检查 Cargo.toml 配置是否正确设置了 crate-type = [\"staticlib\"]"
        exit 1
    fi
fi

# 复制到 deps 目录，确保链接器能找到
mkdir -p target/release/deps
cp "target/release/librust_math.a" "target/release/deps/librust_math.a"
echo "✓ Rust 静态库已复制到 deps 目录"
# 自动查找带哈希后缀的静态库名，并复制一份
HASHED_A=$(find target/release/deps -name "librust_math-*.a" | head -1)
if [ -n "$HASHED_A" ]; then
    cp "target/release/librust_math.a" "$HASHED_A"
    echo "✓ Rust 静态库已重命名为: $HASHED_A"
fi

# 运行 Rust 测试
echo "运行 Rust 单元测试..."
cargo test

if [ $? -eq 0 ]; then
    echo "✓ Rust 测试通过"
else
    echo "✗ Rust 测试失败"
    exit 1
fi
echo ""

# 配置 PHP 扩展
echo "配置 PHP 扩展..."
phpize

if [ $? -eq 0 ]; then
    echo "✓ PHP 扩展配置成功"
else
    echo "✗ PHP 扩展配置失败"
    exit 1
fi
echo ""

# 配置构建
echo "配置构建..."
./configure --enable-rust-math

if [ $? -eq 0 ]; then
    echo "✓ 构建配置成功"
else
    echo "✗ 构建配置失败"
    exit 1
fi
echo ""

# 编译扩展
echo "编译 PHP 扩展..."
# 确保 Rust 静态库存在
cargo build --release
cp target/release/librust_math.a target/release/deps/librust_math.a
HASHED_A=$(find target/release/deps -name "librust_math-*.a" | head -1)
if [ -n "$HASHED_A" ]; then
    cp target/release/librust_math.a "$HASHED_A"
    echo "✓ Rust 静态库已重命名为: $HASHED_A"
fi

make

if [ $? -eq 0 ]; then
    echo "✓ PHP 扩展编译成功"
else
    echo "✗ PHP 扩展编译失败"
    exit 1
fi
echo ""

# 安装扩展
echo "安装 PHP 扩展..."
make install

if [ $? -eq 0 ]; then
    echo "✓ PHP 扩展安装成功"
else
    echo "✗ PHP 扩展安装失败"
    exit 1
fi
echo ""

# 显示安装信息
echo "=== 安装信息 ==="
echo "扩展文件位置: $(php-config --extension-dir)/rust_math.so"
echo "PHP 扩展目录: $(php-config --extension-dir)"
echo ""

# 检查扩展是否可以加载
echo "测试扩展加载..."
php -m | grep rust_math

if [ $? -eq 0 ]; then
    echo "✓ 扩展加载成功"
else
    echo "⚠️  扩展未自动加载，需要手动配置"
    echo "请在 php.ini 中添加: extension=rust_math.so"
fi
echo ""

# 运行 PHP 测试
if [ -f "test.php" ]; then
    echo "运行 PHP 测试..."
    php test.php
    
    if [ $? -eq 0 ]; then
        echo "✓ PHP 测试通过"
    else
        echo "✗ PHP 测试失败"
        exit 1
    fi
    echo ""
fi

echo "=== 构建完成 ==="
echo "🎉 扩展构建和安装成功！"
echo ""
echo "使用方法:"
echo "1. 在 PHP 代码中调用扩展函数"
echo "2. 运行 'php test.php' 进行测试"
echo "3. 查看 README.md 了解更多信息"
echo ""
echo "构建时间: $(date)" 