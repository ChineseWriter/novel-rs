//! 书籍对象的数据结构定义


use core::str;
// 导入标准库
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Add;
use std::path::PathBuf;

// 导入第三方库
use chrono::{DateTime, Local};


/// 书籍来源集合.
///
/// 使用 `HashSet` 可保证来源去重.
pub type Sources = HashSet<String>;

/// 通用标识符.
///
/// 目前将使用 SHA3_256 哈希算法生成.
pub type Uuid = Vec<u8>;


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookState {
    Unknown = 0,    // 未知
    Hiatus = 1,     // 断更
    Ongoing = 2,    // 连载中
    Completed = 3,  // 已完结
}


/// 书籍对象.
///
/// 书籍对象是整个项目的核心数据结构, 所有的模块间数据交互均以该数据结构为基础.
/// 出于该目的考虑, 书籍对象拥有所有数据的所有权, 以便于可能的数据传递.
///
/// 书籍对象包含以下几个部分:
///
/// 1. 书籍元数据
/// 2. 书籍封面(实际存储为 Content 对象)
/// 3. 章节数据
#[derive(Debug)]
pub struct Book {
    pub meta: BookMeta,
    pub cover: Option<Content>,
    pub chapters: Vec<Chapter>
}

/// 章节对象.
///
/// 章节对象是书籍对象的一部分.
///
/// 考虑到章节数据只有文本和图片两种类型, 所以将它们存储为 HTML 格式.
/// 其中图片数据以 Base64 编码的形式存储在 HTML 标签中.
/// 所有的章节数据都存储在外部存储系统中, 书籍对象中只存储其路径.
///
/// 章节对象包含以下几个部分:
///
/// 1. 章节元数据
/// 2. 章节内容
#[derive(Debug)]
pub struct Chapter {
    pub meta: ChapterMeta,
    pub content: Content
}

/// 书籍元数据.
///
/// 该结构只承载描述信息, 不包含正文内容与封面内容.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookMeta {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub state: BookState,
    pub sources: Sources,
    pub other_info: HashMap<String, String>
}

/// 章节元数据.
///
/// 该结构只承载描述信息, 不包含正文内容.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChapterMeta {
    pub id: Uuid,
    pub index: u16,
    pub title: String,
    pub update_time: DateTime<Local>,
    pub sources: Sources,
    pub other_info: HashMap<String, String>
}

/// 具体内容在外部存储中的引用.
///
/// 其中 `length` 表示内容长度, `path` 是内容落盘路径.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Content {
    pub id: Uuid,
    pub length: usize,
    pub path: PathBuf,
}


trait GenerateId {
    fn uuid_str(&self) -> String;

    fn uuid(&self) -> Uuid {
        unimplemented!()
    }
}


impl Book {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn update_time_str(&self) -> String {
        unimplemented!()
    }

    fn sort_chapters(&mut self) {
        self.chapters.sort_by_key(|chapter| chapter.meta.index);
    }
}

impl Chapter {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn update_time_str(&self) -> String {
        unimplemented!()
    }
}


impl Default for Book {
    fn default() -> Self {
        unimplemented!()
    }
}


impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.meta.id == other.meta.id
    }
}


impl Add for Book {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // 书籍合并的前提是两者具有相同的 `id`, 否则不进行合并直接返回左值.
        if self.meta.id != rhs.meta.id { return self }
        // 从左值和右值中解构出字段, 以便于后续合并逻辑的实现.
        let Book {
            meta: left_meta,
            cover: left_cover,
            chapters: left_chapters,
        } = self;
        let Book {
            meta: right_meta,
            cover: right_cover,
            chapters: right_chapters,
        } = rhs;
        // 由于使用了 HashSet 和 HashMap, 直接使用 extend 方法进行合并
        let mut sources = left_meta.sources;
        sources.extend(right_meta.sources);
        let mut other_info = left_meta.other_info;
        other_info.extend(right_meta.other_info);
        // 封面取两者中长度较大的那个, 以保证封面质量.
        let cover = match (left_cover, right_cover) {
            (Some(l), Some(r)) =>
                if l.length >= r.length { Some(l) } else { Some(r) },
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
        };
        todo!();
        Self {
            meta: BookMeta {
                sources,
                other_info,
                ..left_meta
            },
            cover,
            chapters:
        }
    }
}


impl Extend<Chapter> for Book {
    // 请注意, 该 Trait 仅需实现 extend 方法,
    // 其他方法包括 extend_one 和 extend_reserve 均可使用默认实现.

    fn extend<T: IntoIterator<Item = Chapter>>(&mut self, iter: T) {
        unimplemented!()
    }
}

impl GenerateId for BookMeta {
    fn uuid_str(&self) -> String {
        unimplemented!()
    }
}


impl Default for Chapter {
    fn default() -> Self {
        unimplemented!()
    }
}


impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.meta.id == other.meta.id
    }
}


impl Add for Chapter {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // 章节合并的前提是两者具有相同的 `id`, 否则不进行合并直接返回左值.
        if self.meta.id != rhs.meta.id { return self }
        // 从左值和右值中解构出字段, 以便于后续合并逻辑的实现.
        let Chapter {
            meta: left_meta,
            content: left_content,
        } = self;
        let Chapter {
            meta: right_meta,
            content: right_content,
        } = rhs;
        // 由于使用了 HashSet 和 HashMap, 直接使用 extend 方法进行合并
        let mut sources = left_meta.sources;
        sources.extend(right_meta.sources);
        let mut other_info = left_meta.other_info;
        other_info.extend(right_meta.other_info);
        // 更新时间取两者中较晚的那个
        let update_time = left_meta.update_time.max(right_meta.update_time);
        let content =
            if left_content.length >= right_content.length
                { left_content } else { right_content };
        // 最后根据合并规则构造新的章节对象并返回.
        Self {
            meta: ChapterMeta {
                update_time,
                sources,
                other_info,
                ..left_meta
            },
            content
        }
    }
}


impl GenerateId for ChapterMeta {
    fn uuid_str(&self) -> String {
        unimplemented!()
    }
}


impl fmt::Display for BookState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state_str = match self {
            BookState::Unknown => "未知",
            BookState::Hiatus => "断更",
            BookState::Ongoing => "连载中",
            BookState::Completed => "已完结",
        };
        write!(f, "{}", state_str)
    }
}


impl str::FromStr for BookState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "未知" => Ok(BookState::Unknown),
            "断更" => Ok(BookState::Hiatus),
            "连载中" => Ok(BookState::Ongoing),
            "已完结" => Ok(BookState::Completed),
            _ => Err(()), // 默认未知状态
        }
    }
}


impl From<u8> for BookState {
    fn from(value: u8) -> Self {
        match value {
            0 => BookState::Unknown,
            1 => BookState::Hiatus,
            2 => BookState::Ongoing,
            3 => BookState::Completed,
            _ => BookState::Unknown, // 默认未知状态
        }
    }
}
