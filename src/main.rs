//! 项目生成器的命令行入口
//! 可以通过 cargo run 直接执行

// 使用库 crate
use generator::{run_simple_cli, show_cli_help};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 项目生成器 - Rust CLI 模式");
    println!("==============================");

    // 检查命令行参数
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "help" | "--help" | "-h" => {
                match show_cli_help() {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        eprintln!("❌ 显示帮助失败: {}", e);
                        return Ok(());
                    }
                }
            }
            "version" | "--version" | "-v" => {
                println!("项目生成器 v0.0.0");
                return Ok(());
            }
            _ => {
                println!("❌ 未知参数: {}", args[1]);
                println!("使用 'cargo run help' 查看帮助");
                return Ok(());
            }
        }
    }

    // 直接调用 lib.rs 中的 run_simple_cli 函数
    match run_simple_cli() {
        Ok(result) => {
            if result.success {
                println!("\n🎉 操作完成!");
                if let Some(message) = result.message {
                    println!("📝 {}", message);
                }
            } else {
                println!("\n⚠️  操作未完成");
            }
        }
        Err(e) => {
            eprintln!("\n❌ 错误: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
