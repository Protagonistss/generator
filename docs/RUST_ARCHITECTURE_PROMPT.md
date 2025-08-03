# 🦀 Rust 架构设计专业提示词

## 🎯 架构设计核心原则

### 1. 所有权驱动设计 (Ownership-Driven Design)
```rust
// ✅ 清晰的所有权转移
struct ProjectGenerator {
    config: GeneratorConfig,
    templates: TemplateRegistry,
}

impl ProjectGenerator {
    // 消费 self，明确所有权转移
    pub fn generate(self, options: GenerateOptions) -> Result<GeneratedProject> {
        // 实现逻辑
    }
}
```

**设计要点**:
- 明确每个值的所有者
- 避免不必要的克隆和引用
- 使用移动语义优化性能

### 2. 类型安全优先 (Type Safety First)
```rust
// ✅ 使用新类型模式增强类型安全
#[derive(Debug, Clone)]
pub struct ProjectName(String);

#[derive(Debug, Clone)]
pub struct TemplatePath(PathBuf);

impl ProjectName {
    pub fn new(name: String) -> Result<Self, ValidationError> {
        if name.is_empty() || name.contains('/') {
            return Err(ValidationError::InvalidProjectName);
        }
        Ok(ProjectName(name))
    }
}
```

**设计要点**:
- 使用新类型包装原始类型
- 在类型层面编码业务规则
- 让编译器帮助捕获错误

### 3. 错误处理策略 (Error Handling Strategy)
```rust
// ✅ 分层错误处理
#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Template not found: {template}")]
    TemplateNotFound { template: String },
    
    #[error("Invalid project name: {name}")]
    InvalidProjectName { name: String },
    
    #[error("Java environment error: {0}")]
    JavaEnvironment(#[from] JavaError),
}

// napi 错误转换
impl From<GeneratorError> for napi::Error {
    fn from(err: GeneratorError) -> Self {
        napi::Error::new(
            napi::Status::GenericFailure,
            format!("{:#}", err)
        )
    }
}
```

**设计要点**:
- 使用 `thiserror` 定义结构化错误
- 实现错误链和上下文传播
- 提供清晰的错误信息

## 🏗️ napi-rs 架构模式

### 1. 分层架构 (Layered Architecture)
```
┌─────────────────────────────────────┐
│           CLI Layer                 │  ← Node.js CLI 工具
├─────────────────────────────────────┤
│           napi Layer                │  ← Rust ↔ Node.js 接口
├─────────────────────────────────────┤
│         Business Layer              │  ← 核心业务逻辑
├─────────────────────────────────────┤
│        Infrastructure Layer         │  ← 文件系统、外部工具
└─────────────────────────────────────┘
```

### 2. 模块化设计 (Modular Design)
```rust
// src/lib.rs - napi 入口点
#[macro_use]
extern crate napi_derive;

mod error;
mod templates;
mod generators;
mod utils;

use error::GeneratorError;
use generators::{JavaGenerator, VueGenerator, ReactGenerator};

#[napi(object)]
pub struct GenerateOptions {
    pub project_name: String,
    pub template: String,
    pub output_dir: String,
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[napi]
pub async fn generate_project(options: GenerateOptions) -> napi::Result<GenerateResult> {
    let generator = match options.template.as_str() {
        "java" => Box::new(JavaGenerator::new()) as Box<dyn ProjectGenerator>,
        "vue" => Box::new(VueGenerator::new()) as Box<dyn ProjectGenerator>,
        "react" => Box::new(ReactGenerator::new()) as Box<dyn ProjectGenerator>,
        _ => return Err(GeneratorError::UnsupportedTemplate.into()),
    };
    
    let result = generator.generate(options).await
        .map_err(GeneratorError::from)?;
    
    Ok(result)
}
```

### 3. 异步处理模式 (Async Patterns)
```rust
// ✅ 异步 trait 设计
#[async_trait::async_trait]
pub trait ProjectGenerator: Send + Sync {
    async fn generate(&self, options: GenerateOptions) -> Result<GenerateResult, GeneratorError>;
    async fn validate_options(&self, options: &GenerateOptions) -> Result<(), GeneratorError>;
    fn supported_templates(&self) -> Vec<&'static str>;
}

// ✅ 异步实现
pub struct JavaGenerator {
    template_registry: Arc<TemplateRegistry>,
    java_cli: Arc<JavaCli>,
}

#[async_trait::async_trait]
impl ProjectGenerator for JavaGenerator {
    async fn generate(&self, options: GenerateOptions) -> Result<GenerateResult, GeneratorError> {
        // 验证选项
        self.validate_options(&options).await?;
        
        // 异步处理模板
        let template = self.template_registry
            .load_template(&options.template)
            .await?;
        
        // 异步生成项目
        let result = tokio::task::spawn_blocking(move || {
            // CPU 密集型操作
            template.render(options)
        }).await??;
        
        Ok(result)
    }
}
```

## 🔧 性能优化策略

### 1. 零拷贝优化 (Zero-Copy Optimization)
```rust
// ✅ 使用 Cow 避免不必要的克隆
use std::borrow::Cow;

pub fn process_template<'a>(
    template: &'a str,
    variables: &HashMap<String, String>
) -> Cow<'a, str> {
    if variables.is_empty() {
        Cow::Borrowed(template)  // 零拷贝
    } else {
        Cow::Owned(render_template(template, variables))  // 必要时拷贝
    }
}
```

### 2. 内存池模式 (Memory Pool Pattern)
```rust
// ✅ 复用昂贵的资源
pub struct TemplateEngine {
    compiled_templates: Arc<RwLock<HashMap<String, CompiledTemplate>>>,
    template_cache: Arc<Mutex<LruCache<String, String>>>,
}

impl TemplateEngine {
    pub async fn render(&self, template_name: &str, context: &Context) -> Result<String> {
        // 尝试从缓存获取
        if let Some(cached) = self.template_cache.lock().await.get(template_name) {
            return Ok(cached.clone());
        }
        
        // 编译并缓存
        let compiled = self.compile_template(template_name).await?;
        let result = compiled.render(context)?;
        
        self.template_cache.lock().await.put(template_name.to_string(), result.clone());
        Ok(result)
    }
}
```

### 3. 并发处理模式 (Concurrency Patterns)
```rust
// ✅ 并行处理多个任务
pub async fn generate_multiple_files(
    files: Vec<FileTemplate>
) -> Result<Vec<GeneratedFile>, GeneratorError> {
    use futures::stream::{self, StreamExt};
    
    let results = stream::iter(files)
        .map(|file_template| async move {
            tokio::task::spawn_blocking(move || {
                file_template.generate()
            }).await?
        })
        .buffer_unordered(4)  // 限制并发数
        .collect::<Vec<_>>()
        .await;
    
    results.into_iter().collect()
}
```

## 📋 代码质量检查清单

### ✅ 编译时检查
- [ ] 所有 `unwrap()` 都有充分理由
- [ ] 使用 `#[must_use]` 标记重要的返回值
- [ ] 所有公开接口都有文档注释
- [ ] 错误类型实现了 `std::error::Error`

### ✅ 运行时检查
- [ ] 所有异步操作都有超时处理
- [ ] 资源清理逻辑完整
- [ ] 内存使用在合理范围内
- [ ] 并发安全性验证

### ✅ 接口设计
- [ ] napi 导出的类型都是 JavaScript 兼容的
- [ ] 异步函数返回 `Promise`
- [ ] 错误信息对 JavaScript 用户友好
- [ ] TypeScript 类型定义准确

## 🎯 架构决策记录模板

```markdown
# ADR-001: 选择 napi-rs 作为 Rust-Node.js 集成方案

## 状态
已接受

## 上下文
需要将 Rust 核心逻辑暴露给 Node.js 环境使用。

## 决策
使用 napi-rs 而不是 wasm-pack 或 FFI。

## 理由
- 性能优于 WASM
- 类型安全的接口生成
- 成熟的生态系统
- 良好的 TypeScript 支持

## 后果
- 需要为每个平台编译原生模块
- 增加了构建复杂性
- 获得了最佳的运行时性能
```

## 🚀 实施建议

1. **从小做起**: 先实现核心功能，再逐步优化
2. **测试驱动**: 每个模块都要有完整的测试覆盖
3. **文档优先**: API 设计时就要考虑文档的清晰性
4. **性能监控**: 建立基准测试，持续监控性能
5. **错误友好**: 提供清晰、可操作的错误信息

---

**记住**: 好的 Rust 架构不仅要利用语言特性，更要服务于业务目标！