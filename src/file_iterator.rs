use std::path::PathBuf;
use std::fs::{Metadata, DirEntry};
use std::collections::VecDeque;
use crate::CliConfig;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::MetadataExt;

//抽象文件对象,包括一些基本的属性,暂时就需要这么多
pub struct FileItem {
    pub file_name: String,  //文件名
    pub metadata: Metadata, //文件元信息 通过fs::metadata()得到
    pub path: PathBuf,      //文件地址
    pub is_dir: bool,       //是否是文件夹
    pub level: usize,       //当前文件的深度
    pub is_empty: bool,     //文件夹是否是空的
    pub is_last: bool,      //文件夹最后一个文件
    pub permission: u32,
    pub inode: u64, //index node
    pub device_num: u64, //device number
}

impl FileItem {
    //构造函数
    pub fn new(p: PathBuf) -> FileItem {
        let metadata = p.symlink_metadata().unwrap();
        FileItem {
            file_name: p.file_name().unwrap().to_str().unwrap().to_string(),
            path: p.clone(),
            is_dir: metadata.is_dir(),
            level: 0,
            is_empty: false,
            is_last: false,
            permission: metadata.permissions().mode(),
            inode: metadata.ino(),
            device_num: metadata.dev(),
            metadata,
        }
    }
    //无法在初始化FileItem对象时获得的参数我们设置他的set方法
    pub fn set_level(&mut self, level: usize) {
        self.level = level;
    }
    pub fn set_last(&mut self, last: bool) {
        self.is_last = last;
    }
    pub fn set_empty(&mut self, empty: bool) {
        self.is_empty = empty;
    }
}




//printer遍历此对象
pub struct FileContainer {
    pub queue: VecDeque<FileItem>, //双端队列,队列里存着一个个FileItem
    pub config: CliConfig,  //cliConfig
}

//将栈的push和pop抽象为一个迭代器
impl Iterator for FileContainer {
    type Item = FileItem;  //关联类型
    //第一个是target_dir
    fn next(&mut self) -> Option<Self::Item> {
        //从队列头里拿出来一个fileitem
        if let Some(mut item) = self.queue.pop_front() {
            //如果是文件夹且深度小于深度要求就入栈
            if item.is_dir && item.level < self.config.max_level {
                self.push_dir(&mut item);
            }
            //这里给printer打印
            Some(item)
        } else {
            //队列空了
            None
        }
    }
}



impl FileContainer {
    pub fn new(config: CliConfig) -> FileContainer {
        let mut deque = VecDeque::new();
        //push根文件夹也就是target_dir
        deque.push_back(FileItem::new(PathBuf::from(&config.target_dir)));
        FileContainer {
            queue: deque,
            config,
        }
    }

    pub fn push_dir(&mut self, item: &mut FileItem, ) {
        //item是一个文件夹的FileItem
        //遍历出子文件夹先变成DirEntry
        let dirs = item
            .path //拿到文件夹的路径
            .read_dir()
            .map(|read_dir| {
                let dirs: Vec<DirEntry> = read_dir.filter_map(Result::ok).collect();
                //当前文件夹是否为空
                if dirs.len() > 0 {
                    item.set_empty(false);
                }
                dirs
            })
            .expect("push dir错误");

        //把找到的DirEntry初始化为fileitem对象并生成一个FileItem数组
        let mut file_items: Vec<_> = dirs
            .iter()
            .map(|dir| {
                let mut file_item = FileItem::new(dir.path());
                file_item.set_level(item.level + 1);
                file_item
            })
            .collect();

        //是否对文件进行排序
        if !self.config.do_not_sort {
            file_items.sort_by_key(|item| item.file_name.clone());
            file_items.reverse();
        }

        //是否显示隐藏文件
        if !self.config.show_hidden_files {
            file_items.retain(|item| item.clone().file_name.starts_with(".")==false);
        }

        //是否仅显示目录
        if self.config.dir_only {
            file_items.retain(|item| item.clone().is_dir);
        }


        //拿到第一个fileitem设置其last属性为true
        //vec里的第一个 进入栈后就变成了最后一个
        if let Some(item) = file_items.first_mut() {
            item.set_last(true);
        }

        //所有file加入到栈里
        for file_item in file_items {
            self.queue.push_front(file_item);
        }
    }
}