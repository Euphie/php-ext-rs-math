//! 工具函数模块
//! 
//! 这个模块包含一些通用的辅助函数，用于支持数学运算

use std::os::raw::c_long;

/// 检查数字是否为偶数
/// 
/// # 参数
/// * `n` - 要检查的数字
/// 
/// # 返回值
/// 如果是偶数返回 true，否则返回 false
pub fn is_even(n: c_long) -> bool {
    n % 2 == 0
}

/// 检查数字是否为奇数
/// 
/// # 参数
/// * `n` - 要检查的数字
/// 
/// # 返回值
/// 如果是奇数返回 true，否则返回 false
pub fn is_odd(n: c_long) -> bool {
    n % 2 != 0
}

/// 计算数字的平方根（整数部分）
/// 
/// # 参数
/// * `n` - 要计算平方根的数字
/// 
/// # 返回值
/// 平方根的整数部分
pub fn sqrt_int(n: c_long) -> c_long {
    if n <= 0 {
        return 0;
    }
    
    let n_f64 = n as f64;
    n_f64.sqrt() as c_long
}

/// 计算两个数字的最大公约数
/// 
/// 使用欧几里得算法计算最大公约数
/// 
/// # 参数
/// * `a` - 第一个数字
/// * `b` - 第二个数字
/// 
/// # 返回值
/// 最大公约数
pub fn gcd(mut a: c_long, mut b: c_long) -> c_long {
    // 处理负数
    a = a.abs();
    b = b.abs();
    
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    
    a
}

/// 计算两个数字的最小公倍数
/// 
/// # 参数
/// * `a` - 第一个数字
/// * `b` - 第二个数字
/// 
/// # 返回值
/// 最小公倍数
pub fn lcm(a: c_long, b: c_long) -> c_long {
    if a == 0 || b == 0 {
        return 0;
    }
    
    (a * b).abs() / gcd(a, b)
}

/// 检查数字是否在指定范围内
/// 
/// # 参数
/// * `n` - 要检查的数字
/// * `min` - 最小值（包含）
/// * `max` - 最大值（包含）
/// 
/// # 返回值
/// 如果在范围内返回 true，否则返回 false
pub fn is_in_range(n: c_long, min: c_long, max: c_long) -> bool {
    n >= min && n <= max
}

/// 安全的整数乘法，防止溢出
/// 
/// # 参数
/// * `a` - 第一个乘数
/// * `b` - 第二个乘数
/// 
/// # 返回值
/// 乘法结果，如果溢出则返回最大/最小值
pub fn safe_multiply(a: c_long, b: c_long) -> c_long {
    a.saturating_mul(b)
}

/// 安全的整数加法，防止溢出
/// 
/// # 参数
/// * `a` - 第一个加数
/// * `b` - 第二个加数
/// 
/// # 返回值
/// 加法结果，如果溢出则返回最大/最小值
pub fn safe_add(a: c_long, b: c_long) -> c_long {
    a.saturating_add(b)
} 