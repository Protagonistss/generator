//! Java CLI 集成模块
//! 负责Java环境检测和Java CLI jar包调用

use std::process::Command;
use std::env;
use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use crate::utils::get_exe_dir;

/// Java环境信息
#[derive(Debug, Clone)]
pub struct JavaEnvironment {
    pub java_path: String,
    pub version: String,
}

/// 检测Java环境
pub fn detect_java() -> Result<JavaEnvironment> {
    // 首先检查JAVA_HOME环境变量
    if let Ok(java_home) = env::var("JAVA_HOME") {
        let java_path = if cfg!(windows) {
            format!("{}/bin/java.exe", java_home)
        } else {
            format!("{}/bin/java", java_home)
        };
        
        if Path::new(&java_path).exists() {
            if let Ok(version) = get_java_version(&java_path) {
                return Ok(JavaEnvironment {
                    java_path,
                    version,
                });
            }
        }
    }
    
    // 尝试从PATH中查找java
    let java_cmd = if cfg!(windows) { "java.exe" } else { "java" };
    
    // 在Windows上使用where命令，在Unix系统上使用which命令
    let which_cmd = if cfg!(windows) { "where" } else { "which" };
    
    match Command::new(which_cmd).arg(java_cmd).output() {
        Ok(output) if output.status.success() => {
            let java_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(version) = get_java_version(&java_path) {
                Ok(JavaEnvironment {
                    java_path,
                    version,
                })
            } else {
                Err(anyhow!("Failed to get Java version"))
            }
        }
        _ => Err(anyhow!("Java not found in PATH or JAVA_HOME. Please install Java and ensure it's in your PATH or set JAVA_HOME environment variable."))
    }
}

/// 获取Java版本信息
fn get_java_version(java_path: &str) -> Result<String> {
    let output = Command::new(java_path)
        .arg("-version")
        .output()?;
    
    if output.status.success() {
        // Java版本信息通常输出到stderr
        let version_output = String::from_utf8_lossy(&output.stderr);
        // 提取版本号（简化处理）
        if let Some(line) = version_output.lines().next() {
            Ok(line.to_string())
        } else {
            Err(anyhow!("Failed to parse Java version"))
        }
    } else {
        Err(anyhow!("Failed to execute java -version"))
    }
}

/// 获取Java CLI jar包路径
pub fn get_java_cli_jar_path() -> Result<PathBuf> {
    let exe_dir = get_exe_dir()?;
    let jar_path = exe_dir.join("assets").join("java-cli.jar");
    
    if jar_path.exists() {
        Ok(jar_path)
    } else {
        Err(anyhow!("Java CLI jar not found at: {}", jar_path.display()))
    }
}

/// 执行Java CLI命令
pub async fn execute_java_cli(args: Vec<String>) -> Result<String> {
    let java_env = detect_java()?;
    let jar_path = get_java_cli_jar_path()?;
    
    let mut cmd = Command::new(&java_env.java_path);
    cmd.arg("-jar")
       .arg(jar_path)
       .args(args);
    
    let output = cmd.output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("Java CLI execution failed: {}", error_msg))
    }
}

/// Java项目生成选项
#[derive(Debug, Clone)]
pub struct JavaProjectOptions {
    pub name: String,
    pub package_name: Option<String>,
    pub group_id: Option<String>,
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    pub output_path: Option<String>,
}

/// 生成Java项目
pub async fn generate_java_project(options: JavaProjectOptions) -> Result<Vec<String>> {
    let mut args = vec![
        "generate".to_string(),
        "--type".to_string(),
        "java".to_string(),
        "--name".to_string(),
        options.name.clone(),
    ];
    
    if let Some(package_name) = &options.package_name {
        args.push("--package".to_string());
        args.push(package_name.clone());
    }
    
    if let Some(group_id) = &options.group_id {
        args.push("--group-id".to_string());
        args.push(group_id.clone());
    }
    
    if let Some(artifact_id) = &options.artifact_id {
        args.push("--artifact-id".to_string());
        args.push(artifact_id.clone());
    }
    
    if let Some(version) = &options.version {
        args.push("--version".to_string());
        args.push(version.clone());
    }
    
    if let Some(output_path) = &options.output_path {
        args.push("--output".to_string());
        args.push(output_path.clone());
    }
    
    let result = execute_java_cli(args).await?;
    
    // 解析结果，返回生成的文件列表
    // 这里需要根据Java CLI的实际输出格式来解析
    let files: Vec<String> = result
        .lines()
        .filter(|line| line.starts_with("Generated:"))
        .map(|line| line.replace("Generated: ", ""))
        .collect();
    
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_java() {
        // 这个测试只在有Java环境的情况下才会通过
        match detect_java() {
            Ok(java_env) => {
                println!("Java found: {} - {}", java_env.java_path, java_env.version);
                assert!(!java_env.java_path.is_empty());
                assert!(!java_env.version.is_empty());
            }
            Err(e) => {
                println!("Java not found: {}", e);
                // 在CI环境中可能没有Java，所以这里不强制失败
            }
        }
    }
}