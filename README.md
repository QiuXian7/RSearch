# RSearch

## 项目简介

RSearch（Rust Search Tool）是一个基于 Rust 语言开发的命令行文件搜索工具（CLI File Search Tool）。

本项目实现了文件扫描、文件名搜索、文件内容搜索、正则表达式搜索、扩展名过滤、文件大小过滤、最近修改时间过滤、重复文件检测、搜索历史记录以及倒排索引搜索等功能。

项目采用模块化设计，使用 Rust 的所有权机制、错误处理机制、集合容器和文件系统 API 实现，旨在提供一个轻量级、高性能的本地文件搜索解决方案。

---

## 项目特点

* 基于 Rust 开发
* 命令行交互（CLI）
* 模块化设计
* 支持多种搜索方式
* 支持倒排索引
* 支持搜索历史记录
* 支持重复文件检测
* 支持相关度排序
* 支持单元测试

---

## 技术栈

### 开发语言

* Rust 2021 Edition

### 第三方库

| 依赖库        | 用途         |
| ---------- | ---------- |
| clap       | 命令行参数解析    |
| walkdir    | 文件递归遍历     |
| regex      | 正则表达式搜索    |
| serde      | 数据序列化      |
| serde_json | JSON存储     |
| chrono     | 时间处理       |
| sha2       | SHA256哈希计算 |
| anyhow     | 错误处理       |
| thiserror  | 自定义错误      |

---

## 项目结构

```text
src/

├── main.rs
├── cli.rs
├── models.rs
├── scanner.rs
├── search.rs
├── filters.rs
├── duplicate.rs
├── history.rs
├── storage.rs
├── index.rs
└── stats.rs
```

---

## 模块说明

### main.rs

程序入口，负责解析命令行参数并调度各功能模块。

### cli.rs

定义命令行参数结构和子命令。

### models.rs

定义数据模型，例如：

* FileInfo
* SearchResult

### scanner.rs

负责递归扫描目录并获取文件信息。

获取内容包括：

* 文件路径
* 文件名
* 文件扩展名
* 文件大小
* 修改时间

### search.rs

实现：

* 文件名搜索
* 文件内容搜索
* 正则表达式搜索
* 搜索结果预览

### filters.rs

实现：

* 扩展名过滤
* 文件大小过滤
* 最近修改时间过滤

### duplicate.rs

利用 SHA256 哈希算法检测重复文件。

### history.rs

记录用户搜索历史。

### storage.rs

负责索引数据持久化。

### index.rs

实现倒排索引。

### stats.rs

统计目录信息。

### errors.rs

统一管理自定义错误类型。

---

## 功能介绍

### 1. 文件名搜索

根据文件名称查找目标文件。

示例：

```bash
cargo run -- name rust D:\test
```

---

### 2. 文件内容搜索

根据关键词搜索文件内容。

示例：

```bash
cargo run -- content ownership D:\test
```

---

### 3. 正则表达式搜索

支持使用正则表达式匹配内容。

示例：

```bash
cargo run -- regex "Rust.*ownership" D:\test
```

---

### 4. 扩展名过滤

筛选指定扩展名文件。

示例：

```bash
cargo run -- ext md D:\test
```

---

### 5. 文件大小过滤

筛选大于指定大小的文件。

示例：

```bash
cargo run -- size 1024 D:\test
```

---

### 6. 最近修改文件过滤

查找最近 N 天修改的文件。

示例：

```bash
cargo run -- recent 7 D:\test
```

---

### 7. 重复文件检测

检测内容完全相同的文件。

示例：

```bash
cargo run -- duplicate D:\test
```

---

### 8. 搜索历史

查看历史搜索记录。

示例：

```bash
cargo run -- history
```

---

### 9. 构建倒排索引

扫描目录并生成索引文件。

示例：

```bash
cargo run -- build-index D:\test
```

生成：

```text
index.json
```

---

### 10. 索引搜索

通过倒排索引快速搜索。

示例：

```bash
cargo run -- search-index rust
```

输出：

```text
D:\test\rust.md    score=5
D:\test\book.md    score=2
```

---

### 11. 目录统计

统计目录中文件数量及总大小。

示例：

```bash
cargo run -- stats D:\test
```

---

## 倒排索引设计

索引结构如下：

```json
{
  "rust": {
    "rust.md": 8,
    "book.md": 3
  },
  "ownership": {
    "rust.md": 5
  }
}
```

其中：

* Key：关键词
* Value：文件路径及出现次数

搜索时按照关键词出现次数进行相关度排序。

---

## 重复文件检测原理

系统读取文件内容后计算 SHA256 哈希值：

```text
文件内容
    ↓
SHA256
    ↓
哈希值
```

若多个文件哈希值相同，则判定为重复文件。

---

## 测试

项目包含多个单元测试。

运行：

```bash
cargo test
```

示例输出：

```text
running 8 tests

test search::tests::test_search_by_name_found ... ok
test filters::tests::test_filter_by_extension ... ok
test duplicate::tests::test_same_content_hash ... ok

test result: ok
```

---

## 运行环境

* Windows 10 / Windows 11
* Rust 1.70+
* Cargo

---

## 编译运行

安装依赖：

```bash
cargo build
```

运行：

```bash
cargo run
```

查看帮助：

```bash
cargo run -- --help
```

---

## 项目总结

本项目使用 Rust 实现了一个完整的命令行文件搜索系统。

通过文件扫描、内容搜索、正则匹配、过滤器、重复文件检测以及倒排索引等技术，实现了一个功能较为完善的本地搜索工具。

项目实践了 Rust 的模块化开发、所有权机制、错误处理、文件系统操作、集合容器以及单元测试等核心知识，对 Rust 语言学习和工程实践具有较好的参考价值。
