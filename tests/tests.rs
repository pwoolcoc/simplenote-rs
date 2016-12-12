extern crate simplenote;

use simplenote::{Simplenote, Note};

fn get_creds() -> (String, String) {
    let em = env!("SIMPLENOTE_EMAIL");
    let pw = env!("SIMPLENOTE_PASSWORD");
    (em.to_string(), pw.to_string())
}

fn delete_note_helper(client: &Simplenote, n: &Note) {
    if let Some(key) = n.key() {
        client.delete_note(key).expect("Error while trying to delete test note");
    } else {
        panic!("ERROR: note passed to delete_note_helper does not have a key");
    }
}

#[test]
fn test_login() {
    let (em, pw) = get_creds();
    let client = Simplenote::new(em, pw);

    assert!(client.is_ok());
}

#[test]
fn add_note_from_str() {
    let (em, pw) = get_creds();
    let content = "Test Note\n\nThis is a new note";
    let client = Simplenote::new(em, pw).expect("Could not get simplenote client");
    let new_note = client.add_note(content).expect("Could not add new note from str");

    delete_note_helper(&client, &new_note);
}

#[test]
fn add_note_from_note() {
    let (em, pw) = get_creds();
    let content = "Test Note\n\nThis is a new note";
    let client = Simplenote::new(em, pw).expect("Could not get simplenote client");
    let note = Note::new().content(content);
    let new_note = client.add_note(note).expect("Could not add new note from Note");

    delete_note_helper(&client, &new_note);
}

#[test]
fn delete_note() {
    let (em, pw) = get_creds();
    let content = "Test Note\n\nThis is a new note";
    let client = Simplenote::new(em, pw).expect("Could not get simplenote client");
    let new_note = client.add_note(content).expect("Could not add new note from string");
    delete_note_helper(&client, &new_note);
}

#[test]
fn update_note() {
    let (em, pw) = get_creds();
    let content = "Test Note\n\nThis is a new note";
    let client = Simplenote::new(em, pw).expect("Could not get simplenote client");
    let note = client.add_note(content).expect("Colud not add new note from string");
    let original_key = {
        let key = note.key().expect("New note doesn't have a key?");
        key.to_string()
    };
    let mut new_note = note.content("Another Test Note\n\nThis has some new content"); 
    client.update_note(&mut new_note);
}
