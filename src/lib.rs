//! Project Generator - napi-rs 模块
//! 提供项目生成功能的 Node.js 绑定

use napi_derive::napi;
use serde::{Deserialize, Serialize};

// 模块声明
pub mod error;
pub mod utils;
pub mod templates;

// 重新导出错误类型
pub use error::{GeneratorError, Result};


/// 项目生成选项
#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateOptions {
    /// 项目名称
    pub name: String,
    /// 项目类型 (java, vue, react)
    pub project_type: String,
    /// 模板名称
    pub template: Option<String>,
    /// 输出路径
    pub output_path: Option<String>,
    /// 额外变量
    pub variables: Option<std::collections::HashMap<String, String>>,
}

/// 项目生成结果
#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateResult {
    /// 是否成功
    pub success: bool,
    /// 生成的文件列表
    pub files: Vec<String>,
    /// 消息
    pub message: Option<String>,
}

/// 生成项目
#[napi]
pub fn generate_project(options: GenerateOptions) -> napi::Result<GenerateResult> {
    // TODO: 实现项目生成逻辑
    match templates::generate_project_from_template(options) {
        Ok(result) => Ok(result),
        Err(e) => Err(napi::Error::from_reason(e.to_string())),
    }
}

/// 列出可用模板
#[napi]
pub fn list_templates(project_type: String) -> napi::Result<Vec<String>> {
    templates::list_templates_by_type(&project_type)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

/// 获取模板信息
#[napi]
pub fn get_template_info(project_type: String, template: String) -> napi::Result<String> {
    templates::get_template_info(&project_type, &template)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_template_info() {
        // To show test output, use:
        // cargo test -- --nocapture
        println!("This will be shown when running with --nocapture");
        // Or use env var:
        // RUST_TEST_NOCAPTURE=1 cargo test
        println!("This will also be shown with RUST_TEST_NOCAPTURE=1");

    }
}