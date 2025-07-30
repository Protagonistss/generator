# Vue Generator CLI

一个用 Rust 编写的 Vue 项目生成器，用于快速创建 Vue 项目模板。

## 安装

```bash
cargo build --release
```

## 使用方法

### 创建新项目

```bash
# 创建基础 Vue 项目
cargo run -- new my-vue-app --template basic

# 创建 TypeScript Vue 项目
cargo run -- new my-ts-app --template typescript

# 创建集成 Rust 后端的 Vue 项目
cargo run -- new my-fullstack-app --template rust-integration

# 指定目录
cargo run -- new my-app --template basic --dir ./projects/my-app
```

### 查看可用模板

```bash
cargo run -- list
```

### 查看模板信息

```bash
cargo run -- info basic
cargo run -- info typescript
cargo run -- info rust-integration
```

## 可用模板

- **basic**: 基础的 Vue 3 项目模板
- **typescript**: 支持 TypeScript 的 Vue 3 项目
- **rust-integration**: 集成 Rust 后端的 Vue 3 项目

## 开发计划

这是一个渐进式开发的项目，当前版本提供基础的 CLI 框架。后续将逐步添加：

1. 模板文件生成逻辑
2. 配置文件自动生成
3. 依赖管理
4. 更多项目模板
5. 交互式项目配置

## 项目结构

```
generator/
├── Cargo.toml          # 项目配置
├── src/
│   └── main.rs         # 主程序入口
└── README.md           # 项目说明
```

## 贡献

欢迎提交 Issue 和 Pull Request！