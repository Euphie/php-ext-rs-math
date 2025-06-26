/**
 * PHP Rust 数学扩展 - 头文件
 * 
 * 这个文件定义了 Rust 库的 C 接口
 * 它允许 C 代码调用 Rust 函数
 */

#ifndef PHP_RUST_MATH_H
#define PHP_RUST_MATH_H

#include "php.h"

/* 扩展模块 ID */
extern zend_module_entry rust_math_module_entry;
#define phpext_rust_math_ptr &rust_math_module_entry

/* 斐波那契数列结果结构体 */
typedef struct {
    long* numbers;  /* 数列数组 */
    int length;     /* 数组长度 */
} rust_fibonacci_result;

/* Rust 函数声明 - 这些函数在 Rust 库中实现 */

/**
 * 计算两个整数的和
 * 
 * @param a 第一个整数
 * @param b 第二个整数
 * @return 两数之和
 */
long rust_add_impl(long a, long b);

/**
 * 计算两个数的乘积
 * 
 * @param a 第一个数
 * @param b 第二个数
 * @return 两数之积
 */
double rust_multiply_impl(double a, double b);

/**
 * 计算阶乘
 * 
 * @param n 要计算阶乘的非负整数
 * @return n 的阶乘
 */
long rust_factorial_impl(long n);

/**
 * 生成斐波那契数列
 * 
 * @param n 要生成的数列长度
 * @return 包含前 n 个斐波那契数的结构体
 */
rust_fibonacci_result rust_fibonacci_impl(long n);

/**
 * 释放斐波那契数列结果的内存
 * 
 * @param result 要释放的结果结构体
 */
void rust_free_fibonacci_result(rust_fibonacci_result result);

/**
 * 检查一个数是否为质数
 * 
 * @param n 要检查的数
 * @return 如果是质数返回 true，否则返回 false
 */
bool rust_is_prime_impl(long n);

#endif /* PHP_RUST_MATH_H */ 