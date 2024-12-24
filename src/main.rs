use std::{fs, path::Path, time::SystemTime};

use chrono::NaiveDate;
use clap::{Parser, Subcommand};
mod dirbydate;
use dirbydate::*;
use walkdir::WalkDir;
use anyhow::{Context, Result};

#[derive(Parser)]
#[clap(name = "tol", version = "0.1", about = "一个工具集")]
// Define the main CLI structure
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

// 定义子命令枚举
#[derive(Subcommand)]
enum Commands {
    /// 递归指定目录所有文件，按照年月日进行整理
    Dirbydate {
        /// 日期模式， 默认值DateMode::M。
        #[arg(long, short, value_enum, default_value_t = DateMode::M)]
        mode: DateMode,

        /// 输出路径
        output_path: String,
    },

    /*
    /// 删除文件子命令
    Del {
        /// 文件路径
        #[arg(default_value_t = String::from("."))]
        file_path: String,

        /// 指定包含字符串
        #[arg(short, long)]
        content: Vec<String>,

        /// 指定日期
        #[arg(long)]
        date: Option<NaiveDate>,

        /// 开始日期
        #[arg(long, requires = "edate")]
        sdate: Option<NaiveDate>,

        /// 结束日期
        #[arg(long, requires = "sdate")]
        edate: Option<NaiveDate>,
    },
    */
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match &args.command {
        Commands::Dirbydate { mode, output_path } => {
            // 将输出路径转换为 Path 对象
            let to = std::path::Path::new(output_path);
            // 调用递归遍历函数
            recursive_walk(to, &mode)?;
            println!("全部拷贝完成！全部拷贝完成！");
        }
        // Commands::Del {
        //     file_path,
        //     content,
        //     date,
        //     sdate,
        //     edate,
        // } => {
        //     // 将输出路径转换为 Path 对象
        //     let path = std::path::Path::new(file_path);
        //     recursive_del(path, content, date, sdate, edate);
        // }
    }

    Ok(())
}

/*
#[warn(unused_doc_comments, unused_assignments, unused_mut)]
fn recursive_del(
    file_path: &Path,
    content: &Vec<String>,
    date: &Option<NaiveDate>,
    sdate: &Option<NaiveDate>,
    edate: &Option<NaiveDate>,
) {
    for entry in WalkDir::new(file_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let mut content_flag = true;
            let mut date_flag = true;
            let mut sedate_flag = true;

            // 检查文件名是否包含content的内容
            for c in content {
                if path.file_name().unwrap().to_str().unwrap().contains(c) {
                    content_flag = true;
                    break;
                }
                content_flag = false;
            }

            /// 使用chrono获取path的最后修改时间，再与变量date做比较
            if let Some(d) = date {
                date_flag = false;

                if let Ok(metadata) = fs::metadata(path) {
                    if let Ok(modified) = metadata.modified() {
                        let modified_date =
                            chrono::DateTime::<chrono::Local>::from(modified).date_naive();
                        date_flag = modified_date == *d;
                    }
                }
            }

            /// 使用chrono获取path的最后修改时间，判断修改时间是否在sdate和edate之间
            if let Some(sd) = sdate {
                if let Some(ed) = edate {
                    if let Ok(metadata) = fs::metadata(path) {
                        if let Ok(modified) = metadata.modified() {
                            sedate_flag = false;
                            let modified_date =
                                chrono::DateTime::<chrono::Local>::from(modified).date_naive();
                            sedate_flag = (modified_date >= *sd) && (modified_date <= *ed);
                        }
                    }
                }
            }

            if content_flag && date_flag && sedate_flag {
                // fs::remove_file(path).unwrap();
                println!("{}", path.display());
            }
        }
    }
}
    */
