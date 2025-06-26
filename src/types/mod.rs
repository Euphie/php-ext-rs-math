//! 类型定义模块
//! 
//! 这个模块定义了与 C 代码交互的数据结构
//! 所有结构体都使用 #[repr(C)] 确保内存布局与 C 兼容

use std::os::raw::{c_long, c_int};

/// 斐波那契数列结果结构体
/// 
/// 这个结构体用于在 Rust 和 C 之间传递斐波那契数列结果
/// #[repr(C)] 确保内存布局与 C 代码兼容
#[repr(C)]
pub struct RustFibonacciResult {
    /// 斐波那契数列数组的指针
    /// 注意：这个指针指向的内存需要在 C 代码中手动释放
    pub numbers: *mut c_long,
    
    /// 数组的长度
    pub length: c_int,
}

/// 数学运算错误码
/// 
/// 用于表示数学运算中的各种错误情况
#[repr(C)]
pub enum MathError {
    /// 成功
    Success = 0,
    
    /// 负数错误（如负数阶乘）
    NegativeNumber = -1,
    
    /// 溢出错误（如阶乘结果过大）
    Overflow = -2,
    
    /// 无效参数错误
    InvalidParameter = -3,
}

impl MathError {
    /// 将错误码转换为字符串描述
    pub fn to_string(&self) -> &'static str {
        match self {
            MathError::Success => "成功",
            MathError::NegativeNumber => "负数错误",
            MathError::Overflow => "溢出错误",
            MathError::InvalidParameter => "无效参数",
        }
    }
}

/// 数学运算结果
/// 
/// 用于包装数学运算的结果，包含成功值和错误信息
#[repr(C)]
pub struct MathResult<T> {
    /// 运算结果
    pub value: T,
    
    /// 错误码
    pub error: MathError,
}

impl<T> MathResult<T> {
    /// 创建成功结果
    pub fn success(value: T) -> Self {
        Self {
            value,
            error: MathError::Success,
        }
    }
    
    /// 创建错误结果
    pub fn error(error: MathError, value: T) -> Self {
        Self { value, error }
    }
    
    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        matches!(self.error, MathError::Success)
    }
    
    /// 检查是否出错
    pub fn is_error(&self) -> bool {
        !self.is_success()
    }
} 