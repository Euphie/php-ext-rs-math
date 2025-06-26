<?php
/**
 * PHP Rust 数学扩展测试文件
 * 
 * 这个文件用于测试扩展提供的所有数学函数
 * 运行方式：php test.php
 */

echo "=== PHP Rust 数学扩展测试 ===\n";
echo "作者：Rust 学习者\n";
echo "时间：" . date('Y-m-d H:i:s') . "\n\n";

// 检查扩展是否已加载
if (!extension_loaded("rust_math")) {
    echo "错误：rust_math 扩展未加载！\n";
    echo "请确保扩展已正确安装并启用。\n";
    exit(1);
}

echo "✓ 扩展加载成功\n\n";

// 测试函数列表
$tests = [
    'rust_add' => [
        'description' => '加法运算',
        'tests' => [
            [5, 3, 8],
            [-5, 3, -2],
            [0, 0, 0],
            [100, 200, 300],
        ]
    ],
    'rust_multiply' => [
        'description' => '乘法运算',
        'tests' => [
            [5.0, 3.0, 15.0],
            [-2.0, 4.0, -8.0],
            [0.0, 5.0, 0.0],
            [2.5, 3.5, 8.75],
        ]
    ],
    'rust_factorial' => [
        'description' => '阶乘计算',
        'tests' => [
            [0, 1],
            [1, 1],
            [5, 120],
            [10, 3628800],
        ]
    ],
    'rust_fibonacci' => [
        'description' => '斐波那契数列',
        'tests' => [
            [0, []],
            [1, [0]],
            [2, [0, 1]],
            [5, [0, 1, 1, 2, 3]],
            [8, [0, 1, 1, 2, 3, 5, 8, 13]],
        ]
    ],
    'rust_is_prime' => [
        'description' => '质数检查',
        'tests' => [
            [2, true],
            [3, true],
            [4, false],
            [5, true],
            [6, false],
            [7, true],
            [8, false],
            [9, false],
            [10, false],
            [11, true],
            [1, false],
            [0, false],
            [-1, false],
        ]
    ]
];

// 运行所有测试
$passed = 0;
$total = 0;

foreach ($tests as $function => $test_info) {
    echo "测试 {$test_info['description']} ({$function}):\n";
    
    foreach ($test_info['tests'] as $test) {
        $total++;
        $args = array_slice($test, 0, -1);
        $expected = end($test);
        
        try {
            $result = call_user_func_array($function, $args);
            
            if ($function === 'rust_fibonacci') {
                // 斐波那契数列返回数组，需要特殊处理
                $success = is_array($result) && $result == $expected;
                $result_str = '[' . implode(', ', $result) . ']';
                $expected_str = '[' . implode(', ', $expected) . ']';
            } else {
                $success = $result === $expected;
                $result_str = var_export($result, true);
                $expected_str = var_export($expected, true);
            }
            
            if ($success) {
                echo "  ✓ ";
                $passed++;
            } else {
                echo "  ✗ ";
            }
            
            echo "参数: (" . implode(', ', $args) . ") ";
            echo "结果: {$result_str} ";
            echo "期望: {$expected_str}\n";
            
        } catch (Exception $e) {
            echo "  ✗ 参数: (" . implode(', ', $args) . ") 异常: " . $e->getMessage() . "\n";
        }
    }
    echo "\n";
}

// 性能测试
echo "=== 性能测试 ===\n";

// 测试阶乘性能
$start = microtime(true);
for ($i = 0; $i < 1000; $i++) {
    rust_factorial(10);
}
$end = microtime(true);
echo "阶乘计算 1000 次耗时: " . round(($end - $start) * 1000, 2) . "ms\n";

// 测试质数检查性能
$start = microtime(true);
for ($i = 0; $i < 1000; $i++) {
    rust_is_prime(97);
}
$end = microtime(true);
echo "质数检查 1000 次耗时: " . round(($end - $start) * 1000, 2) . "ms\n";

// 测试斐波那契性能
$start = microtime(true);
for ($i = 0; $i < 100; $i++) {
    rust_fibonacci(20);
}
$end = microtime(true);
echo "斐波那契数列计算 100 次耗时: " . round(($end - $start) * 1000, 2) . "ms\n";

echo "\n=== 测试结果 ===\n";
echo "通过: {$passed}/{$total} 测试\n";
echo "成功率: " . round(($passed / $total) * 100, 1) . "%\n";

if ($passed === $total) {
    echo "🎉 所有测试通过！扩展工作正常。\n";
} else {
    echo "⚠️  部分测试失败，请检查扩展实现。\n";
}

echo "\n=== 扩展信息 ===\n";
echo "PHP 版本: " . PHP_VERSION . "\n";
echo "扩展版本: " . (defined('PHP_RUST_MATH_VERSION') ? PHP_RUST_MATH_VERSION : '未知') . "\n";
echo "扩展路径: " . (extension_loaded('rust_math') ? '已加载' : '未加载') . "\n";

// 显示扩展的详细信息
if (function_exists('phpinfo')) {
    echo "\n扩展详细信息:\n";
    ob_start();
    phpinfo(INFO_MODULES);
    $phpinfo = ob_get_clean();
    
    // 提取 rust_math 相关信息
    if (preg_match('/rust_math.*?<\/table>/s', $phpinfo, $matches)) {
        echo $matches[0] . "\n";
    }
} 