//! PHP 数学扩展 - 使用 Rust 编写
//! 
//! 这个扩展提供了丰富的数学函数，演示如何使用 Rust 编写 PHP 扩展
//! 
//! ## 模块结构
//! 
//! - `types`: 定义与 C 代码交互的数据结构
//! - `utils`: 工具函数和辅助方法
//! - `math`: 数学运算函数
//!   - `basic`: 基本数学运算（加减乘除等）
//!   - `advanced`: 高级数学运算（三角函数、对数等）
//! 
//! ## 作者
//! 学习 Rust 的新手
//! 
//! ## 目的
//! 学习 Rust 和 PHP 扩展开发

// 导入必要的标准库模块
use std::os::raw::{c_long, c_double, c_int};
use std::ptr;

// 声明子模块
pub mod types;
pub mod utils;
pub mod math;

// 重新导出主要类型
pub use types::{RustFibonacciResult, MathError, MathResult};

// 重新导出主要数学函数
pub use math::{add, multiply, factorial, fibonacci, is_prime, gcd, lcm, power};

// ============================================================================
// C 接口函数 - 这些函数将被导出到 C 代码
// ============================================================================

/// 导出函数：计算两个整数的和
/// 
/// 这个函数可以从 C 代码调用
/// #[no_mangle] 告诉 Rust 不要修改函数名
/// extern "C" 指定使用 C 调用约定
#[no_mangle]
pub extern "C" fn rust_add_impl(a: c_long, b: c_long) -> c_long {
    add(a, b)
}

/// 导出函数：计算两个数的乘积
#[no_mangle]
pub extern "C" fn rust_multiply_impl(a: c_double, b: c_double) -> c_double {
    multiply(a, b)
}

/// 导出函数：计算阶乘
#[no_mangle]
pub extern "C" fn rust_factorial_impl(n: c_long) -> c_long {
    match factorial(n) {
        MathResult { value, error: MathError::Success } => value,
        MathResult { value: _, error: MathError::NegativeNumber } => -1,
        MathResult { value: _, error: MathError::Overflow } => -2,
        _ => -3,
    }
}

/// 导出函数：生成斐波那契数列
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

/// 导出函数：释放斐波那契数列结果的内存
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

/// 导出函数：检查一个数是否为质数
#[no_mangle]
pub extern "C" fn rust_is_prime_impl(n: c_long) -> bool {
    is_prime(n)
}

/// 导出函数：计算最大公约数
#[no_mangle]
pub extern "C" fn rust_gcd_impl(a: c_long, b: c_long) -> c_long {
    gcd(a, b)
}

/// 导出函数：计算最小公倍数
#[no_mangle]
pub extern "C" fn rust_lcm_impl(a: c_long, b: c_long) -> c_long {
    lcm(a, b)
}

/// 导出函数：计算幂运算
#[no_mangle]
pub extern "C" fn rust_power_impl(base: c_long, exponent: c_long) -> c_long {
    match power(base, exponent) {
        MathResult { value, error: MathError::Success } => value,
        _ => -1, // 错误情况
    }
}

// ============================================================================
// 单元测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rust_add_impl() {
        assert_eq!(rust_add_impl(5, 3), 8);
        assert_eq!(rust_add_impl(-5, 3), -2);
        assert_eq!(rust_add_impl(0, 0), 0);
        assert_eq!(rust_add_impl(100, 200), 300);
    }
    
    #[test]
    fn test_rust_multiply_impl() {
        assert_eq!(rust_multiply_impl(5.0, 3.0), 15.0);
        assert_eq!(rust_multiply_impl(-2.0, 4.0), -8.0);
        assert_eq!(rust_multiply_impl(0.0, 5.0), 0.0);
        assert_eq!(rust_multiply_impl(2.5, 3.5), 8.75);
    }
    
    #[test]
    fn test_rust_factorial_impl() {
        assert_eq!(rust_factorial_impl(0), 1);
        assert_eq!(rust_factorial_impl(1), 1);
        assert_eq!(rust_factorial_impl(5), 120);
        assert_eq!(rust_factorial_impl(10), 3628800);
        assert_eq!(rust_factorial_impl(-1), -1); // 负数错误
        assert_eq!(rust_factorial_impl(25), -2); // 溢出错误
    }
    
    #[test]
    fn test_rust_fibonacci_impl() {
        let result = rust_fibonacci_impl(0);
        assert_eq!(result.length, 0);
        
        let result = rust_fibonacci_impl(1);
        assert_eq!(result.length, 1);
        unsafe {
            assert_eq!(*result.numbers, 0);
        }
        
        let result = rust_fibonacci_impl(5);
        assert_eq!(result.length, 5);
        unsafe {
            let numbers = std::slice::from_raw_parts(result.numbers, 5);
            assert_eq!(numbers, [0, 1, 1, 2, 3]);
        }
        
        // 清理内存
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
    fn test_rust_gcd_impl() {
        assert_eq!(rust_gcd_impl(12, 18), 6);
        assert_eq!(rust_gcd_impl(7, 13), 1);
        assert_eq!(rust_gcd_impl(0, 5), 5);
        assert_eq!(rust_gcd_impl(-12, 18), 6);
    }
    
    #[test]
    fn test_rust_lcm_impl() {
        assert_eq!(rust_lcm_impl(12, 18), 36);
        assert_eq!(rust_lcm_impl(7, 13), 91);
        assert_eq!(rust_lcm_impl(0, 5), 0);
    }
    
    #[test]
    fn test_rust_power_impl() {
        assert_eq!(rust_power_impl(2, 3), 8);
        assert_eq!(rust_power_impl(5, 0), 1);
        assert_eq!(rust_power_impl(2, 10), 1024);
        assert_eq!(rust_power_impl(2, -1), -1); // 负数指数错误
    }
    
    // 内部函数测试
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).value, 1);
        assert_eq!(factorial(1).value, 1);
        assert_eq!(factorial(5).value, 120);
        assert_eq!(factorial(10).value, 3628800);
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), vec![]);
        assert_eq!(fibonacci(1), vec![0]);
        assert_eq!(fibonacci(2), vec![0, 1]);
        assert_eq!(fibonacci(5), vec![0, 1, 1, 2, 3]);
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