use std::collections::HashMap;
use std::mem;

#[derive(Clone)]
struct TFIDF {
    pb: PersistentData,
    wm: Box<WordMap>,
    dm: Box<DocMap>,
}

#[derive(Clone)]
struct PersistentData {
    updated: bool,

    doc_count: i64,
    word_count: i64,

    docs: Vec<Doc>,
    words: Vec<String>,
}

#[derive(Clone)]
struct Doc {
    id: String,
    words: Vec<String>,
}

#[derive(Clone)]
struct WordMap {
    m: HashMap<String, Box<Option<Word>>>,
}

impl WordMap {
    pub fn set_word(&mut self, w: Word) {
        self.m.insert(w.value.clone(), Box::new(Some(w)));
    }

    pub fn get_word(&self, s: String) -> Option<Word> {
        match self.m.get(&s) {
            Some(r) => {
                let b = r.as_ref().clone();
                b
            }
            None => None,
        }
    }
}

#[derive(Clone)]
struct Word {
    value: String,
    index: i64,
    doc_set: Box<DocSet>,
}

impl Word {
    pub fn get_index(&self) -> int {
        self.index
    }

    pub fn add_doc(&mut self, doc_id: String) {
        // self.doc_set.as_ref()
    }
}

#[derive(Clone)]
struct DocSet {
    m: HashMap<String, bool>,
}

impl DocSet {
    pub fn append(&mut self, str: String) {
        self.m.insert(str, true);
    }

    pub fn del(&mut self, str: String) {
        self.m.remove(&str);
    }

    pub fn exits(&self, str: String) -> bool {
        match self.m.get(&str) {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(Clone)]
struct DocMap {
    m: HashMap<String, Box<Option<Doc>>>,
}

impl DocMap {
    pub fn set_doc(&mut self, d: Doc) {
        self.m.insert(d.id.clone(), Box::new(Some(d)));
    }

    pub fn get_doc(&self, id: String) -> Option<Doc> {
        match self.m.get(&id) {
            Some(r) => {
                let p = r.as_ref().clone();
                p
            }
            None => None
        }
    }
}
