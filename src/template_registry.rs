//! 模板注册表管理模块
//! 支持多种模板源：Git、HTTP、npm、本地文件

use crate::{GeneratorError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

/// 模板注册表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRegistryConfig {
    pub registries: Vec<TemplateRegistry>,
    pub cache_dir: PathBuf,
    pub cache_ttl: u64, // 缓存时间（秒）
}

/// 模板注册表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRegistry {
    pub name: String,
    pub source: TemplateSource,
    pub enabled: bool,
    pub priority: u32, // 优先级，数字越小优先级越高
}

/// 模板源类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateSource {
    #[serde(rename = "local")]
    Local { 
        path: PathBuf 
    },
    
    #[serde(rename = "git")]
    Git { 
        url: String,
        branch: Option<String>,
        subfolder: Option<String>,
        auth: Option<GitAuth>,
    },
    
    #[serde(rename = "http")]
    Http { 
        url: String,
        checksum: Option<String>,
        auth: Option<HttpAuth>,
    },
    
    #[serde(rename = "npm")]
    Npm { 
        package: String,
        version: String,
        registry: Option<String>,
    },
}

/// Git 认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitAuth {
    pub username: Option<String>,
    pub token: Option<String>,
}

/// HTTP 认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpAuth {
    pub bearer_token: Option<String>,
    pub basic_auth: Option<(String, String)>,
}

/// 模板元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub project_type: String,
    pub variables: Vec<TemplateVariable>,
    pub dependencies: Vec<String>,
    pub tags: Vec<String>,
}

/// 模板变量定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub default: Option<String>,
    pub required: bool,
    pub var_type: VariableType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    String,
    Boolean,
    Number,
    Choice { options: Vec<String> },
}

/// 模板管理器
pub struct TemplateManager {
    config: TemplateRegistryConfig,
    cache: HashMap<String, CachedTemplate>,
}

/// 缓存的模板
#[derive(Debug, Clone)]
pub struct CachedTemplate {
    pub metadata: TemplateMetadata,
    pub path: PathBuf,
    pub cached_at: std::time::SystemTime,
}

impl TemplateManager {
    /// 创建新的模板管理器
    pub fn new(config: TemplateRegistryConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    /// 列出所有可用模板
    pub async fn list_templates(&mut self, project_type: Option<&str>) -> Result<Vec<TemplateMetadata>> {
        let mut templates = Vec::new();
        
        // 按优先级排序注册表
        let mut registries = self.config.registries.clone();
        registries.sort_by_key(|r| r.priority);
        
        for registry in registries {
            if !registry.enabled {
                continue;
            }
            
            match self.load_templates_from_registry(&registry).await {
                Ok(mut registry_templates) => {
                    // 过滤项目类型
                    if let Some(pt) = project_type {
                        registry_templates.retain(|t| t.project_type == pt);
                    }
                    templates.extend(registry_templates);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load templates from registry '{}': {}", 
                             registry.name, e);
                }
            }
        }
        
        Ok(templates)
    }

    /// 获取特定模板
    pub async fn get_template(&mut self, project_type: &str, template_name: &str) -> Result<PathBuf> {
        let cache_key = format!("{}:{}", project_type, template_name);
        
        // 检查缓存
        if let Some(cached) = self.cache.get(&cache_key) {
            if !self.is_cache_expired(&cached) {
                return Ok(cached.path.clone());
            }
        }
        
        // 从注册表加载
        for registry in &self.config.registries {
            if !registry.enabled {
                continue;
            }
            
            if let Ok(template_path) = self.load_template_from_registry(
                registry, project_type, template_name
            ).await {
                // 更新缓存
                if let Ok(metadata) = self.load_template_metadata(&template_path).await {
                    self.cache.insert(cache_key, CachedTemplate {
                        metadata,
                        path: template_path.clone(),
                        cached_at: std::time::SystemTime::now(),
                    });
                }
                
                return Ok(template_path);
            }
        }
        
        Err(GeneratorError::TemplateNotFound(format!("{}:{}", project_type, template_name)))
    }

    /// 从注册表加载模板列表
    async fn load_templates_from_registry(&self, registry: &TemplateRegistry) -> Result<Vec<TemplateMetadata>> {
        match &registry.source {
            TemplateSource::Local { path } => {
                self.load_local_templates(path).await
            }
            TemplateSource::Git { url, branch, subfolder, auth } => {
                self.load_git_templates(url, branch.as_deref(), subfolder.as_deref(), auth).await
            }
            TemplateSource::Http { url, checksum, auth } => {
                self.load_http_templates(url, checksum.as_deref(), auth).await
            }
            TemplateSource::Npm { package, version, registry } => {
                self.load_npm_templates(package, version, registry.as_deref()).await
            }
        }
    }

    /// 从注册表加载特定模板
    async fn load_template_from_registry(
        &self, 
        registry: &TemplateRegistry, 
        project_type: &str, 
        template_name: &str
    ) -> Result<PathBuf> {
        // TODO: 实现具体的加载逻辑
        todo!("实现模板加载逻辑")
    }

    /// 加载本地模板
    async fn load_local_templates(&self, path: &PathBuf) -> Result<Vec<TemplateMetadata>> {
        // TODO: 扫描本地目录，加载模板元数据
        todo!("实现本地模板加载")
    }

    /// 加载 Git 模板
    async fn load_git_templates(
        &self, 
        url: &str, 
        branch: Option<&str>, 
        subfolder: Option<&str>,
        auth: &Option<GitAuth>
    ) -> Result<Vec<TemplateMetadata>> {
        // TODO: 克隆或更新 Git 仓库，加载模板
        todo!("实现 Git 模板加载")
    }

    /// 加载 HTTP 模板
    async fn load_http_templates(
        &self, 
        url: &str, 
        checksum: Option<&str>,
        auth: &Option<HttpAuth>
    ) -> Result<Vec<TemplateMetadata>> {
        // TODO: 下载并解压模板包
        todo!("实现 HTTP 模板加载")
    }

    /// 加载 npm 模板
    async fn load_npm_templates(
        &self, 
        package: &str, 
        version: &str,
        registry: Option<&str>
    ) -> Result<Vec<TemplateMetadata>> {
        // TODO: 从 npm 下载模板包
        todo!("实现 npm 模板加载")
    }

    /// 加载模板元数据
    async fn load_template_metadata(&self, template_path: &PathBuf) -> Result<TemplateMetadata> {
        let metadata_path = template_path.join("template.json");
        let content = fs::read_to_string(metadata_path).await?;
        let metadata: TemplateMetadata = serde_json::from_str(&content)?;
        Ok(metadata)
    }

    /// 检查缓存是否过期
    fn is_cache_expired(&self, cached: &CachedTemplate) -> bool {
        if let Ok(elapsed) = cached.cached_at.elapsed() {
            elapsed.as_secs() > self.config.cache_ttl
        } else {
            true
        }
    }
}

/// 默认配置
impl Default for TemplateRegistryConfig {
    fn default() -> Self {
        Self {
            registries: vec![
                TemplateRegistry {
                    name: "local".to_string(),
                    source: TemplateSource::Local {
                        path: PathBuf::from("./templates"),
                    },
                    enabled: true,
                    priority: 0,
                },
            ],
            cache_dir: PathBuf::from("./.template_cache"),
            cache_ttl: 3600, // 1小时
        }
    }
}