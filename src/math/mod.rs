//! 数学运算模块
//! 
//! 这个模块包含所有数学函数的实现
//! 包括基本运算、阶乘、斐波那契数列、质数检查等

use std::os::raw::{c_long, c_double};
use crate::types::{MathError, MathResult};
use crate::utils::{sqrt_int, safe_multiply, safe_add};

/// 基本数学运算模块
pub mod basic;
/// 高级数学运算模块
pub mod advanced;

/// 计算两个整数的和
/// 
/// # 参数
/// * `a` - 第一个整数
/// * `b` - 第二个整数
/// 
/// # 返回值
/// 两个整数的和
pub fn add(a: c_long, b: c_long) -> c_long {
    safe_add(a, b)
}

/// 计算两个浮点数的乘积
/// 
/// # 参数
/// * `a` - 第一个浮点数
/// * `b` - 第二个浮点数
/// 
/// # 返回值
/// 两个浮点数的乘积
pub fn multiply(a: c_double, b: c_double) -> c_double {
    a * b
}

/// 计算阶乘
/// 
/// # 参数
/// * `n` - 要计算阶乘的非负整数
/// 
/// # 返回值
/// 阶乘结果，如果输入无效则返回错误码
pub fn factorial(n: c_long) -> MathResult<c_long> {
    // 参数验证
    if n < 0 {
        return MathResult::error(MathError::NegativeNumber, -1);
    }
    
    if n > 20 {
        return MathResult::error(MathError::Overflow, -2);
    }
    
    // 计算阶乘
    let result = if n <= 1 {
        1
    } else {
        let mut result: c_long = 1;
        for i in 2..=n {
            result = safe_multiply(result, i);
        }
        result
    };
    
    MathResult::success(result)
}

/// 生成斐波那契数列
/// 
/// # 参数
/// * `n` - 要生成的数列长度
/// 
/// # 返回值
/// 包含前 n 个斐波那契数的向量
pub fn fibonacci(n: c_long) -> Vec<c_long> {
    if n <= 0 {
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
        let next = safe_add(prev1, prev2);
        sequence.push(next);
    }
    
    sequence
}

/// 检查一个数是否为质数
/// 
/// # 参数
/// * `n` - 要检查的数
/// 
/// # 返回值
/// 如果是质数返回 true，否则返回 false
pub fn is_prime(n: c_long) -> bool {
    // 2 是唯一的偶质数
    if n == 2 {
        return true;
    }
    
    // 小于 2 的数或偶数（除了 2）都不是质数
    if n < 2 || n % 2 == 0 {
        return false;
    }
    
    // 检查奇数因子，只需要检查到 sqrt(n)
    let sqrt_n = sqrt_int(n);
    let mut i: c_long = 3;
    
    while i <= sqrt_n {
        if n % i == 0 {
            return false;
        }
        i += 2; // 只检查奇数
    }
    
    true
}

/// 计算最大公约数
/// 
/// # 参数
/// * `a` - 第一个数字
/// * `b` - 第二个数字
/// 
/// # 返回值
/// 最大公约数
pub fn gcd(a: c_long, b: c_long) -> c_long {
    crate::utils::gcd(a, b)
}

/// 计算最小公倍数
/// 
/// # 参数
/// * `a` - 第一个数字
/// * `b` - 第二个数字
/// 
/// # 返回值
/// 最小公倍数
pub fn lcm(a: c_long, b: c_long) -> c_long {
    crate::utils::lcm(a, b)
}

/// 计算幂运算
/// 
/// # 参数
/// * `base` - 底数
/// * `exponent` - 指数
/// 
/// # 返回值
/// 幂运算结果
pub fn power(base: c_long, exponent: c_long) -> MathResult<c_long> {
    if exponent < 0 {
        return MathResult::error(MathError::InvalidParameter, 0);
    }
    
    if exponent == 0 {
        return MathResult::success(1);
    }
    
    let mut result: c_long = 1;
    let mut base = base;
    let mut exp = exponent;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = safe_multiply(result, base);
        }
        base = safe_multiply(base, base);
        exp /= 2;
    }
    
    MathResult::success(result)
} 