use std::fmt::Display;
use std::io::Read;
use std::cell::RefCell;

use model::Note;
use errors::*;

use serde_json as json;
use reqwest::{Client, Method};
use reqwest::header::ContentType;
use base64;

const AUTH_URL: &'static str = "https://app.simplenote.com/api/login";
const DATA_URL: &'static str = "https://app.simplenote.com/api2/data";
const INDEX_URL: &'static str = "https://app.simplenote.com/api2/index?";
const NOTE_LIST_LENGTH: u8 = 100;

/// API Client. All communications with the API go through this struct
///
/// # Examples
///
/// ```ignore
/// let client = Simplenote::new("username", "password").uwrap();
/// ```
pub struct Simplenote {
    username: String,
    password: String,
    token: RefCell<Option<String>>,
    client: Client,
}

impl Simplenote {
    pub fn new<T: Display, U: Display>(username: T, password: U) -> Result<Simplenote> {
        Ok(Simplenote {
            username: username.to_string(),
            password: password.to_string(),
            token: RefCell::new(None),
            client: Client::new().chain_err(|| "Could not get reqwest client")?,
        })
    }

    pub fn get_note(&self, note_id: &str) -> Result<Note> {
        self.get_note_with_version(note_id, None)
    }

    pub fn get_note_with_version<T: Into<Option<u32>>>(&self, note_id: &str, version: T)
                                                       -> Result<Note> {
        self._get_note_with_version(note_id, version.into())
    }

    fn _get_note_with_version(&self, note_id: &str, version: Option<u32>) -> Result<Note> {
        let url = if let Some(v) = version {
            format!("{}/{}", note_id, v)
        } else {
            format!("{}", note_id)
        };

        let params = format!("auth={}&email={}", self.token()?, self.username);

        let mut res = self.client.get(&format!("{}/{}?{}", DATA_URL, url, params))
                                 .send()
                                 .chain_err(|| "Could not get note")?;
        let mut body = String::new();
        let _ = res.read_to_string(&mut body).chain_err(|| "Could not ready body")?;
        let v: Note = json::from_str(&body).chain_err(|| "Could not decode Note")?;
        Ok(v)
    }

    pub fn add_note<I: Into<Note>>(&self, note: I) -> Result<Note> {
        let mut new_note = note.into();
        self.update_note(&mut new_note)
    }

    pub fn update_note(&self, note: &mut Note) -> Result<Note> {
        let url = if note.has_key() {
            let _ = note.set_modified().chain_err(|| "Could not set modified date")?;
            format!("{}/{}?auth={}&email={}",
                    DATA_URL,
                    note.key().unwrap(),
                    self.token()?,
                    self.username)
        } else {
            format!("{}?auth={}&email={}",
                    DATA_URL,
                    self.token()?,
                    self.username)
        };

        let mut res = self.client.post(&url)
                                 .json(&note)
                                 .send()
                                 .chain_err(|| "Could not update note")?;
        let mut body = String::new();
        let _ = res.read_to_string(&mut body).chain_err(|| "Could not ready body")?;
        let v: Note = json::from_str(&body).chain_err(|| "Could not decode Note")?;
        Ok(v)
    }

    pub fn trash_note(&self, note_id: &str) -> Result<Note> {
        let mut note = self.get_note(note_id)?;
        let _ = note.delete()?;
        self.update_note(&mut note)
    }

    pub fn delete_note(&self, note_id: &str) -> Result<()> {
        let _ = self.trash_note(note_id)?;
        let url = format!("{}/{}?auth={}&email={}",
                          DATA_URL,
                          note_id,
                          self.token()?,
                          self.username);
        let _ = self.client.request(Method::Delete, &url)
                                 .send()
                                 .chain_err(|| "Could not delete note")?;
        Ok(())
    }

    /// Get notes list
    ///
    /// TODO: implement filters
    pub fn notes(&self) -> Result<Vec<Note>> {
        self.notes_filtered(vec![])
    }

    pub fn notes_filtered(&self, _filters: Vec<Filter>) -> Result<Vec<Note>> {
        let mut mark = "mark".to_string();
        let mut notes = vec![];
        loop {
            let params = format!("auth={}&email={}&length={}&mark={}", 
                                    self.token()?,
                                    self.username,
                                    NOTE_LIST_LENGTH,
                                    mark);

            let url = format!("{}{}", INDEX_URL, params);
            let mut res = self.client.get(&url).send().chain_err(|| "Could not get note list")?;
            let mut body = String::new();
            let _ = res.read_to_string(&mut body).chain_err(|| "Could not read note list")?;
            let notes_resp: NotesResponse = json::from_str(&body).chain_err(|| "Could not decode note list JSON")?;
            notes.extend(notes_resp.data);
            if notes_resp.mark.is_none() {
                break
            } else {
                mark = notes_resp.mark.unwrap();
            }
        }
        Ok(notes)
    }

    fn token(&self) -> Result<String> {
        if self.token.borrow().is_some() {
            let s = self.token.borrow().clone();
            return Ok(s.unwrap());
        }
        self.auth()
    }

    pub fn auth(&self) -> Result<String> {
        let auth_params = format!("email={}&password={}", self.username, self.password);
        debug!("Using auth params: {}", auth_params);
        let encoded = base64::encode(auth_params.as_bytes());
        debug!("base64-encoded auth params: {}", encoded);
        let mut res = self.client.post(AUTH_URL)
                             .body(encoded)
                             .header(ContentType::form_url_encoded())
                             .send()
                             .chain_err(|| "Could not log in to simplenote.com")?;
        if ! res.status().is_success() {
            debug!("Response was `{:?}`", &res);
            return Err(format!("Authentication was not successful. Status was {}", &res.status()).into());
        }
        let mut token = String::new();
        debug!("Token is {}", token);
        let _  = res.read_to_string(&mut token).chain_err(|| "Could not get token from response")?;
        *self.token.borrow_mut() = Some(token.clone());
        Ok(token)
    }

}

pub enum Filter {
    Since(String),
    Tag(String),
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct NotesResponse {
    count: u64,
    pub data: Vec<Note>,
    time: f64,
    pub mark: Option<String>,
}


