//! Build script for napi-rs
//! 处理构建时的配置和资源

fn main() {
    // 告诉 Cargo 在模板文件变化时重新构建
    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:rerun-if-changed=config/");
    
    // napi-rs 构建配置
    napi_build::setup();
}