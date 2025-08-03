//! 工具函数模块
//! 提供文件系统操作、路径处理等通用功能

use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;

/// 跨平台路径规范化
pub fn normalize_path(path: &str) -> PathBuf {
    let normalized = if cfg!(windows) {
        path.replace('/', "\\")
    } else {
        path.replace('\\', "/")
    };
    PathBuf::from(normalized)
}

/// 确保目录存在，如果不存在则创建
pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 复制文件到目标位置
pub fn copy_file(src: &Path, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        ensure_dir_exists(parent)?;
    }
    fs::copy(src, dest)?;
    Ok(())
}

/// 递归复制目录
pub fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    ensure_dir_exists(dest)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            copy_file(&src_path, &dest_path)?;
        }
    }
    
    Ok(())
}

/// 替换文件中的变量
pub fn replace_variables_in_file(file_path: &Path, variables: &std::collections::HashMap<String, String>) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let mut new_content = content;
    
    for (key, value) in variables {
        let placeholder = format!("{{{{{}}}}}", key);
        new_content = new_content.replace(&placeholder, value);
    }
    
    fs::write(file_path, new_content)?;
    Ok(())
}

/// 获取当前可执行文件的目录
pub fn get_exe_dir() -> Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    Ok(exe_path.parent().unwrap().to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_normalize_path() {
        let path = "src/lib.rs";
        let normalized = normalize_path(path);
        
        if cfg!(windows) {
            assert_eq!(normalized, PathBuf::from("src\\lib.rs"));
        } else {
            assert_eq!(normalized, PathBuf::from("src/lib.rs"));
        }
    }
    
    #[test]
    fn test_ensure_dir_exists() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test");
        
        assert!(!test_dir.exists());
        ensure_dir_exists(&test_dir).unwrap();
        assert!(test_dir.exists());
    }
}