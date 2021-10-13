use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use serde_json;

use super::*;

#[derive(Clone)]
struct TFIDF {
    pd: PersistentData,
    wm: Box<WordMap>,
    dm: Box<DocMap>,
}

impl TFIDF {
    pub fn new() -> TFIDF {
        TFIDF {
            pd: PersistentData::default(),
            wm: Box::new(WordMap::new()),
            dm: Box::new(DocMap::new()),
        }
    }

    pub fn load_from(&mut self, pd_file_name: String, fd_file_name: String) -> Result<()> {
        let pd_file_handler = File::open(pd_file_name)?;
        let fd_file_handler = File::open(fd_file_name)?;

        let mut pd: PersistentData = serde_json::from_reader(pd_file_handler)?;
        let mut fd: PersistentData = serde_json::from_reader(fd_file_handler)?;
        self.pd.doc_count = fd.doc_count;
        self.pd.word_count = fd.word_count;

        self.pd.docs = pd.docs.clone();
        self.pd.words = pd.words.clone();

        // GO GC考虑所以写下面  而RUST 可以不考虑
        // for (index, value) in pd.docs.iter().enumerate() {
        //     let doc = Doc{
        //         id: value.id.clone(),
        //         words: value.words.clone(),
        //     };
        //     self.pd.docs[index] = doc;
        // }

        self.init_derived_data();
        Ok(())
    }

    fn init_derived_data(&mut self) {
        for (k, v) in self.pd.words.iter().enumerate() {
            self.wm.set_word(Word {
                doc_set: Box::from(DocSet::new()),
                index: k,
                value: v.clone(),
            })
        }

        for (_, v) in self.pd.docs.iter().enumerate() {
            self.dm.set_doc(v.clone());
            for (_, v2) in v.words.iter().enumerate() {
                match self.wm.get_word(v2.clone()) {
                    Some(mut r) => {
                        r.add_doc(v.id.clone());
                        self.wm.set_word(r);
                    }
                    None => continue,
                }
            }
        }
    }

    pub fn save(&self, pd_file_name: String, fd_file_name: String) -> Result<()> {
        let pd = PersistentData {
            updated: false,
            doc_count: self.pd.docs.len(),
            word_count: self.pd.words.len(),
            docs: self.pd.docs.clone(),
            words: self.pd.words.clone(),
        };

        let mut fd = PersistentData::default();
        fd.doc_count = self.pd.docs.len();
        fd.word_count = self.pd.words.len();

        let pd = serde_json::to_string(&pd)?;
        let fd = serde_json::to_string(&fd)?;

        let mut pd_file = File::create(pd_file_name)?;
        pd_file.write_all(pd.as_bytes())?;

        let mut fd_file = File::create(fd_file_name)?;
        fd_file.write_all(fd.as_bytes())?;

        Ok(())
    }

    pub fn doc_count(&self) -> usize {
        self.pd.docs.len()
    }

    pub fn word_count(&self) -> usize {
        self.pd.words.len()
    }

    pub fn tf(&self, doc: Doc, word: String) -> f64 {
        let mut count: f64 = 0.0;

        for v in doc.words.iter() {
            if word.eq(v) {
                count += 1.0;
            }
        }

        count / doc.words.len() as f64
    }

    pub fn tf_vector(&self, doc: Doc) -> Vec<f64> {
        let mut count_map: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();
        for i in doc.words.iter() {
            match count_map.get(i) {
                Some(r) => {
                    count_map.insert(i.clone(), r + 1);
                }
                None => {
                    count_map.insert(i.clone(), 1);
                }
            }
        }

        for i in doc.words.iter() {
            result.push(count_map.get(i).unwrap().clone() as f64 / doc.words.len() as f64)
        }
        result
    }

    pub fn idf(&self, w: String) -> Option<f64> {
        if let None = self.wm.get_word(w.clone()) {
            return None;
        }

        return Some(
            self.doc_count().log2() as f64 / self.wm.get_word(w).unwrap().doc_count() as f64 + 1.0,
        );
    }

    pub fn idf_vector(&self, doc: Doc) -> Option<Vec<f64>> {
        let mut result = Vec::with_capacity(doc.words.len());
        for i in doc.words.iter() {
            match self.idf(i.clone()) {
                Some(r) => {
                    result.push(r);
                }
                None => {
                    return None;
                }
            }
        }

        Some(result)
    }

    fn dot_product_in(&self, x: Vec<f64>, y: Vec<f64>) -> Vec<f64> {
        let mut result = Vec::with_capacity(x.len());
        for (k, v) in y.iter().enumerate() {
            result.insert(k, x.get(k).unwrap() * v)
        }
        result
    }

    pub fn dot_product(&self, a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
        if a.len() > b.len() {
            return self.dot_product_in(a, b);
        }
        return self.dot_product_in(b, a);
    }

    pub fn upsert_docs(&mut self, docs: Vec<Doc>) {
        for i in docs.iter() {}
    }

    pub fn upsert_doc(&mut self, doc: Doc) {
        let pre_doc = self.dm.get_doc(doc.id.clone());
        if let None = pre_doc {
            let i = self.pd.append_doc(doc.clone());
            self.dm.set_doc(self.pd.docs.get(i).unwrap().clone());
            self.re_index_words(doc.clone());
            return;
        }

        let (incr, decr) = pre_doc.wor
    }

    pub fn re_index_words(&mut self, doc: Doc) {
        for i in doc.words.iter() {
            let w = self.wm.get_word(i.clone());
            if let None = w {
                let mut p = DocSet::new();
                p.append(doc.id.clone());
                self.wm.set_word(Word {
                    value: i.clone(),
                    index: self.pd.append_word(i.clone()),
                    doc_set: Box::new(p),
                });
                continue;
            }
            w.unwrap().doc_set.append(doc.id.clone());
        }
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

impl Doc {
    pub fn words_diff(new_words: Vec<String>) -> (Vec<String>, Vec<String>) {
        (new_words.clone(), new_words)
    }
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
    index: usize,
    doc_set: Box<DocSet>,
}

impl Word {
    pub fn get_index(&self) -> usize {
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
    pub fn new() -> DocSet {
        DocSet { m: HashMap::new() }
    }

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
