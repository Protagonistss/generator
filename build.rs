fn main() {
    // 告诉 Cargo 在配置文件变化时重新构建
    println!("cargo:rerun-if-changed=config/");
    
    // napi-rs 构建配置
    napi_build::setup();
}