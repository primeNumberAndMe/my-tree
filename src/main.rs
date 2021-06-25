mod printer;
mod file_iterator;

use clap::{App, Arg};



//此结构体用来保存dir以及level参数
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub target_dir: String,
    pub max_level: usize,
    pub do_not_sort: bool, //为true表示不需要排序 默认按名字排序
    pub show_hidden_files: bool, //显示隐藏文件 默认隐藏
    pub dir_only: bool,  //仅显示目录 有点难
    pub full_path: bool, //是否显示全路径
    pub show_permission: bool,
    pub show_inodes: bool,
    pub show_device_num: bool,
}

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let author = env!("CARGO_PKG_AUTHORS");
    let version = env!("CARGO_PKG_VERSION");
    App::new(name).version(version).author(author);

    let matches = App::new(name)
        .version(version)
        .author(author)
        .args(&[
            Arg::with_name("dir").index(1).help("Specify the dirctory you wanna itereate, or my-tree will use the current working dircectory"),
            Arg::with_name("level") // tree  ~/Desktop --level 2  两层目录  -l 2
                .short("L")
                .help("Max display depth of the directory tree.")
                .takes_value(true),
            Arg::with_name("not_sort")
                .short("U")
                .help("Do not sort. Lists files in directory order."),
            Arg::with_name("show_hidden_files")
                .short("a")
                .help("All files are printed. By default tree does not print hidden files (those beginning with a dot '.')."),
            Arg::with_name("dir_only")
                .short("d")
                .help("List directories only."),
            Arg::with_name("full_path")
                .short("f")
                .help("Prints the full path for each file."),
            Arg::with_name("show_permission")
                .short("p")
                .help("Print the file type and permissions for each file (as per ls -l)"),
            Arg::with_name("show_inodes")
                .long("inodes")
                .help("Prints the inode number of the file or directory"),
            Arg::with_name("show_device_num")
                .long("device")
                .help("Prints the device number to which the file or directory belongs"),
        ])
        .get_matches();




    //以下代码对命令行参数进行解析：

    //level参数
    //如果给出则使用用户给出的参数
    //如果没有给出则使用usize::MAX
    let max_level: usize = if let Some(max_level) = matches.value_of("level") {
        max_level.parse().expect("转int错误")
    } else {
        usize::MAX
    };

    //dir参数
    let target_dir: String = if let Some(dir) = matches.value_of("dir") {
        dir.to_string()
    } else {
        std::env::current_dir()
            .expect("当前目录解析错误")
            .to_str()
            .unwrap()
            .to_string()
    };

    //not sort参数
    let mut not_sort = false;
    if matches.is_present("not_sort") {
        not_sort = true;
    }

    //show hidden files
    let mut show_hidden_files = false;
    if matches.is_present("show_hidden_files") {
        show_hidden_files = true;
    }

    //dir only
    let mut dir_only = false;
    if matches.is_present("dir_only") {
        dir_only = true;
    }

    let mut full_path = false;
    if matches.is_present("full_path"){
        full_path=true;
    }

    let mut show_permission = false;
    if matches.is_present("show_permission") {
        show_permission=true;
    }

    let mut show_inodes = false;
    if matches.is_present("show_inodes") {
        show_inodes = true;
    }

    let mut show_device_num = false;
    if matches.is_present("show_device_num"){
        show_device_num=true;
    }

    //把tree命令入参存到struct里
    let cli_config = CliConfig {
        target_dir: target_dir,
        max_level: max_level,
        do_not_sort: not_sort,
        show_hidden_files: show_hidden_files,
        dir_only: dir_only,
        full_path: full_path,
        show_permission: show_permission,
        show_inodes: show_inodes,
        show_device_num: show_device_num,
    };


    //开始运行
    printer::Printer::new(cli_config).start();
}