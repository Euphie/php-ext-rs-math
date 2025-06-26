//! 基本数学运算模块
//! 
//! 这个模块包含基本的数学运算函数
//! 如加减乘除、取模、绝对值等

use std::os::raw::{c_long, c_double};
use crate::utils::{safe_add, safe_multiply};

/// 整数加法运算
/// 
/// # 参数
/// * `a` - 第一个加数
/// * `b` - 第二个加数
/// 
/// # 返回值
/// 两个整数的和，防止溢出
pub fn add_integers(a: c_long, b: c_long) -> c_long {
    safe_add(a, b)
}

/// 整数减法运算
/// 
/// # 参数
/// * `a` - 被减数
/// * `b` - 减数
/// 
/// # 返回值
/// 两个整数的差，防止溢出
pub fn subtract_integers(a: c_long, b: c_long) -> c_long {
    a.saturating_sub(b)
}

/// 整数乘法运算
/// 
/// # 参数
/// * `a` - 第一个乘数
/// * `b` - 第二个乘数
/// 
/// # 返回值
/// 两个整数的积，防止溢出
pub fn multiply_integers(a: c_long, b: c_long) -> c_long {
    safe_multiply(a, b)
}

/// 整数除法运算
/// 
/// # 参数
/// * `a` - 被除数
/// * `b` - 除数
/// 
/// # 返回值
/// 两个整数的商，如果除数为0则返回0
pub fn divide_integers(a: c_long, b: c_long) -> c_long {
    if b == 0 {
        0
    } else {
        a / b
    }
}

/// 整数取模运算
/// 
/// # 参数
/// * `a` - 被除数
/// * `b` - 除数
/// 
/// # 返回值
/// 两个整数的余数，如果除数为0则返回0
pub fn modulo_integers(a: c_long, b: c_long) -> c_long {
    if b == 0 {
        0
    } else {
        a % b
    }
}

/// 浮点数加法运算
/// 
/// # 参数
/// * `a` - 第一个加数
/// * `b` - 第二个加数
/// 
/// # 返回值
/// 两个浮点数的和
pub fn add_floats(a: c_double, b: c_double) -> c_double {
    a + b
}

/// 浮点数减法运算
/// 
/// # 参数
/// * `a` - 被减数
/// * `b` - 减数
/// 
/// # 返回值
/// 两个浮点数的差
pub fn subtract_floats(a: c_double, b: c_double) -> c_double {
    a - b
}

/// 浮点数乘法运算
/// 
/// # 参数
/// * `a` - 第一个乘数
/// * `b` - 第二个乘数
/// 
/// # 返回值
/// 两个浮点数的积
pub fn multiply_floats(a: c_double, b: c_double) -> c_double {
    a * b
}

/// 浮点数除法运算
/// 
/// # 参数
/// * `a` - 被除数
/// * `b` - 除数
/// 
/// # 返回值
/// 两个浮点数的商，如果除数为0则返回0.0
pub fn divide_floats(a: c_double, b: c_double) -> c_double {
    if b == 0.0 {
        0.0
    } else {
        a / b
    }
}

/// 计算整数的绝对值
/// 
/// # 参数
/// * `n` - 要计算绝对值的整数
/// 
/// # 返回值
/// 整数的绝对值
pub fn abs_integer(n: c_long) -> c_long {
    n.abs()
}

/// 计算浮点数的绝对值
/// 
/// # 参数
/// * `n` - 要计算绝对值的浮点数
/// 
/// # 返回值
/// 浮点数的绝对值
pub fn abs_float(n: c_double) -> c_double {
    n.abs()
}

/// 计算两个整数的最大值
/// 
/// # 参数
/// * `a` - 第一个整数
/// * `b` - 第二个整数
/// 
/// # 返回值
/// 两个整数中的最大值
pub fn max_integers(a: c_long, b: c_long) -> c_long {
    a.max(b)
}

/// 计算两个整数的最小值
/// 
/// # 参数
/// * `a` - 第一个整数
/// * `b` - 第二个整数
/// 
/// # 返回值
/// 两个整数中的最小值
pub fn min_integers(a: c_long, b: c_long) -> c_long {
    a.min(b)
}

/// 计算两个浮点数的最大值
/// 
/// # 参数
/// * `a` - 第一个浮点数
/// * `b` - 第二个浮点数
/// 
/// # 返回值
/// 两个浮点数中的最大值
pub fn max_floats(a: c_double, b: c_double) -> c_double {
    a.max(b)
}

/// 计算两个浮点数的最小值
/// 
/// # 参数
/// * `a` - 第一个浮点数
/// * `b` - 第二个浮点数
/// 
/// # 返回值
/// 两个浮点数中的最小值
pub fn min_floats(a: c_double, b: c_double) -> c_double {
    a.min(b)
} 