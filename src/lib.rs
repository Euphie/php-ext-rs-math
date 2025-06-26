//! PHP 数学扩展 - 使用 Rust 编写
//! 
//! 这个扩展提供了几个简单的数学函数，演示如何使用 Rust 编写 PHP 扩展
//! 
//! 作者：学习 Rust 的新手
//! 目的：学习 Rust 和 PHP 扩展开发

// 导入必要的标准库模块
use std::os::raw::{c_long, c_double, c_int};
use std::ptr;

// 定义斐波那契数列结果结构体，与 C 代码保持一致
#[repr(C)]
pub struct RustFibonacciResult {
    pub numbers: *mut c_long,
    pub length: c_int,
}

// 导出函数：计算两个整数的和
// 
// 这个函数可以从 C 代码调用
// #[no_mangle] 告诉 Rust 不要修改函数名
// extern "C" 指定使用 C 调用约定
#[no_mangle]
pub extern "C" fn rust_add_impl(a: c_long, b: c_long) -> c_long {
    // 简单的加法运算
    a + b
}

// 导出函数：计算两个数的乘积
#[no_mangle]
pub extern "C" fn rust_multiply_impl(a: c_double, b: c_double) -> c_double {
    // 简单的乘法运算
    a * b
}

// 导出函数：计算阶乘
#[no_mangle]
pub extern "C" fn rust_factorial_impl(n: c_long) -> c_long {
    // 参数验证
    if n < 0 {
        return -1; // 错误码
    }
    
    if n > 20 {
        return -2; // 溢出错误码
    }
    
    // 计算阶乘
    factorial(n)
}

// 导出函数：生成斐波那契数列
#[no_mangle]
pub extern "C" fn rust_fibonacci_impl(n: c_long) -> RustFibonacciResult {
    // 参数验证
    if n < 0 || n > 100 {
        return RustFibonacciResult {
            numbers: ptr::null_mut(),
            length: 0,
        };
    }
    
    // 生成斐波那契数列
    let sequence = fibonacci(n);
    
    // 将 Rust 向量转换为 C 数组
    let length = sequence.len() as c_int;
    let mut numbers = Vec::with_capacity(length as usize);
    
    for &num in &sequence {
        numbers.push(num as c_long);
    }
    
    // 获取数组的原始指针
    let numbers_ptr = numbers.as_mut_ptr();
    
    // 防止 Rust 在函数结束时释放内存
    std::mem::forget(numbers);
    
    RustFibonacciResult {
        numbers: numbers_ptr,
        length,
    }
}

// 导出函数：释放斐波那契数列结果的内存
#[no_mangle]
pub extern "C" fn rust_free_fibonacci_result(result: RustFibonacciResult) {
    // 将 C 数组转换回 Rust 向量，这样 Rust 会自动释放内存
    if !result.numbers.is_null() && result.length > 0 {
        unsafe {
            let _numbers = Vec::from_raw_parts(
                result.numbers,
                result.length as usize,
                result.length as usize,
            );
            // 当 _numbers 离开作用域时，内存会被自动释放
        }
    }
}

// 导出函数：检查一个数是否为质数
#[no_mangle]
pub extern "C" fn rust_is_prime_impl(n: c_long) -> bool {
    // 参数验证
    if n < 2 {
        return false;
    }
    
    // 检查是否为质数
    is_prime(n)
}

// 内部函数：计算阶乘
// 
// 参数：n - 要计算阶乘的非负整数
// 返回值：n 的阶乘
fn factorial(n: c_long) -> c_long {
    if n <= 1 {
        return 1;
    }
    
    // 使用迭代方法计算阶乘，避免栈溢出
    let mut result: c_long = 1;
    for i in 2..=n {
        result = result.saturating_mul(i);
    }
    
    result
}

// 内部函数：生成斐波那契数列
// 
// 参数：n - 要生成的数列长度
// 返回值：包含前 n 个斐波那契数的向量
fn fibonacci(n: c_long) -> Vec<c_long> {
    if n == 0 {
        return Vec::new();
    }
    
    if n == 1 {
        return vec![0];
    }
    
    if n == 2 {
        return vec![0, 1];
    }
    
    // 使用迭代方法生成斐波那契数列
    let mut sequence = vec![0, 1];
    
    for i in 2..n {
        let prev1: c_long = sequence[(i - 1) as usize];
        let prev2: c_long = sequence[(i - 2) as usize];
        let next = prev1.saturating_add(prev2);
        sequence.push(next);
    }
    
    sequence
}

// 内部函数：检查一个数是否为质数
// 
// 参数：n - 要检查的数
// 返回值：如果是质数返回 true，否则返回 false
fn is_prime(n: c_long) -> bool {
    // 2 是唯一的偶质数
    if n == 2 {
        return true;
    }
    
    // 小于 2 的数或偶数（除了 2）都不是质数
    if n < 2 || n % 2 == 0 {
        return false;
    }
    
    // 检查奇数因子，只需要检查到 sqrt(n)
    let sqrt_n = (n as f64).sqrt() as c_long;
    let mut i: c_long = 3;
    
    while i <= sqrt_n {
        if n % i == 0 {
            return false;
        }
        i += 2; // 只检查奇数
    }
    
    true
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rust_add_impl() {
        assert_eq!(rust_add_impl(5, 3), 8);
        assert_eq!(rust_add_impl(-5, 3), -2);
        assert_eq!(rust_add_impl(0, 0), 0);
    }
    
    #[test]
    fn test_rust_multiply_impl() {
        assert_eq!(rust_multiply_impl(5.0, 3.0), 15.0);
        assert_eq!(rust_multiply_impl(-2.0, 4.0), -8.0);
        assert_eq!(rust_multiply_impl(0.0, 5.0), 0.0);
    }
    
    #[test]
    fn test_rust_factorial_impl() {
        assert_eq!(rust_factorial_impl(0), 1);
        assert_eq!(rust_factorial_impl(1), 1);
        assert_eq!(rust_factorial_impl(5), 120);
        assert_eq!(rust_factorial_impl(10), 3628800);
        assert_eq!(rust_factorial_impl(-1), -1); // 错误码
    }
    
    #[test]
    fn test_rust_fibonacci_impl() {
        let result = rust_fibonacci_impl(0);
        assert_eq!(result.length, 0);
        rust_free_fibonacci_result(result);
        
        let result = rust_fibonacci_impl(1);
        assert_eq!(result.length, 1);
        unsafe {
            assert_eq!(*result.numbers, 0);
        }
        rust_free_fibonacci_result(result);
        
        let result = rust_fibonacci_impl(5);
        assert_eq!(result.length, 5);
        unsafe {
            assert_eq!(*result.numbers, 0);
            assert_eq!(*result.numbers.offset(1), 1);
            assert_eq!(*result.numbers.offset(2), 1);
            assert_eq!(*result.numbers.offset(3), 2);
            assert_eq!(*result.numbers.offset(4), 3);
        }
        rust_free_fibonacci_result(result);
    }
    
    #[test]
    fn test_rust_is_prime_impl() {
        assert_eq!(rust_is_prime_impl(2), true);
        assert_eq!(rust_is_prime_impl(3), true);
        assert_eq!(rust_is_prime_impl(4), false);
        assert_eq!(rust_is_prime_impl(5), true);
        assert_eq!(rust_is_prime_impl(6), false);
        assert_eq!(rust_is_prime_impl(7), true);
        assert_eq!(rust_is_prime_impl(8), false);
        assert_eq!(rust_is_prime_impl(9), false);
        assert_eq!(rust_is_prime_impl(10), false);
        assert_eq!(rust_is_prime_impl(11), true);
        assert_eq!(rust_is_prime_impl(1), false);
        assert_eq!(rust_is_prime_impl(0), false);
        assert_eq!(rust_is_prime_impl(-1), false);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), vec![]);
        assert_eq!(fibonacci(1), vec![0]);
        assert_eq!(fibonacci(2), vec![0, 1]);
        assert_eq!(fibonacci(5), vec![0, 1, 1, 2, 3]);
        assert_eq!(fibonacci(8), vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }
    
    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(-1), false);
    }
} 