use crate::CliConfig;
use crate::file_iterator::FileContainer;
use colored::*;



#[derive(Debug, Clone)]
pub struct Printer {
    pub config: CliConfig,
}
//统计下有多少个文件和文件夹
#[derive(Debug)]
pub struct Summary {
    pub folders_count: usize,
    pub files_count: usize,
}
impl Printer {
    pub fn new(config: CliConfig) -> Printer {
        Printer { config }
    }


    pub fn start(&self) {
        //初始化container
        let mut file_container = FileContainer::new(self.config.clone());
        //FileContainer::new()方法会把根路径的file_item push进去

        let mut summary = Summary {
            folders_count: 0,
            files_count: 0,
        };

        let cur = file_container.next().unwrap();//去掉根目录
        let tip = "working directory:".to_string();
        println!("{} {}", tip.bold().blue(), cur.path.display().to_string().purple()); //打印一个点 表示当前目录



        for item in file_container {

            if item.is_dir {
                summary.folders_count += 1;
            } else {
                summary.files_count += 1;
            }
            let mut iden = String::new();


            for _ in 0..item.level-1{
                iden.push_str("│   ");
            }
            if item.is_last {
                iden.push_str("└── ");
            }else{
                iden.push_str("├── ");
            }

            let mut permission_str = String::from("[");
            if self.config.show_permission {
                if item.is_dir {
                    permission_str.push_str("d");
                }else{
                    permission_str.push_str("-");
                }
                let mut owner_per = 0;
                let mut group_per = 0;
                let mut other_per = 0;

                owner_per = item.permission%512/64;
                group_per = ((item.permission%512)-(owner_per*64))/(8);
                other_per = (item.permission%512)%8;

                match owner_per {
                    7 => permission_str.push_str("rwx"),
                    6 => permission_str.push_str("rw-"),
                    5 => permission_str.push_str("r-x"),
                    4 => permission_str.push_str("r--"),
                    3 => permission_str.push_str("-wx"),
                    2 => permission_str.push_str("-w-"),
                    1 => permission_str.push_str("--x"),
                    0 => permission_str.push_str("---"),
                    _ => (),
                }
                match group_per {
                    7 => permission_str.push_str("rwx"),
                    6 => permission_str.push_str("rw-"),
                    5 => permission_str.push_str("r-x"),
                    4 => permission_str.push_str("r--"),
                    3 => permission_str.push_str("-wx"),
                    2 => permission_str.push_str("-w-"),
                    1 => permission_str.push_str("--x"),
                    0 => permission_str.push_str("---"),
                    _ => (),
                }
                match other_per {
                    7 => permission_str.push_str("rwx"),
                    6 => permission_str.push_str("rw-"),
                    5 => permission_str.push_str("r-x"),
                    4 => permission_str.push_str("r--"),
                    3 => permission_str.push_str("-wx"),
                    2 => permission_str.push_str("-w-"),
                    1 => permission_str.push_str("--x"),
                    0 => permission_str.push_str("---"),
                    _ => (),
                }
                permission_str.push_str("]  ");
            }

            let mut inode_str = String::from("[");
            if self.config.show_inodes {
                inode_str.push_str(item.inode.to_string().as_str());
                inode_str.push_str("]  ");
            }

            let mut device_num_str = String::from("[");
            if self.config.show_device_num {
                device_num_str.push_str(item.device_num.to_string().as_str());
                device_num_str.push_str("]  ");
            }

            if self.config.full_path {
                if self.config.show_permission {
                    iden.push_str(permission_str.as_str());
                }
                if self.config.show_inodes {
                    iden.push_str(inode_str.as_str());
                }
                if self.config.show_device_num {
                    iden.push_str(device_num_str.as_str());
                }
                if item.is_dir{
                    println!("{}{}", iden, item.path.display().to_string().red().bold());
                }else{
                    println!("{}{}", iden, item.path.display().to_string().green());
                }
            }else{
                if self.config.show_permission {
                    iden.push_str(permission_str.as_str());
                }
                if self.config.show_inodes {
                    iden.push_str(inode_str.as_str());
                }
                if self.config.show_device_num {
                    iden.push_str(device_num_str.as_str());
                }

                if item.is_dir{
                    println!("{}{}", iden, item.file_name.red().bold());
                }else{
                    println!("{}{}", iden, item.file_name.green());
                }
            }

        }


        if !self.config.dir_only {
            println!("\n{} directories, {} files", summary.folders_count.to_string().blue().bold(), summary.files_count.to_string().blue().bold());
        }else{
            println!("\n{} directories", summary.folders_count.to_string().blue().bold());
        }
    }
}


