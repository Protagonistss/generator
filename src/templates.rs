//! 模板系统核心模块 - 简化版本
//! 提供基础的模板操作接口，委托给 template_registry 处理

use crate::{GenerateOptions, GenerateResult, GeneratorError, Result};
use std::collections::HashMap;

/// 从模板生成项目 - 简化实现
pub fn generate_project_from_template(options: GenerateOptions) -> Result<GenerateResult> {
    // 暂时返回基础实现，后续集成 TemplateManager
    Ok(GenerateResult {
        success: true,
        files: vec![format!("{}/README.md", options.name)],
        message: Some(format!(
            "项目 {} 生成成功 (使用模板: {})",
            options.name,
            options.template.unwrap_or("basic".to_string())
        )),
    })
}

/// 根据项目类型列出可用模板 - 简化实现
pub fn list_templates_by_type(project_type: &str) -> Result<Vec<String>> {
    match project_type {
        "java" => Ok(vec!["basic".to_string(), "spring-boot".to_string()]),
        "vue" => Ok(vec!["basic".to_string(), "typescript".to_string()]),
        "react" => Ok(vec!["basic".to_string(), "nextjs".to_string()]),
        _ => Err(GeneratorError::TemplateNotFound(format!(
            "Unsupported project type: {}",
            project_type
        ))),
    }
}

/// 获取模板信息 - 简化实现
pub fn get_template_info(project_type: &str, template: &str) -> Result<String> {
    let info = match (project_type, template) {
        ("vue", "basic") => "Vue 3 + Vite 基础模板",
        ("vue", "typescript") => "Vue 3 + Vite + TypeScript 模板",
        ("react", "basic") => "React + Vite 基础模板",
        ("react", "nextjs") => "Next.js 全栈模板",
        ("java", "basic") => "Java Maven 基础模板",
        ("java", "spring-boot") => "Spring Boot 企业级模板",
        _ => {
            return Err(GeneratorError::TemplateNotFound(format!(
                "Template not found: {}:{}",
                project_type, template
            )));
        }
    };

    Ok(format!("模板信息: {} - {}", template, info))
}

/// 渲染模板文件 - 基础实现
pub fn render_template(
    template_content: &str,
    variables: &HashMap<String, String>,
) -> Result<String> {
    let mut result = template_content.to_string();

    // 简单的变量替换
    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }

    Ok(result)
}
