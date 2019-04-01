眼看就要失业了, 为了混口饭吃, 我念了一句诗: "苟利国家生死以, 岂因祸福避趋之", 然后开始刷[LeetCode](https://leetcode.com),

问题集:

* 仓库中的大部分题目来自https://leetcode.com/problemset/top-100-liked-questions
* 有零星几道题是来自[数组专题训练](https://leetcode.com/tag/array/)的题目
* 有些题目还没有`Rust Version`, 有点气人... 还是偷偷用起了`C++`. I am angry!

每道题的题名作为`crate name`, 利用`Cargo`创建一个`library crate`, 所有的题目都添加了单元测试, 方便以后添加新的解法.

如果一道题目有多种解法, 我会在`Cargo.toml`中添加一个`features`列表, 默认的`feature`是时间复杂度最小的解法. 举个栗子:

[32-Longest Valid Parentheses](https://leetcode.com/problems/longest-valid-parentheses/) 有三种比较好的解法:

1. Dynamic Programming(Time: O(n)/Space: O(n))
2. Stack (Same as above)
3. Two Pass Scanning (Time: O(n)/Space: O(1))

这个时候, 我们的`Cargo.toml`就变成了:

```toml
[features]
deafalut = ["best"]

dp=[]
stack=[]
best=[]
```

很惭愧，就做了一点微小的工作，谢谢大家 ! 👓
