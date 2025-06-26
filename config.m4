dnl PHP 数学扩展配置文件
dnl 这个文件用于配置 PHP 扩展的构建过程

PHP_ARG_ENABLE(rust_math, 
    [是否启用 Rust 数学扩展支持],
    [AS_HELP_STRING([--enable-rust-math],
        [启用 Rust 编写的数学扩展])])

if test "$PHP_RUST_MATH" != "no"; then
    dnl 检查 Rust 是否已安装
    AC_CHECK_PROG(RUSTC, rustc, rustc, no)
    if test "$RUSTC" = "no"; then
        AC_MSG_ERROR([需要安装 Rust 编译器 (rustc)])
    fi
    
    dnl 检查 Cargo 是否已安装
    AC_CHECK_PROG(CARGO, cargo, cargo, no)
    if test "$CARGO" = "no"; then
        AC_MSG_ERROR([需要安装 Cargo 包管理器])
    fi
    
    dnl 设置扩展源文件
    PHP_REQUIRE_CXX()
    PHP_ADD_LIBRARY(stdc++, 1, RUST_MATH_SHARED_LIBADD)
    
    dnl 构建 Rust 静态库
    AC_MSG_CHECKING([构建 Rust 静态库])
    cd $srcdir && $CARGO build --release
    if test $? -ne 0; then
        AC_MSG_ERROR([Rust 静态库构建失败])
    fi
    AC_MSG_RESULT([完成])
    
    dnl 查找 Rust 静态库文件
    RUST_LIB_PATH="$srcdir/target/release/deps"
    RUST_LIB_FILE=$(find $RUST_LIB_PATH -name "librust_math*.a" | head -1)
    if test -z "$RUST_LIB_FILE"; then
        AC_MSG_ERROR([未找到 Rust 静态库文件])
    fi
    AC_MSG_NOTICE([找到 Rust 静态库: $RUST_LIB_FILE])
    ln -sf "$RUST_LIB_FILE" "$RUST_LIB_PATH/librust_math.a"
    
    dnl 链接 Rust 静态库
    RUST_MATH_SHARED_LIBADD="${RUST_MATH_SHARED_LIBADD} ${RUST_LIB_FILE}"
    
    dnl 设置扩展信息
    PHP_SUBST(RUST_MATH_SHARED_LIBADD)
    PHP_NEW_EXTENSION(rust_math, 
        php_rust_math.c, 
        $ext_shared,
        ,, 
        cxx)
    
    dnl 包含头文件目录
    PHP_ADD_INCLUDE($srcdir/include)
fi 