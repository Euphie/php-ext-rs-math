/**
 * PHP Rust 数学扩展 - C 包装层
 * 
 * 这个文件作为 Rust 库和 PHP 之间的桥梁
 * 它处理 PHP 扩展的初始化和函数调用
 */

#include "php.h"
#include "php_ini.h"
#include "ext/standard/info.h"
#include "php_rust_math.h"

/* 扩展版本信息 */
#define PHP_RUST_MATH_VERSION "0.1.0"

/* 函数声明 */
PHP_MINIT_FUNCTION(rust_math);
PHP_MSHUTDOWN_FUNCTION(rust_math);
PHP_RINIT_FUNCTION(rust_math);
PHP_RSHUTDOWN_FUNCTION(rust_math);
PHP_MINFO_FUNCTION(rust_math);

/* 导出函数声明 */
PHP_FUNCTION(rust_add);
PHP_FUNCTION(rust_multiply);
PHP_FUNCTION(rust_factorial);
PHP_FUNCTION(rust_fibonacci);
PHP_FUNCTION(rust_is_prime);

/* 函数列表 */
static const zend_function_entry rust_math_functions[] = {
    PHP_FE(rust_add, NULL)
    PHP_FE(rust_multiply, NULL)
    PHP_FE(rust_factorial, NULL)
    PHP_FE(rust_fibonacci, NULL)
    PHP_FE(rust_is_prime, NULL)
    PHP_FE_END
};

/* 扩展模块 ID */
zend_module_entry rust_math_module_entry = {
    STANDARD_MODULE_HEADER,
    "rust_math",                    /* 扩展名称 */
    rust_math_functions,            /* 函数列表 */
    PHP_MINIT(rust_math),           /* 模块初始化函数 */
    PHP_MSHUTDOWN(rust_math),       /* 模块关闭函数 */
    PHP_RINIT(rust_math),           /* 请求初始化函数 */
    PHP_RSHUTDOWN(rust_math),       /* 请求关闭函数 */
    PHP_MINFO(rust_math),           /* 模块信息函数 */
    PHP_RUST_MATH_VERSION,          /* 扩展版本 */
    STANDARD_MODULE_PROPERTIES
};

/* 实现 Zend 模块入口 */
#ifdef COMPILE_DL_RUST_MATH
#ifdef ZTS
ZEND_TSRMLS_CACHE_DEFINE()
#endif
ZEND_GET_MODULE(rust_math)
#endif

/* 模块初始化函数 */
PHP_MINIT_FUNCTION(rust_math)
{
    /* 在这里可以注册 INI 设置、常量等 */
    return SUCCESS;
}

/* 模块关闭函数 */
PHP_MSHUTDOWN_FUNCTION(rust_math)
{
    /* 清理资源 */
    return SUCCESS;
}

/* 请求初始化函数 */
PHP_RINIT_FUNCTION(rust_math)
{
#if defined(COMPILE_DL_RUST_MATH) && defined(ZTS)
    ZEND_TSRMLS_CACHE_UPDATE();
#endif
    return SUCCESS;
}

/* 请求关闭函数 */
PHP_RSHUTDOWN_FUNCTION(rust_math)
{
    return SUCCESS;
}

/* 模块信息函数 */
PHP_MINFO_FUNCTION(rust_math)
{
    php_info_print_table_start();
    php_info_print_table_header(2, "Rust 数学扩展支持", "enabled");
    php_info_print_table_row(2, "版本", PHP_RUST_MATH_VERSION);
    php_info_print_table_row(2, "作者", "Rust 学习者");
    php_info_print_table_row(2, "描述", "使用 Rust 编写的 PHP 数学扩展");
    php_info_print_table_end();
}

/* PHP 函数：rust_add - 计算两个整数的和 */
PHP_FUNCTION(rust_add)
{
    long a, b;
    
    /* 解析参数 */
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "ll", &a, &b) == FAILURE) {
        RETURN_NULL();
    }
    
    /* 调用 Rust 函数 */
    long result = rust_add_impl(a, b);
    
    /* 返回结果 */
    RETURN_LONG(result);
}

/* PHP 函数：rust_multiply - 计算两个数的乘积 */
PHP_FUNCTION(rust_multiply)
{
    double a, b;
    
    /* 解析参数 */
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "dd", &a, &b) == FAILURE) {
        RETURN_NULL();
    }
    
    /* 调用 Rust 函数 */
    double result = rust_multiply_impl(a, b);
    
    /* 返回结果 */
    RETURN_DOUBLE(result);
}

/* PHP 函数：rust_factorial - 计算阶乘 */
PHP_FUNCTION(rust_factorial)
{
    long n;
    
    /* 解析参数 */
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "l", &n) == FAILURE) {
        RETURN_NULL();
    }
    
    /* 参数验证 */
    if (n < 0) {
        php_error_docref(NULL, E_WARNING, "阶乘不能计算负数");
        RETURN_NULL();
    }
    
    if (n > 20) {
        php_error_docref(NULL, E_WARNING, "阶乘数值过大，可能导致溢出");
        RETURN_NULL();
    }
    
    /* 调用 Rust 函数 */
    long result = rust_factorial_impl(n);
    
    /* 返回结果 */
    RETURN_LONG(result);
}

/* PHP 函数：rust_fibonacci - 生成斐波那契数列 */
PHP_FUNCTION(rust_fibonacci)
{
    long n;
    
    /* 解析参数 */
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "l", &n) == FAILURE) {
        RETURN_NULL();
    }
    
    /* 参数验证 */
    if (n < 0) {
        php_error_docref(NULL, E_WARNING, "斐波那契数列长度不能为负数");
        RETURN_NULL();
    }
    
    if (n > 100) {
        php_error_docref(NULL, E_WARNING, "斐波那契数列长度不能超过 100");
        RETURN_NULL();
    }
    
    /* 调用 Rust 函数获取数列 */
    rust_fibonacci_result result = rust_fibonacci_impl(n);
    
    /* 创建 PHP 数组 */
    array_init(return_value);
    
    /* 将结果添加到数组中 */
    for (int i = 0; i < result.length; i++) {
        add_next_index_long(return_value, result.numbers[i]);
    }
    
    /* 释放 Rust 分配的内存 */
    rust_free_fibonacci_result(result);
}

/* PHP 函数：rust_is_prime - 检查是否为质数 */
PHP_FUNCTION(rust_is_prime)
{
    long n;
    
    /* 解析参数 */
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "l", &n) == FAILURE) {
        RETURN_NULL();
    }
    
    /* 参数验证 */
    if (n < 2) {
        RETURN_BOOL(0);
    }
    
    /* 调用 Rust 函数 */
    bool result = rust_is_prime_impl(n);
    
    /* 返回结果 */
    RETURN_BOOL(result);
}

#ifndef ZEND_DLEXPORT
#define ZEND_DLEXPORT
#endif

ZEND_DLEXPORT zend_module_entry *get_module(void) {
    return &rust_math_module_entry;
} 