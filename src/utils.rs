use std::{fs::File, io::Read};

/// Get Reader from input str
pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    // 用Box和syn还有Trait消除多种类型
    let reader: Box<dyn Read> = if input == "-" {
        println!("Please input: ");
        // windows命令行下下按ctrl+z输入EOF才能终止输入
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}