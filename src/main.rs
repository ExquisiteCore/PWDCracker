use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct PersonalInfo {
    name: String,
    surname: String,
    nickname: String,
    birth_year: String,
    birth_month: String,
    birth_day: String,
    phone: String,
    email: String,
    pet_name: String,
    favorite_number: String,
    company: String,
    school: String,
}

impl PersonalInfo {
    fn new() -> Self {
        PersonalInfo {
            name: String::new(),
            surname: String::new(),
            nickname: String::new(),
            birth_year: String::new(),
            birth_month: String::new(),
            birth_day: String::new(),
            phone: String::new(),
            email: String::new(),
            pet_name: String::new(),
            favorite_number: String::new(),
            company: String::new(),
            school: String::new(),
        }
    }
}

struct PasswordGenerator {
    info: PersonalInfo,
    passwords: HashSet<String>,
}

impl PasswordGenerator {
    fn new(info: PersonalInfo) -> Self {
        PasswordGenerator {
            info,
            passwords: HashSet::new(),
        }
    }

    fn generate_passwords(&mut self) {
        // 基础信息组合
        self.add_basic_combinations();
        
        // 数字组合
        self.add_number_combinations();
        
        // 常见弱密码模式
        self.add_common_patterns();
        
        // 键盘模式
        self.add_keyboard_patterns();
        
        // 特殊字符组合
        self.add_special_combinations();
    }

    fn add_basic_combinations(&mut self) {
        let fields = vec![
            &self.info.name,
            &self.info.surname,
            &self.info.nickname,
            &self.info.pet_name,
            &self.info.company,
            &self.info.school,
        ];

        // 单独字段
        for field in &fields {
            if !field.is_empty() {
                self.passwords.insert(field.to_lowercase());
                self.passwords.insert(field.to_string());
                self.passwords.insert(capitalize_first(field));
            }
        }

        // 两字段组合
        for i in 0..fields.len() {
            for j in i + 1..fields.len() {
                if !fields[i].is_empty() && !fields[j].is_empty() {
                    self.passwords.insert(format!("{}{}", fields[i].to_lowercase(), fields[j].to_lowercase()));
                    self.passwords.insert(format!("{}{}", fields[i], fields[j]));
                    self.passwords.insert(format!("{}{}", capitalize_first(fields[i]), capitalize_first(fields[j])));
                }
            }
        }
    }

    fn add_number_combinations(&mut self) {
        let base_words = vec![
            &self.info.name,
            &self.info.surname,
            &self.info.nickname,
            &self.info.pet_name,
        ];

        let numbers = vec![
            &self.info.birth_year,
            &self.info.birth_month,
            &self.info.birth_day,
            &self.info.favorite_number,
            &self.info.phone[self.info.phone.len().saturating_sub(4)..], // 后4位
        ];

        for word in &base_words {
            if !word.is_empty() {
                for num in &numbers {
                    if !num.is_empty() {
                        self.passwords.insert(format!("{}{}", word.to_lowercase(), num));
                        self.passwords.insert(format!("{}{}", word, num));
                        self.passwords.insert(format!("{}{}", num, word.to_lowercase()));
                        self.passwords.insert(format!("{}{}", num, word));
                    }
                }
                
                // 常见数字组合
                let common_nums = vec!["123", "321", "666", "888", "999", "000", "111", "222"];
                for num in &common_nums {
                    self.passwords.insert(format!("{}{}", word.to_lowercase(), num));
                    self.passwords.insert(format!("{}{}", word, num));
                }
            }
        }

        // 生日组合
        if !self.info.birth_year.is_empty() && !self.info.birth_month.is_empty() && !self.info.birth_day.is_empty() {
            let birth_combinations = vec![
                format!("{}{}{}", self.info.birth_year, self.info.birth_month, self.info.birth_day),
                format!("{}{}", self.info.birth_month, self.info.birth_day),
                format!("{}{}", self.info.birth_day, self.info.birth_month),
                format!("{}", &self.info.birth_year[2..]), // 年份后两位
            ];
            
            for birth in birth_combinations {
                self.passwords.insert(birth.clone());
                for word in &base_words {
                    if !word.is_empty() {
                        self.passwords.insert(format!("{}{}", word.to_lowercase(), birth));
                        self.passwords.insert(format!("{}{}", birth, word.to_lowercase()));
                    }
                }
            }
        }
    }

    fn add_common_patterns(&mut self) {
        let common_passwords = vec![
            "password", "123456", "123456789", "qwerty", "abc123",
            "password123", "admin", "root", "user", "guest",
            "welcome", "login", "pass", "test", "demo",
            "12345678", "1234567890", "qwertyuiop", "asdfghjkl",
            "zxcvbnm", "iloveyou", "princess", "rockyou",
        ];

        for pwd in common_passwords {
            self.passwords.insert(pwd.to_string());
            self.passwords.insert(capitalize_first(pwd));
            self.passwords.insert(pwd.to_uppercase());
        }

        // 与个人信息结合的常见模式
        let base_words = vec![&self.info.name, &self.info.surname, &self.info.nickname];
        for word in &base_words {
            if !word.is_empty() {
                self.passwords.insert(format!("{}123", word.to_lowercase()));
                self.passwords.insert(format!("{}321", word.to_lowercase()));
                self.passwords.insert(format!("{}666", word.to_lowercase()));
                self.passwords.insert(format!("{}888", word.to_lowercase()));
                self.passwords.insert(format!("i_love_{}", word.to_lowercase()));
                self.passwords.insert(format!("my_{}", word.to_lowercase()));
            }
        }
    }

    fn add_keyboard_patterns(&mut self) {
        let keyboard_patterns = vec![
            "qwerty", "asdfgh", "zxcvbn", "qwertyui", "asdfghjk",
            "zxcvbnm", "1qaz2wsx", "qazwsx", "123qwe", "qwe123",
            "asd123", "zxc123", "147258", "159357", "741852",
        ];

        for pattern in keyboard_patterns {
            self.passwords.insert(pattern.to_string());
            self.passwords.insert(pattern.to_uppercase());
            self.passwords.insert(capitalize_first(pattern));
        }
    }

    fn add_special_combinations(&mut self) {
        let base_words = vec![&self.info.name, &self.info.surname, &self.info.nickname];
        let special_chars = vec!["!", "@", "#", "$", "%", "*", ".", "_"];

        for word in &base_words {
            if !word.is_empty() {
                for special in &special_chars {
                    self.passwords.insert(format!("{}{}", word.to_lowercase(), special));
                    self.passwords.insert(format!("{}{}", special, word.to_lowercase()));
                    self.passwords.insert(format!("{}{}{}", word.to_lowercase(), special, "123"));
                }
            }
        }
    }

    fn get_passwords(&self) -> Vec<String> {
        let mut passwords: Vec<String> = self.passwords.iter().cloned().collect();
        passwords.sort();
        passwords
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if !chars.is_empty() {
        chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
    }
    chars.into_iter().collect()
}

fn collect_personal_info() -> PersonalInfo {
    let mut info = PersonalInfo::new();
    
    println!("=== 弱密码生成器 ===");
    println!("请输入个人信息（可选项可直接回车跳过）:\n");

    print!("姓名: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.name).unwrap();
    info.name = info.name.trim().to_string();

    print!("姓氏: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.surname).unwrap();
    info.surname = info.surname.trim().to_string();

    print!("昵称/网名: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.nickname).unwrap();
    info.nickname = info.nickname.trim().to_string();

    print!("出生年份 (如: 1990): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.birth_year).unwrap();
    info.birth_year = info.birth_year.trim().to_string();

    print!("出生月份 (如: 05): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.birth_month).unwrap();
    info.birth_month = info.birth_month.trim().to_string();

    print!("出生日期 (如: 15): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.birth_day).unwrap();
    info.birth_day = info.birth_day.trim().to_string();

    print!("手机号码: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.phone).unwrap();
    info.phone = info.phone.trim().to_string();

    print!("邮箱地址: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.email).unwrap();
    info.email = info.email.trim().to_string();

    print!("宠物名字: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.pet_name).unwrap();
    info.pet_name = info.pet_name.trim().to_string();

    print!("幸运数字: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.favorite_number).unwrap();
    info.favorite_number = info.favorite_number.trim().to_string();

    print!("公司名称: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.company).unwrap();
    info.company = info.company.trim().to_string();

    print!("学校名称: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut info.school).unwrap();
    info.school = info.school.trim().to_string();

    info
}

fn main() {
    let matches = Command::new("PWD Cracker")
        .version("0.1.0")
        .author("Your Name")
        .about("根据个人信息生成常见弱密码")
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("交互式输入个人信息")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("输出文件路径")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("limit")
                .short('l')
                .long("limit")
                .value_name("NUMBER")
                .help("限制输出密码数量")
                .action(clap::ArgAction::Set),
        )
        .get_matches();

    let info = if matches.get_flag("interactive") {
        collect_personal_info()
    } else {
        println!("使用 -i 或 --interactive 参数进入交互模式");
        return;
    };

    println!("\n正在生成密码...");
    
    let mut generator = PasswordGenerator::new(info);
    generator.generate_passwords();
    
    let mut passwords = generator.get_passwords();
    
    // 限制输出数量
    if let Some(limit_str) = matches.get_one::<String>("limit") {
        if let Ok(limit) = limit_str.parse::<usize>() {
            passwords.truncate(limit);
        }
    }

    println!("\n生成了 {} 个可能的弱密码:\n", passwords.len());
    
    // 输出到文件或控制台
    if let Some(output_file) = matches.get_one::<String>("output") {
        match std::fs::write(output_file, passwords.join("\n")) {
            Ok(_) => println!("密码已保存到文件: {}", output_file),
            Err(e) => eprintln!("保存文件失败: {}", e),
        }
    } else {
        for (i, password) in passwords.iter().enumerate() {
            println!("{:4}. {}", i + 1, password);
        }
    }

    println!("\n⚠️  警告: 这些密码仅用于安全测试目的，请勿用于非法用途！");
    println!("💡 建议: 使用强密码并启用双因素认证来保护您的账户安全。");
}
