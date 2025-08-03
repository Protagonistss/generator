<execution>
  <constraint>
    ## Rust 开发的客观技术限制
    - **借用检查器约束**: 必须满足所有权和生命周期规则，无法绕过编译时检查
    - **napi-rs 类型限制**: 只能导出 napi 支持的类型，复杂类型需要序列化处理
    - **异步运行时约束**: tokio 与 Node.js 事件循环的集成限制
    - **跨平台编译限制**: 不同目标平台的工具链和依赖差异
    - **内存安全要求**: 不允许使用 unsafe 代码，除非有充分理由和安全保证
  </constraint>

  <rule>
    ## Rust 开发强制性规则
    - **所有权清晰**: 每个值都有明确的所有者，避免悬垂指针和内存泄漏
    - **错误处理完整**: 使用 Result<T, E> 处理所有可能的错误情况
    - **类型安全优先**: 利用类型系统在编译时捕获尽可能多的错误
    - **异步一致性**: 在异步上下文中保持一致的错误处理和资源管理
    - **接口稳定性**: napi 导出的接口必须保持向后兼容
    - **文档完整性**: 所有公开接口必须有完整的文档和使用示例
  </rule>

  <guideline>
    ## Rust 开发指导原则
    - **零成本抽象**: 优先使用编译时优化，避免运行时开销
    - **组合优于继承**: 使用 trait 和泛型实现代码复用
    - **显式优于隐式**: 明确表达意图，避免隐式类型转换和行为
    - **渐进式优化**: 先保证正确性，再进行性能优化
    - **生态集成**: 优先使用成熟的 crate，避免重复造轮子
    - **测试驱动**: 编写测试用例验证功能正确性和性能表现
  </guideline>

  <process>
    ## napi-rs 项目开发流程
    
    ### Step 1: 项目初始化
    ```bash
    # 初始化 Rust 项目
    cargo init --lib
    
    # 配置 Cargo.toml
    [lib]
    crate-type = ["cdylib"]
    
    [dependencies]
    napi = "2"
    napi-derive = "2"
    tokio = { version = "1", features = ["full"] }
    ```
    
    ### Step 2: 核心模块设计
    ```rust
    // src/lib.rs - napi 入口点
    #[macro_use]
    extern crate napi_derive;
    
    // 导出主要接口
    #[napi]
    pub async fn main_function(input: String) -> napi::Result<String> {
        // 实现核心逻辑
    }
    ```
    
    ### Step 3: 类型定义和映射
    ```rust
    // 定义输入输出结构
    #[napi(object)]
    pub struct InputOptions {
        pub name: String,
        pub template: Option<String>,
    }
    
    #[napi(object)]
    pub struct GenerateResult {
        pub success: bool,
        pub files: Vec<String>,
        pub message: Option<String>,
    }
    ```
    
    ### Step 4: 异步处理实现
    ```rust
    #[napi]
    pub async fn generate_project(options: InputOptions) -> napi::Result<GenerateResult> {
        // 使用 tokio 处理异步操作
        let result = tokio::task::spawn_blocking(move || {
            // CPU 密集型操作
        }).await?;
        
        Ok(result)
    }
    ```
    
    ### Step 5: 错误处理策略
    ```rust
    use anyhow::{Context, Result};
    use napi::Error;
    
    fn convert_error(err: anyhow::Error) -> napi::Error {
        Error::new(
            napi::Status::GenericFailure,
            format!("{:#}", err)
        )
    }
    ```
    
    ### Step 6: 构建配置
    ```json
    // package.json
    {
      "scripts": {
        "build": "napi build --platform --release",
        "build:debug": "napi build --platform"
      }
    }
    ```
    
    ### Step 7: 测试和验证
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[tokio::test]
        async fn test_generate_project() {
            let options = InputOptions {
                name: "test-project".to_string(),
                template: Some("basic".to_string()),
            };
            
            let result = generate_project(options).await;
            assert!(result.is_ok());
        }
    }
    ```
  </process>

  <criteria>
    ## Rust 代码质量评价标准

    ### 正确性指标
    - ✅ 编译通过，无警告
    - ✅ 所有测试用例通过
    - ✅ 内存安全，无泄漏
    - ✅ 线程安全，无数据竞争

    ### 性能指标
    - ✅ 启动时间 < 100ms
    - ✅ 内存使用合理
    - ✅ CPU 使用效率高
    - ✅ 异步操作响应及时

    ### 可维护性
    - ✅ 代码结构清晰
    - ✅ 文档完整准确
    - ✅ 错误信息有用
    - ✅ 接口设计合理

    ### 集成质量
    - ✅ napi 接口稳定
    - ✅ TypeScript 类型正确
    - ✅ 跨平台兼容
    - ✅ 构建流程顺畅
  </criteria>
</execution>