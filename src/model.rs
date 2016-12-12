use std::time::{SystemTime, UNIX_EPOCH};
use errors::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Note {
    modifydate: Option<f64>,
    tags: Vec<String>,
    deleted: u64,
    createdate: Option<f64>,
    systemtags: Vec<String>,
    version: Option<u64>,
    syncnum: Option<u64>,
    key: Option<String>,
    minversion: Option<u64>,
    content: Option<String>,
}

impl Note {
    pub fn new() -> Note {
        Note {
            modifydate: None,
            tags: vec![],
            deleted: 0,
            createdate: None,
            systemtags: vec![],
            version: None,
            syncnum: None,
            key: None,
            minversion: None,
            content: None,
        }
    }

    pub fn content(mut self, s: &str) -> Note {
        self.content = Some(s.to_string());
        self
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    pub fn key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    pub fn set_modified(&mut self) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).chain_err(|| "Could not get duration since unix epoch")?;
        let now = now.as_secs();
        let now = now as f64;
        self.modifydate = Some(now);
        Ok(())
    }

    pub fn delete(&mut self) -> Result<()> {
        self.deleted = 1;
        Ok(())
    }

    pub fn add_tag(mut self, tag: &str) -> Note {
        let t = tag.to_string();
        if ! self.tags.contains(&t) {
            self.tags.push(t);
        }
        self
    }

    pub fn tags(&self) -> &[String] {
        self.tags.as_ref()
    }
}

impl <'a> From<&'a str> for Note {
    fn from(s: &'a str) -> Note {
        From::from(s.to_string())
    }
}

impl From<String> for Note {
    fn from(s: String) -> Note {
        Note {
            modifydate: None,
            tags: vec![],
            deleted: 0,
            createdate: None,
            systemtags: vec![],
            version: None,
            syncnum: None,
            key: None,
            minversion: None,
            content: Some(s),
        }
    }
}

