use std::{fs, path::Path, time::Duration};

use anyhow::{bail, Context, Result};
use chrono::{DateTime, Local};
use clap::ValueEnum;
use indicatif::{ProgressBar, ProgressStyle};

/// 日期模式 enum类型，包含3个值 y,m,d
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum DateMode {
    /// 年
    Y,
    /// 月
    M,
    /// 日
    D,
}

pub fn recursive_walk(to: &std::path::Path, mode: &DateMode) -> Result<()> {
    // 计数器，记录当前处理了多少个文件
    let mut counter = 0;
    // 进度条，用于显示已经处理了多少个文件，当前正在处理什么文件
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message("开始拷贝文件...");
    // for entry in walkdir::WalkDir::new(".")
    //     .into_iter()
    //     .filter_map(|e| e.ok())
    for entry in walkdir::WalkDir::new(".") {
        let entry = entry.unwrap();
        // 判断entry是否文件
        if entry.file_type().is_file() {
            counter += 1;
            let file_metadata = entry.metadata().unwrap();
            let modified_time = file_metadata.modified().unwrap();
            let datetime: DateTime<Local> = modified_time.into();
            let dir_name = dirname(to, mode, datetime)?;
            cratedir(&dir_name)?;

            // 拷贝文件到路径为dir_name的目录
            let target_path = Path::new(&dir_name).join(entry.file_name());
            // println!("正在拷贝：{:?} 到： {:?}", &entry.path(), &target_path);
            pb.set_message(format!(
                "已处理数量：{:?}，正在拷贝：{:?} 到： {:?}",
                counter,
                &entry.path(),
                &target_path
            ));
            fs::copy(&entry.path(), &target_path)
                .with_context(|| format!("Failed to copy file : {}", &dir_name))?;
        }
    }
    pb.finish_with_message("Done!");
    Ok(())
}

// 创建归档的文件夹
// 文件夹已存在则不做处理
#[warn(dead_code)]
fn cratedir(dirpath: &String) -> Result<()> {
    let path = Path::new(&dirpath);

    if !path.exists() {
        match fs::create_dir_all(path) {
            // Ok(_) => println!("Directory created: {}", dirpath),
            // Err(e) => eprintln!("Failed to create directory: {}", e),
            Ok(_) => Ok(()),
            Err(e) => bail!("Failed to create directory: {}", e),
        }
    } else {
        // println!("Directory already exists: {}", dirpath);
        Ok(())
    }
}

// 函数dirname，根据参数mode以及参数file的日期，返回字符串。Y返回YYYY，M返回YYYY/MM，D返回YYYY/MM/DD
fn dirname(to: &std::path::Path, mode: &DateMode, datetime: DateTime<Local>) -> Result<String> {
    let name = match mode {
        DateMode::Y => datetime.format("%Y").to_string(),
        DateMode::M => datetime.format("%Y/%m").to_string(),
        DateMode::D => datetime.format("%Y/%m/%d").to_string(),
    };

    //获取当前路径并将变量name添加一起
    let target_path = to.join(name);
    Ok(target_path.to_str().unwrap().to_string())
}
