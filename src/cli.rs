use crate::{GenerateOptions, GenerateResult, GeneratorError, Result};
use std::io::{self, Write};

pub struct GenCli;

impl GenCli {
    pub fn new() -> Self {
        Self
    }

    pub fn run_interactive(&self) -> Result<GenerateResult> {
        // 选择生成方式
        let generation_mode = self.select_generation_mode("请选择生成模式")?;
        println!("{}", generation_mode);

        // TODO 模版源加载
        match generation_mode.as_str() {
            "NOP" => {
                println!("🎯 使用 NOP 平台模式");
                // NOP 模式的特定逻辑
                println!("📋 NOP 平台将使用预定义的模板和配置");
            }
            "CUSTOM" => {
                println!("🎯 使用自定义模式");
                // 自定义模式的逻辑
                println!("📋 自定义模式允许您选择任意模板源");
            }
            _ => {
                println!("❌ 未知的生成模式: {}", generation_mode);
                return Ok(GenerateResult {
                    success: false,
                    files: vec![],
                    message: Some(format!("未知的生成模式: {}", generation_mode)),
                });
            }
        }

        // 2. 选择项目类型
        let project_type = self.select_project_type()?;
        // 1. 获取项目名称
        let project_name = self.get_input("请输入项目名称")?;

        // 3. 选择模板（简化）
        let template = self.select_template(&project_type)?;

        // 4. 确认生成
        println!("\n📋 生成信息:");
        println!("   项目名称: {}", project_name);
        println!("   项目类型: {}", project_type);
        println!("   模板: {}", template);

        if self.confirm("确认生成项目?")? {
            let options = GenerateOptions {
                name: project_name,
                project_type,
                template: Some(template),
                output_path: None,
                variables: None,
            };

            println!("🔄 正在生成项目...");
            let result = crate::templates::generate_project_from_template(options)?;

            if result.success {
                println!("✅ 项目生成成功!");
            }

            Ok(result)
        } else {
            println!("❌ 用户取消生成");
            Ok(GenerateResult {
                success: false,
                files: vec![],
                message: Some("用户取消".to_string()),
            })
        }
    }

    fn select_generation_mode(&self, _prompt: &str) -> Result<String> {
        let types = vec![("NOP", "NOP平台"), ("CUSTOM", "自定义")];

        println!("\n🎯 请选择生成模式:");
        for (i, (name, desc)) in types.iter().enumerate() {
            println!("{}. {} - {}", i + 1, name, desc);
        }

        loop {
            print!("请输入选项 (1-{}): ", types.len());
            io::stdout().flush().map_err(|e| GeneratorError::Io(e))?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| GeneratorError::Io(e))?;

            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice > 0 && choice <= types.len() {
                    return Ok(types[choice - 1].0.to_string());
                }
            }

            println!("❌ 无效选项，请输入 1-{}", types.len());
        }
    }

    /// 获取用户输入
    fn get_input(&self, prompt: &str) -> Result<String> {
        loop {
            print!("{}: ", prompt);
            io::stdout().flush().map_err(|e| GeneratorError::Io(e))?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| GeneratorError::Io(e))?;

            let input = input.trim().to_string();
            if !input.is_empty() {
                return Ok(input);
            }

            println!("❌ 输入不能为空，请重新输入");
        }
    }

    /// 选择项目类型
    fn select_project_type(&self) -> Result<String> {
        let types = vec![
            ("vue", "Vue.js 项目"),
            ("react", "React 项目"),
            ("java", "Java 项目"),
        ];

        println!("\n🎯 请选择项目类型:");
        for (i, (name, desc)) in types.iter().enumerate() {
            println!("{}. {} - {}", i + 1, name, desc);
        }

        loop {
            print!("请输入选项 (1-{}): ", types.len());
            io::stdout().flush().map_err(|e| GeneratorError::Io(e))?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| GeneratorError::Io(e))?;

            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice > 0 && choice <= types.len() {
                    return Ok(types[choice - 1].0.to_string());
                }
            }

            println!("❌ 无效选项，请输入 1-{}", types.len());
        }
    }

    /// 选择模板
    fn select_template(&self, project_type: &str) -> Result<String> {
        // 简化实现：根据项目类型返回默认模板
        let template = match project_type {
            "vue" => "basic",
            "react" => "basic",
            "java" => "spring-boot",
            _ => "basic",
        };

        println!("\n🎨 使用模板: {}", template);
        Ok(template.to_string())
    }

    /// 确认操作
    fn confirm(&self, message: &str) -> Result<bool> {
        print!("{} (Y/n): ", message);
        io::stdout().flush().map_err(|e| GeneratorError::Io(e))?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| GeneratorError::Io(e))?;

        let input = input.trim().to_lowercase();
        Ok(matches!(input.as_str(), "" | "y" | "yes"))
    }

    /// 显示帮助信息
    pub fn show_help(&self) {
        println!("🚀 项目生成器 CLI");
        println!("================");
        println!();
        println!("支持的项目类型:");
        println!("  - vue: Vue.js 项目");
        println!("  - react: React 项目");
        println!("  - java: Java 项目");
        println!();
        println!("使用方法:");
        println!("  运行交互模式，按提示输入信息即可生成项目");
    }
}
