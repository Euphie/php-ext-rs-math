dnl PHP 数学扩展配置文件
dnl 这个文件用于配置 PHP 扩展的构建过程

PHP_ARG_ENABLE(rust-math, 是否启用 Rust 数学扩展支持,
[  --enable-rust-math      Enable Rust math extension support])

if test "$PHP_RUST_MATH" != "no"; then
    dnl 检查 Rust 工具链
    AC_CHECK_PROG(RUSTC, rustc, rustc, no)
    AC_CHECK_PROG(CARGO, cargo, cargo, no)
    
    if test "$RUSTC" = "no" || test "$CARGO" = "no"; then
        AC_MSG_ERROR([需要 Rust 工具链。请安装 rustc 和 cargo。])
    fi
    
    dnl 检查 C++ 编译器（用于链接）
    AC_PROG_CXX
    
    dnl 构建 Rust 静态库
    echo "构建 Rust 静态库..."
    cargo build --release
    if test $? -ne 0; then
        AC_MSG_ERROR([Rust 静态库构建失败])
    fi
    echo "完成"
    
    dnl 查找 Rust 静态库
    RUST_LIB_FILE=""
    if test -f "target/release/librust_math.a"; then
        RUST_LIB_FILE="target/release/librust_math.a"
    elif test -f "target/release/deps/librust_math.a"; then
        RUST_LIB_FILE="target/release/deps/librust_math.a"
    else
        RUST_LIB_FILES=$(find target/release -name "librust_math*.a" -type f 2>/dev/null)
        if test -n "$RUST_LIB_FILES"; then
            RUST_LIB_FILE=$(echo "$RUST_LIB_FILES" | head -1)
        fi
    fi
    
    if test -z "$RUST_LIB_FILE" || test ! -f "$RUST_LIB_FILE"; then
        AC_MSG_ERROR([未找到 Rust 静态库])
    fi
    
    echo "configure: 找到 Rust 静态库: $RUST_LIB_FILE"
    
    dnl 设置扩展信息
    PHP_REQUIRE_CXX()
    PHP_ADD_INCLUDE([$ext_srcdir/c_src])
    
    dnl 编译 C 源文件
    PHP_NEW_EXTENSION(rust_math, c_src/php_rust_math.c, $ext_shared)
    
    dnl 链接 Rust 静态库
    RUST_MATH_SHARED_LIBADD="${RUST_MATH_SHARED_LIBADD} ${RUST_LIB_FILE}"
    
    dnl 链接 C++ 标准库
    PHP_ADD_LIBRARY(stdc++, 1, RUST_MATH_SHARED_LIBADD)
    
    dnl 设置扩展依赖
    PHP_SUBST(RUST_MATH_SHARED_LIBADD)
fi 