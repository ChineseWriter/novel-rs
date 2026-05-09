//! 书籍对象的数据结构定义


// 导入标准库
use core::str;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::BuildHasher;
use std::ops::{Add, AddAssign};

// 导入第三方库
use chrono::{DateTime, Local};
use rapidhash::quality::SeedableState;
use base64::engine::{general_purpose, Engine as _};


// 该常量用于生成 UUID 的种子, 以保证生成的 UUID 在不同运行环境中具有一致性.
const UUID_SEED: u64 = 4209936242969777109;

// 该常量用于表示未知的更新时间, 以便于在没有章节数据时返回一个合理的字符串.
const NO_TIME_STRING: &str = "未知";

// 该常量用于格式化更新时间字符串, 以便于在需要返回更新时间字符串时使用.
const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";


/// 书籍来源集合.
///
/// 使用 `HashSet` 可保证来源去重.
pub type Sources = HashSet<String>;

/// 通用标识符.
///
/// 目前将使用 SHA3_256 哈希算法生成.
pub type Uuid = Box<[u8]>;


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BookState {
    Unknown = 0,    // 未知
    Hiatus = 1,     // 断更
    Ongoing = 2,    // 连载中
    Completed = 3,  // 已完结
}


/// 章节内容类型.
///
/// 目前仅支持文本和图片两种类型, 以便于后续的处理和存储.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    Text,   // 文本
    Image,  // 图片
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
    pub content_type: ContentType,
    pub data: Vec<u8>,
    pub length: usize
}


trait GenerateId {
    fn uuid_str(&self) -> String;

    fn uuid(&self) -> Uuid {
        let result = SeedableState::new(UUID_SEED)
            .hash_one(self.uuid_str().as_bytes())
            .to_le_bytes();
        Box::new(result)
    }
}


impl Book {
    pub fn new(
        meta: BookMeta, cover: Option<Content>, chapters: Vec<Chapter>
    ) -> Self {
        Self { meta, cover, chapters }
    }

    pub fn update_time_str(&self) -> String {
        if self.chapters.is_empty() {
            NO_TIME_STRING.to_string()
        } else {
            let latest_update_time = self.chapters.iter()
                .map(|chapter| chapter.meta.update_time)
                .max();
            if let Some(latest_update_time) = latest_update_time {
                latest_update_time.format(TIME_FORMAT).to_string()
            } else {
                NO_TIME_STRING.to_string()
            }
        }
    }

    fn sort_chapters(&mut self) {
        self.chapters.sort_by_key(|chapter| chapter.meta.index);
    }
}

impl Chapter {
    pub fn new(meta: ChapterMeta, content: Content) -> Self {
        Self { meta, content }
    }

    pub fn update_time_str(&self) -> String {
        self.meta.update_time.format(TIME_FORMAT).to_string()
    }
}


impl BookMeta {
    pub fn new(
        title: String, author: String, state: BookState,
        sources: HashSet<String>, other_info: HashMap<String, String>
    ) -> Self {
        let mut obj = Self {
            id: Box::new([0u8; 8]), title, author,
            state, sources, other_info
        };
        obj.id = obj.uuid();
        obj
    }
}


impl ChapterMeta {
    pub fn new(
        index: u16, title: String, update_time: DateTime<Local>,
        sources: HashSet<String>, other_info: HashMap<String, String>
    ) -> Self {
        let mut obj = Self {
            id: Box::new([0u8; 8]), index, title,
            update_time, sources, other_info
        };
        obj.id = obj.uuid();
        obj
    }
}


impl Content {
    pub fn new(content_type: ContentType, data: Vec<u8>) -> Self {
        let length = data.len();
        let mut obj = Self {
            id: Box::new([0u8; 8]), content_type, data, length
        };
        obj.id = obj.uuid();
        obj
    }
}


impl Default for Book {
    fn default() -> Self {
        Self {
            meta: BookMeta::new(
                "默认书籍名".to_string(), "默认作者名".to_string(),
                BookState::Unknown, HashSet::new(), HashMap::new()
            ),
            cover: None,
            chapters: vec![Chapter::default()]
        }
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
        // 这一步实际上也验证了书籍名和作者名一致.
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
        // 书籍状态取两者中较高的那个, 以保证状态的准确性.
        let state = left_meta.state.max(right_meta.state);
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
        // 合并章节
        let mut chapters: HashMap<_, _> = left_chapters.into_iter()
            .map(|chapter| (chapter.meta.id.clone(), chapter)).collect();
        for chapter in right_chapters {
            if let Some(existing) = chapters.get_mut(&chapter.meta.id) {
                *existing += chapter;
            } else {
                chapters.insert(chapter.meta.id.clone(), chapter);
            }
        };
        let chapters: Vec<_> = chapters.into_values().collect();
        // 构造合并后的书籍对象
        let mut result = Self::Output {
            meta: BookMeta {
                id: left_meta.id,          // 保持原有 ID 不变
                title: left_meta.title,    // 保持原有标题不变
                author: left_meta.author,  // 保持原有作者不变
                state,                     // 使用合并后的状态
                sources,                   // 使用合并后的来源
                other_info,                // 使用合并后的其他信息
            },
            cover,                         // 使用合并后的封面
            chapters,                      // 使用合并后的章节
        };
        result.sort_chapters();  // 确保章节按索引排序
        result
    }
}


impl Extend<Chapter> for Book {
    // 请注意, 该 Trait 仅需实现 extend 方法,
    // 其他方法包括 extend_one 和 extend_reserve 均可使用默认实现.

    fn extend<T: IntoIterator<Item = Chapter>>(&mut self, iter: T) {
        let mut chapters: HashMap<_, _> = self.chapters.drain(..)
            .map(|item| (item.meta.id.clone(), item)).collect();

        for chapter in iter {
            if let Some(existing) = chapters.get_mut(&chapter.meta.id) {
                *existing += chapter;
            } else {
                chapters.insert(chapter.meta.id.clone(), chapter);
            }
        };

        self.chapters = chapters.into_values().collect();
        self.sort_chapters();
    }
}


impl Default for Chapter {
    fn default() -> Self {
        Self {
            meta: ChapterMeta::new(
                0, String::new(),
                Local::now(), HashSet::new(),
                HashMap::new()
            ),
            content: Content::new(
                ContentType::Text, "\t默认章节内容".to_string().into_bytes()
            )
        }
    }
}


impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.meta.id == other.meta.id
    }
}


impl AddAssign for Chapter {
    fn add_assign(&mut self, rhs: Self) {
        // 章节合并的前提是两者具有相同的 `id`, 否则不进行合并直接返回左值.
        if self.meta.id != rhs.meta.id { return }
        // 从左值和右值中解构出字段, 以便于后续合并逻辑的实现.
        let Chapter {
            meta: right_meta,
            content: right_content,
        } = rhs;
        // 由于使用了 HashSet 和 HashMap, 直接使用 extend 方法进行合并
        self.meta.sources.extend(right_meta.sources);
        self.meta.other_info.extend(right_meta.other_info);
        // 更新时间取两者中较晚的那个
        self.meta.update_time = self.meta.update_time.max(right_meta.update_time);
        // 内容取两者中长度较大的那个, 以保证内容质量.
        if self.content.length < right_content.length {
            self.content = right_content;
        }
    }
}


impl GenerateId for BookMeta {
    fn uuid_str(&self) -> String {
        format!("《{}》 - {}", self.title, self.author)
    }
}


impl GenerateId for ChapterMeta {
    fn uuid_str(&self) -> String {
        format!("第{}章 {}", self.index, self.title)
    }
}


impl GenerateId for Content {
    fn uuid_str(&self) -> String {
        general_purpose::STANDARD.encode(&self.data)
    }
}


impl fmt::Display for BookState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state_str = match self {
            BookState::Unknown   => "未知",
            BookState::Hiatus    => "断更",
            BookState::Ongoing   => "连载中",
            BookState::Completed => "已完结",
        };
        write!(f, "{}", state_str)
    }
}


impl str::FromStr for BookState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "未知"   => Ok(BookState::Unknown),
            "断更"   => Ok(BookState::Hiatus),
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
