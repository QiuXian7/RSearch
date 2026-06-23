mod cli;
mod duplicate;
mod filters;
mod history;
mod index;
mod models;
mod scanner;
mod search;
mod stats;
mod storage;

use clap::Parser;

use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Name { keyword, path } => {
            history::add_history(&format!("name {} {}", keyword, path));

            let files = scanner::scan_directory(&path)?;

            let result = search::search_by_name(&files, &keyword);

            println!("\n找到 {} 个文件:", result.len());

            for file in result {
                println!("{}", file.path);
            }
        }

        Commands::Content { keyword, path } => {
            history::add_history(&format!("content {} {}", keyword, path));

            let files = scanner::scan_directory(&path)?;

            let result = search::search_content_with_preview(&files, &keyword);

            println!("\n找到 {} 个结果", result.len());

            for (file, preview) in result {
                println!("{}", file.path);

                println!("  {}", preview);

                println!();
            }
        }

        Commands::Ext { ext, path } => {
            history::add_history(&format!("ext {} {}", ext, path));

            let files = scanner::scan_directory(&path)?;

            let result = filters::filter_by_extension(&files, &ext);

            println!("\n找到 {} 个文件:", result.len());

            for file in result {
                println!("{}", file.path);
            }
        }

        Commands::Size { min_size, path } => {
            history::add_history(&format!("size {} {}", min_size, path));

            let files = scanner::scan_directory(&path)?;

            let result = filters::filter_by_size(&files, min_size);

            println!("\n找到 {} 个文件:", result.len());

            for file in result {
                println!("{} ({} bytes)", file.path, file.size);
            }
        }

        Commands::Recent { days, path } => {
            history::add_history(&format!("recent {} {}", days, path));

            let files = scanner::scan_directory(&path)?;

            let result = filters::filter_recent(&files, days);

            println!("\n找到 {} 个文件:\n", result.len());

            for file in result {
                println!("{}", file.path);
            }
        }

        Commands::Regex { pattern, path } => {
            history::add_history(&format!("regex {} {}", pattern, path));

            let files = scanner::scan_directory(&path)?;

            let result = search::regex_search(&files, &pattern);

            println!("\n找到 {} 个文件:", result.len());

            for file in result {
                println!("{}", file.path);
            }
        }

        Commands::Duplicate { path } => {
            history::add_history(&format!("duplicate {}", path));

            let files = scanner::scan_directory(&path)?;

            let result = duplicate::find_duplicates(&files);

            if result.is_empty() {
                println!("未发现重复文件");
            } else {
                println!("\n发现重复文件:");

                for (_, paths) in result {
                    println!("----------------");

                    for path in paths {
                        println!("{}", path);
                    }

                    println!();
                }
            }
        }

        Commands::History => {
            history::show_history();
        }

        Commands::BuildIndex { path } => {
            history::add_history(&format!("build-index {}", path));

            let files = scanner::scan_directory(&path)?;

            let index = index::build_index(&files);

            storage::save_index(&index)?;

            println!("索引构建完成");
        }

        Commands::SearchIndex { keyword } => {
            history::add_history(&format!("search-index {}", keyword));

            let index = storage::load_index()?;

            let result = index::search_index(&index, &keyword);

            println!("\n找到 {} 个结果\n", result.len());

            for item in result {
                println!("{}   score={}", item.path, item.score);
            }
        }

        Commands::Stats { path } => {
            let files = scanner::scan_directory(&path)?;

            let stat = stats::calculate_stats(&files);

            println!("\n目录统计");

            println!("文件数量: {}", stat.total_files);

            println!("总大小: {} bytes", stat.total_size);
        }
    }

    Ok(())
}
