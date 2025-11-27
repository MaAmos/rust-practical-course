// fm - 文件管理器
// A simple file manager written in Rust
mod args;
use args::Args;
// 这一行必须加 否则无法直接调用parse方法
use clap::Parser;
mod utils;
// 直接引入函数（因为在 mod.rs 里 pub use 了）
use utils::{get_file_list, print_file_list_table, print_file_list_row};
fn main() {
    let args_opts = Args::parse();
    let file_list = get_file_list(&args_opts, None).unwrap();

    // 打印参数 和格式化输出结果
    let stats_fields = &args_opts.stats;

    // 设置输出的格式
    let output_format = args_opts.output.unwrap();
    if let args::OutputFormat::Csv = output_format {
        print_file_list_table(&file_list, stats_fields);
    } else {
        // 拼接字符串 按行输出
        print_file_list_row(&file_list, stats_fields);
    }

    // println!("{:?}\n当前目录为{:#?}", args_opts, file_list);
}
