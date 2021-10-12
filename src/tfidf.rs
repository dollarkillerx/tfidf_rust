use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::mem;
use serde_json;
use super::*;

#[derive(Clone)]
struct TFIDF {
    pb: PersistentData,
    wm: Box<WordMap>,
    dm: Box<DocMap>,
}

impl TFIDF {
    pub fn new() -> TFIDF {
        TFIDF {
            pb: PersistentData::default(),
            wm: Box::new(WordMap::new()),
            dm: Box::new(DocMap::new()),
        }
    }

    pub fn load_from(&mut self, pd_file_name: String, fd_file_name: String) -> Result<()> {
        let pd_file_handler = File::open(pd_file_name)?;
        let fd_file_handler = File::open(fd_file_name)?;

        let mut pd:PersistentData = serde_json::from_reader(pd_file_handler)?;
        let mut fd:PersistentData = serde_json::from_reader(fd_file_handler)?;
        self.pb.doc_count = fd.doc_count;
        self.pb.word_count = fd.word_count;

        self.pb.docs = Vec::with_capacity(pd.docs.len());
        self.pb.words = Vec::with_capacity(pd.words.len());

        for (index, value) in pb.docs.enumerate() {

        }

        Ok(())
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct PersistentData {
    updated: bool,

    doc_count: usize,
    word_count: usize,

    docs: Vec<Doc>,
    words: Vec<String>,
}

impl PersistentData {
    pub fn append_word(&mut self, s: String) -> usize {
        self.words.push(s);
        self.updated = true;
        self.word_count = self.words.len();
        self.word_count - 1
    }

    pub fn append_doc(&mut self, doc: Doc) -> usize {
        self.docs.push(doc);
        self.updated = true;
        self.doc_count = self.docs.len();
        self.doc_count - 1
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Doc {
    id: String,
    words: Vec<String>,
}

#[derive(Clone)]
struct WordMap {
    m: HashMap<String, Box<Option<Word>>>,
}

impl WordMap {
    pub fn new() -> WordMap {
        WordMap { m: HashMap::new() }
    }

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
    pub fn get_index(&self) -> i64 {
        self.index
    }

    pub fn add_doc(&mut self, doc_id: String) {
        self.doc_set.append(doc_id)
    }

    pub fn del_doc(&mut self, doc_id: String) {
        self.doc_set.del(doc_id)
    }

    pub fn doc_count(&self) -> usize {
        self.doc_set.count()
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

    pub fn count(&self) -> usize {
        self.m.len()
    }
}

#[derive(Clone)]
struct DocMap {
    m: HashMap<String, Box<Option<Doc>>>,
}

impl DocMap {
    pub fn new() -> DocMap {
        DocMap { m: HashMap::new() }
    }

    pub fn set_doc(&mut self, d: Doc) {
        self.m.insert(d.id.clone(), Box::new(Some(d)));
    }

    pub fn get_doc(&self, id: String) -> Option<Doc> {
        match self.m.get(&id) {
            Some(r) => {
                let p = r.as_ref().clone();
                p
            }
            None => None,
        }
    }
}
