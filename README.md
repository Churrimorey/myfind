# myfind

### 功能实现

在基础功能的基础上，实现了进阶中的五个功能，具体如下：

将该代码重构到多个模块中

因为考虑到`main.rs`主要进行与外部的交互，而`find()`和`walk_tree()`实现了内部的功能，因此可以将该两个方法封装到另一个模块；考虑到`find()`和`walk_tree()`的功能并不在一个层次上，`walk_tree()`操作更底层，因此分别将两个方法封装在两个模块中。

支持命令行彩色输出

增加了依赖库`colored`，实现命令行输出的彩色

![image-20230904191650357](C:\Users\ASUS\AppData\Roaming\Typora\typora-user-images\image-20230904191650357.png)

增加新的功能，`-v`/`--verbose` 参数输出所有遍历到的文件

设定`-v`或`--verbose`只能是第一个参数或者最后一个参数

同时支持匹配多个正则

通过`iterator`和`closure`实现每一个正则表达式结果的交集运算

```rust
let tmp_matches: Vec<&String> = matches
                                .iter()
                                .filter(|&s| dir_matches.contains(s))
                                .collect();
dir_matches.clear();
for s in tmp_matches {
	dir_matches.push(s.to_string());
}
```

支持同时搜索多个 path

与多个正则表达式通过路径中的`\`和`/`进行区分，路径参数在前，正则表达式参数在后

通过`iterator`和`closure`实现每一个路径结果的并集运算

```rust
let dir_matches: Vec<&String> = dir_matches
            .iter()
            .filter(|&s| !final_matches.contains(s))
            .collect();
for filename in dir_matches {
	final_matches.push(filename.to_string());
}
```

### 综合结果

![image-20230904192707039](C:\Users\ASUS\AppData\Roaming\Typora\typora-user-images\image-20230904192707039.png)