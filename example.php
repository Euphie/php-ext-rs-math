<?php
/**
 * PHP Rust 数学扩展使用示例
 * 
 * 这个文件展示了如何使用扩展提供的数学函数
 */

// 检查扩展是否已加载
if (!extension_loaded("rust_math")) {
    die("错误：请先安装并启用 rust_math 扩展\n");
}

echo "=== PHP Rust 数学扩展使用示例 ===\n\n";

// 示例 1：基本数学运算
echo "1. 基本数学运算:\n";
$a = 10;
$b = 5;
echo "   {$a} + {$b} = " . rust_add($a, $b) . "\n";
echo "   {$a} × {$b} = " . rust_multiply($a, $b) . "\n";
echo "   {$a} × 3.14 = " . rust_multiply($a, 3.14) . "\n\n";

// 示例 2：阶乘计算
echo "2. 阶乘计算:\n";
$numbers = [0, 1, 5, 10];
foreach ($numbers as $n) {
    $result = rust_factorial($n);
    echo "   {$n}! = {$result}\n";
}
echo "\n";

// 示例 3：斐波那契数列
echo "3. 斐波那契数列:\n";
$lengths = [5, 8, 10];
foreach ($lengths as $len) {
    $sequence = rust_fibonacci($len);
    echo "   前{$len}个斐波那契数: [" . implode(", ", $sequence) . "]\n";
}
echo "\n";

// 示例 4：质数检查
echo "4. 质数检查:\n";
$test_numbers = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 20];
foreach ($test_numbers as $num) {
    $is_prime = rust_is_prime($num) ? "是" : "否";
    echo "   {$num} 是质数吗？{$is_prime}\n";
}
echo "\n";

// 示例 5：性能对比
echo "5. 性能测试:\n";

// 测试阶乘性能
$start = microtime(true);
for ($i = 0; $i < 1000; $i++) {
    rust_factorial(10);
}
$rust_time = (microtime(true) - $start) * 1000;

// 对比 PHP 原生实现
$start = microtime(true);
for ($i = 0; $i < 1000; $i++) {
    $result = 1;
    for ($j = 2; $j <= 10; $j++) {
        $result *= $j;
    }
}
$php_time = (microtime(true) - $start) * 1000;

echo "   阶乘计算 1000 次:\n";
echo "   - Rust 扩展: " . round($rust_time, 2) . "ms\n";
echo "   - PHP 原生: " . round($php_time, 2) . "ms\n";
echo "   - 性能提升: " . round($php_time / $rust_time, 1) . "x\n\n";

// 示例 6：错误处理
echo "6. 错误处理示例:\n";

// 测试无效输入
try {
    $result = rust_factorial(-1);
    echo "   负数阶乘结果: {$result}\n";
} catch (Exception $e) {
    echo "   负数阶乘错误: " . $e->getMessage() . "\n";
}

try {
    $result = rust_fibonacci(-5);
    echo "   负数斐波那契结果: " . (is_array($result) ? "[" . implode(", ", $result) . "]" : $result) . "\n";
} catch (Exception $e) {
    echo "   负数斐波那契错误: " . $e->getMessage() . "\n";
}

echo "\n=== 示例完成 ===\n";
echo "这些示例展示了扩展的基本用法。\n";
echo "更多信息请查看 README.md 文件。\n"; 