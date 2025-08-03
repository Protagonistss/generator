//! 错误处理模块
//! 定义项目中使用的错误类型

use thiserror::Error;

/// 项目生成器错误类型
#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Invalid project name: {0}")]
    InvalidProjectName(String),

    #[error("Java environment error: {0}")]
    JavaEnvironment(String),

    #[error("Template processing error: {0}")]
    TemplateProcessing(String),

    #[error("File operation error: {0}")]
    FileOperation(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("External command failed: {0}")]
    ExternalCommand(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Template engine error: {0}")]
    TemplateEngine(#[from] handlebars::RenderError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// 项目生成器结果类型
pub type Result<T> = std::result::Result<T, GeneratorError>;

/// 将GeneratorError转换为napi::Error
impl From<GeneratorError> for napi::Error {
    fn from(err: GeneratorError) -> Self {
        napi::Error::from_reason(err.to_string())
    }
}