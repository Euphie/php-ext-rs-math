# PHP Rust 扩展开发环境
# 这个 Dockerfile 创建了一个包含 Rust 和 PHP 开发工具的环境

# 使用 Ubuntu 22.04 作为基础镜像
FROM ubuntu:22.04

# 设置环境变量，避免交互式安装
ENV DEBIAN_FRONTEND=noninteractive

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    # 基本开发工具
    build-essential \
    git \
    curl \
    wget \
    pkg-config \
    # PHP 开发环境
    php8.1-dev \
    php8.1-cli \
    php8.1-common \
    # 构建工具
    autoconf \
    automake \
    libtool \
    # 清理缓存
    && rm -rf /var/lib/apt/lists/*

# 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 验证安装
RUN rustc --version && cargo --version && php --version

# 设置工作目录
WORKDIR /app

# 复制项目文件
COPY . .

# 构建 Rust 库
RUN cargo build --release

# 创建构建脚本
RUN echo '#!/bin/bash\n\
echo "开始构建 PHP 扩展..."\n\
\n\
# 构建 Rust 库\n\
echo "构建 Rust 库..."\n\
cargo build --release\n\
\n\
# 配置 PHP 扩展\n\
echo "配置 PHP 扩展..."\n\
phpize\n\
\n\
# 配置构建\n\
echo "配置构建..."\n\
./configure --enable-rust-math\n\
\n\
# 编译扩展\n\
echo "编译扩展..."\n\
make\n\
\n\
# 安装扩展\n\
echo "安装扩展..."\n\
make install\n\
\n\
echo "构建完成！"\n\
echo "扩展已安装到: $(php-config --extension-dir)/rust_math.so"\n\
' > /app/build.sh && chmod +x /app/build.sh

# 创建测试脚本
RUN echo '#!/bin/bash\n\
echo "测试 PHP 扩展..."\n\
\n\
# 创建测试 PHP 文件\n\
cat > /app/test.php << "EOF"\n\
<?php\n\
// 加载扩展\n\
if (!extension_loaded("rust_math")) {\n\
    dl("rust_math.so");\n\
}\n\
\n\
echo "=== Rust 数学扩展测试 ===\n";\n\
\n\
// 测试加法\n\
echo "测试加法: 5 + 3 = " . rust_add(5, 3) . "\n";\n\
\n\
// 测试乘法\n\
echo "测试乘法: 4.5 * 2.0 = " . rust_multiply(4.5, 2.0) . "\n";\n\
\n\
// 测试阶乘\n\
echo "测试阶乘: 5! = " . rust_factorial(5) . "\n";\n\
\n\
// 测试斐波那契数列\n\
echo "测试斐波那契数列 (前8个): ";\n\
$fib = rust_fibonacci(8);\n\
echo implode(", ", $fib) . "\n";\n\
\n\
// 测试质数检查\n\
echo "测试质数检查:\n";\n\
$numbers = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11];\n\
foreach ($numbers as $num) {\n\
    $is_prime = rust_is_prime($num) ? "是" : "否";\n\
    echo "  $num 是质数吗？$is_prime\n";\n\
}\n\
\n\
echo "=== 测试完成 ===\n";\n\
EOF\n\
\n\
# 运行测试\n\
php /app/test.php\n\
' > /app/test.sh && chmod +x /app/test.sh

# 设置默认命令
CMD ["/bin/bash"] 