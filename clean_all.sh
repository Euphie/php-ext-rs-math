#!/bin/bash
# 一键彻底清理所有构建和配置产物
# 用于还原项目到最干净的源码状态

set -e

# 清理 Rust 构建产物
echo "清理 Rust 构建产物..."
rm -rf target/
rm -f Cargo.lock

# 清理 PHP 扩展构建产物
echo "清理 PHP 扩展构建产物..."
rm -rf .libs/ autom4te.cache/ build/ modules/ c_src/.libs/ c_src/*.lo c_src/*.dep c_src/*.o
rm -f *.lo *.dep *.la *.so *.a
rm -f config.log config.status config.h config.h.in~ configure~ config.nice Makefile Makefile.objects Makefile.fragments libtool run-tests.php rust_math.la

# 清理 autoconf/automake 产物
echo "清理 autoconf/automake 产物..."
rm -f config.h.in config.m4 configure config.nice config.status
rm -rf include/

# 清理临时和日志文件
echo "清理临时和日志文件..."
rm -f *.tmp *.temp *.log

# 清理空目录
echo "清理空目录..."
find . -type d -empty -delete

echo "✅ 项目已彻底清理完成！" 