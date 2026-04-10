//! 书籍对象的数据结构定义


// 导入标准库
use std::{
    path,
    collections::HashMap,
    time::SystemTime
};


struct Book {
    id: [u8; 64],
    type_: BookType,
    title: String,
    author: String,
    cover: Blob,
    sources: Vec<String>,
    chapters: Vec<Chapter>,
    others: HashMap<String, String>
}


struct Chapter {
    id: [u8; 64],
    index: u16,
    title: String,
    update_time: SystemTime,
    content: Blob,
    pictures: Vec<Blob>,
    sources: Vec<String>,
    others: HashMap<String, String>
}


struct Blob {
    id: [u8; 64],
    type_: BlobType,
    path: path::PathBuf,
}


enum BookType {
    Novel,
    Comic
}

enum BlobType {
    Picture,
    Text
}
