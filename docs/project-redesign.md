# 项目重新设计方案

## 当前需求分析

### 核心需求
1. **Java CLI集成**: 管理现有的Java CLI jar包，用于自动生成项目
2. **前端项目生成**: 支持Vue和React项目快速生成
3. **npm发布**: 工具需要发布到npm供用户安装使用

### 技术栈选择：napi-rs 架构 (采用)

> 使用 napi-rs 将 Rust 代码直接编译为 Node.js 原生模块，提供更好的性能和集成体验

#### 项目结构 (napi-rs 单仓库)
```
project-generator/
├── Cargo.toml           # Rust 配置
├── package.json         # npm 配置
├── build.rs             # 构建脚本
├── src/
│   ├── lib.rs           # napi-rs 入口
│   ├── java_cli.rs      # Java CLI 集成
│   ├── vue_gen.rs       # Vue 项目生成
│   ├── react_gen.rs     # React 项目生成
│   ├── templates.rs     # 模板管理
│   └── utils.rs         # 工具函数
├── bin/
│   └── cli.js           # CLI 入口脚本
├── lib/
│   └── index.js         # Node.js 接口封装
├── templates/
│   ├── vue/
│   │   ├── basic/
│   │   ├── typescript/
│   │   └── admin/
│   └── react/
│       ├── basic/
│       ├── typescript/
│       └── nextjs/
├── assets/
│   └── java-cli.jar     # Java CLI jar包
├── .github/
│   └── workflows/
│       └── ci.yml       # CI/CD 配置
└── README.md
```

**napi-rs 架构优势:**
- **原生性能**: 直接编译为 Node.js 原生模块，无进程间通信开销
- **类型安全**: TypeScript 类型定义自动生成
- **简化部署**: 单一 npm 包，无需管理多个二进制文件
- **开发体验**: 统一的构建流程，支持热重载
- **内存效率**: 直接在 Node.js 进程中运行，共享内存空间
- **错误处理**: 更好的错误传播和堆栈跟踪

### napi-rs 实现
#### 核心接口设计
```rust
// Rust 端主要接口
#[napi]
pub async fn generate_project(options: GenerateOptions) -> napi::Result<GenerateResult>

#[napi]
pub fn list_templates(project_type: String) -> napi::Result<Vec<String>>

#[napi]
pub fn get_template_info(project_type: String, template: String) -> napi::Result<String>
```

#### Node.js 接口封装
```javascript
// lib/index.js - 简单的 JavaScript 包装器
const { generateProject, listTemplates, getTemplateInfo } = require('../index.node');

class ProjectGenerator {
  async generate(options) {
    return await generateProject(options);
  }
  
  listTemplates(type) {
    return listTemplates(type);
  }
}

module.exports = { ProjectGenerator };
```

#### CLI 工具
```javascript
// bin/cli.js - 基于 commander 的 CLI
#!/usr/bin/env node
const { Command } = require('commander');
const { ProjectGenerator } = require('../lib');

// 基本命令：generate, list
// 支持 --type, --name, --template, --output 参数
```

#### 配置文件

**Cargo.toml**
```toml
[package]
name = "project-generator"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
napi = "2"
napi-derive = "2"
# 其他必要依赖...
```

**package.json**
```json
{
  "name": "@your-org/project-generator",
  "main": "lib/index.js",
  "bin": {
    "project-gen": "./bin/cli.js"
  },
  "scripts": {
    "build": "napi build --platform --release",
    "prepublishOnly": "napi prepublish -t npm"
  },
  "dependencies": {
    "commander": "^9.0.0"
  },
  "devDependencies": {
    "@napi-rs/cli": "^2.0.0"
  }
}
```

#### 发布和分发流程
```yaml
# .github/workflows/ci.yml
# 多平台构建：Windows, macOS, Linux
# 自动发布到 npm
```

```javascript
// npm仓库 - scripts/download-binaries.js
const https = require('https');
const fs = require('fs');
const path = require('path');

const RUST_REPO = 'username/project-generator-core';
const VERSION = process.env.RUST_VERSION || 'latest';

async function downloadBinaries() {
  const platforms = [
    { os: 'win32', arch: 'x64', target: 'x86_64-pc-windows-msvc', ext: '.exe' },
    { os: 'darwin', arch: 'x64', target: 'x86_64-apple-darwin', ext: '' },
    { os: 'linux', arch: 'x64', target: 'x86_64-unknown-linux-gnu', ext: '' }
  ];

  for (const platform of platforms) {
    const url = `https://github.com/${RUST_REPO}/releases/download/${VERSION}/project-gen${platform.ext}-${platform.target}`;
    const outputDir = path.join(__dirname, '..', 'binaries', platform.os, platform.arch);
    const outputFile = path.join(outputDir, `project-gen${platform.ext}`);
    
    await downloadFile(url, outputFile);
    
    // 设置执行权限 (Unix系统)
    if (platform.ext === '') {
      fs.chmodSync(outputFile, 0o755);
    }
  }
}

downloadBinaries().catch(console.error);
```

### 跨平台设计考虑

#### 1. 二进制文件管理
```
binaries/
├── win32/
│   ├── x64/
│   │   └── project-gen.exe
│   └── arm64/
│       └── project-gen.exe
├── darwin/
│   ├── x64/
│   │   └── project-gen
│   └── arm64/
│       └── project-gen
└── linux/
    ├── x64/
    │   └── project-gen
    ├── arm64/
    │   └── project-gen
    └── musl/
        └── project-gen
```

#### 2. 平台检测与二进制选择
```javascript
// bin/cli.js
const os = require('os');
const path = require('path');

function getBinaryPath() {
  const platform = os.platform();
  const arch = os.arch();
  
  const platformMap = {
    'win32': 'win32',
    'darwin': 'darwin',
    'linux': 'linux'
  };
  
  const archMap = {
    'x64': 'x64',
    'arm64': 'arm64',
    'aarch64': 'arm64'
  };
  
  const platformDir = platformMap[platform];
  const archDir = archMap[arch] || 'x64';
  
  const binaryName = platform === 'win32' ? 'project-gen.exe' : 'project-gen';
  
  return path.join(__dirname, '..', 'binaries', platformDir, archDir, binaryName);
}
```

#### 3. Java环境跨平台兼容
```rust
// src/java_cli.rs
use std::process::Command;
use std::env;

pub fn detect_java() -> Result<String, String> {
    // 检查JAVA_HOME环境变量
    if let Ok(java_home) = env::var("JAVA_HOME") {
        let java_path = if cfg!(windows) {
            format!("{}/bin/java.exe", java_home)
        } else {
            format!("{}/bin/java", java_home)
        };
        
        if std::path::Path::new(&java_path).exists() {
            return Ok(java_path);
        }
    }
    
    // 尝试从PATH中查找java
    let java_cmd = if cfg!(windows) { "java.exe" } else { "java" };
    
    match Command::new("which").arg(java_cmd).output() {
        Ok(output) if output.status.success() => {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        }
        _ => Err("Java not found in PATH or JAVA_HOME".to_string())
    }
}
```

#### 4. 路径处理跨平台兼容
```rust
// src/templates.rs
use std::path::{Path, PathBuf};

pub fn normalize_path(path: &str) -> PathBuf {
    // 处理Windows和Unix路径分隔符
    let normalized = if cfg!(windows) {
        path.replace('/', "\\")
    } else {
        path.replace('\\', "/")
    };
    
    PathBuf::from(normalized)
}

pub fn get_template_path(template_type: &str, template_name: &str) -> PathBuf {
    let mut path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    
    path.push("templates");
    path.push(template_type);
    path.push(template_name);
    
    path
}
```

#### 5. npm包发布配置
```json
// package.json
{
  "name": "project-generator",
  "version": "1.0.0",
  "bin": {
    "project-gen": "./bin/cli.js"
  },
  "files": [
    "bin/",
    "binaries/",
    "templates/",
    "assets/"
  ],
  "os": ["win32", "darwin", "linux"],
  "cpu": ["x64", "arm64"],
  "engines": {
    "node": ">=14.0.0"
  }
}
```

#### 6. CI/CD构建流程
```yaml
# .github/workflows/build.yml
name: Build Cross-Platform Binaries

on: [push, pull_request]

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            binary: project-gen.exe
            path: binaries/win32/x64/
          - os: windows-latest
            target: aarch64-pc-windows-msvc
            binary: project-gen.exe
            path: binaries/win32/arm64/
          - os: macos-latest
            target: x86_64-apple-darwin
            binary: project-gen
            path: binaries/darwin/x64/
          - os: macos-latest
            target: aarch64-apple-darwin
            binary: project-gen
            path: binaries/darwin/arm64/
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            binary: project-gen
            path: binaries/linux/x64/
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            binary: project-gen
            path: binaries/linux/arm64/
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          target: ${{ matrix.target }}
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      - name: Copy binary
        run: |
          mkdir -p ${{ matrix.path }}
          cp target/${{ matrix.target }}/release/${{ matrix.binary }} ${{ matrix.path }}
```

### 前端项目生成策略

#### 模板方式 (推荐)
**优势:**
- 维护简单
- 自定义程度高
- 版本控制友好
- 快速生成

**实现:**
```
templates/
├── vue/
│   ├── basic/
│   ├── typescript/
│   └── admin/
└── react/
    ├── basic/
    ├── typescript/
    └── nextjs/
```

#### 动态生成方式
**优势:**
- 更灵活的配置
- 可以集成最新的脚手架工具
- 减少模板维护工作

**实现:**
- Vue: 调用 `npm create vue@latest`
- React: 调用 `npx create-react-app` 或 `npm create vite@latest`

### 推荐架构

```javascript
// CLI主入口
class ProjectGenerator {
  constructor() {
    this.javaCliPath = path.join(__dirname, '../assets/java-cli.jar');
  }

  async generateProject(type, options) {
    switch(type) {
      case 'java':
        return this.runJavaCli(options);
      case 'vue':
        return this.generateVueProject(options);
      case 'react':
        return this.generateReactProject(options);
    }
  }

  async runJavaCli(options) {
    // 调用Java CLI jar包
    const command = `java -jar ${this.javaCliPath} ${options.join(' ')}`;
    return execSync(command);
  }

  async generateVueProject(options) {
    // 使用模板或动态生成
    if (options.useTemplate) {
      return this.copyTemplate('vue', options.template, options.name);
    } else {
      return this.runVueCreate(options);
    }
  }
}
```

### 命令行接口设计

```bash
# 安装
npm install -g @your-org/project-generator

# 生成项目
project-gen generate --type vue --name my-app
project-gen generate --type react --name my-app
project-gen generate --type java --name my-app

# 列出模板
project-gen list --type vue
```

### 下一步行动计划

1. **决定技术栈**: Node.js CLI vs Rust CLI + npm wrapper
2. **设计模板结构**: 确定Vue/React模板的组织方式
3. **Java CLI集成**: 测试Java CLI调用方式
4. **实现MVP**: 先实现一个基础版本
5. **npm发布**: 配置发布流程

### 建议

考虑到您的需求，我建议：
1. **使用Node.js重写CLI**: 更适合npm生态和前端项目生成
2. **采用模板方式**: 对于常用配置，提供预设模板
3. **保留动态生成选项**: 对于需要最新版本的场景
4. **Java CLI作为子模块**: 通过child_process调用

这样既能满足您的所有需求，又能保持项目的简洁性和可维护性。

## 总结

采用 **napi-rs 架构**，将 Rust 编译为 Node.js 原生模块：

### 核心特性
- ✅ Java CLI 集成
- ✅ Vue/React 项目生成
- ✅ npm 发布和分发
- ✅ 跨平台支持

### 主要优势
- **性能**: Rust 原生性能
- **集成**: 完美融入 npm 生态
- **简化**: 单仓库管理
- **自动化**: CI/CD 自动构建发布
