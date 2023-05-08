use crate::server::config::{DB_URL, INTERNAL_ERROR, NOT_FOUND, OK_RESPONSE};
use crate::server::database::get_connection;
use crate::server::model::Note;
use crate::server::utils::{get_id, get_user_request_body};

// Controllers
pub fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), get_connection(DB_URL)) {
        (Ok(note), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO notes (title, body) VALUES ($1, $2)",
                    &[&note.title, &note.body],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Note created".to_string())
        }
        _ => (
            INTERNAL_ERROR.to_string(),
            "Unable to create note".to_string(),
        ),
    }
}

pub fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), get_connection(DB_URL)) {
        (Ok(id), Ok(mut client)) => {
            match client.query_one("SELECT * FROM notes WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let note = Note {
                        id: Some(row.get(0)),
                        title: row.get(1),
                        body: row.get(2),
                    };

                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string(&note).unwrap(),
                    )
                }
                _ => (NOT_FOUND.to_string(), "Note not found".to_string()),
            }
        }
        _ => (INTERNAL_ERROR.to_string(), "Unable to get note".to_string()),
    }
}

pub fn handle_get_all_request(_request: &str) -> (String, String) {
    match get_connection(DB_URL) {
        Ok(mut client) => {
            let mut notes = vec![];
            for row in client.query("SELECT * FROM notes", &[]).unwrap() {
                notes.push(Note {
                    id: Some(row.get(0)),
                    title: row.get(1),
                    body: row.get(2),
                });
            }

            (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&notes).unwrap(),
            )
        }
        _ => (
            INTERNAL_ERROR.to_string(),
            "Unable to get notes".to_string(),
        ),
    }
}

pub fn handle_put_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        get_user_request_body(&request),
        get_connection(DB_URL),
    ) {
        (Ok(id), Ok(note), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE notes SET title = $1, body = $2 WHERE id = $3",
                    &[&note.title, &note.body, &id],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "Note updated".to_string())
        }
        _ => (
            INTERNAL_ERROR.to_string(),
            "Unable to update note".to_string(),
        ),
    }
}

pub fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), get_connection(DB_URL)) {
        (Ok(id), Ok(mut client)) => {
            let rows = client
                .execute("DELETE FROM notes WHERE id = $1", &[&id])
                .unwrap();

            if rows == 0 {
                return (NOT_FOUND.to_string(), "Note not found".to_string());
            }

            (OK_RESPONSE.to_string(), "Note deleted".to_string())
        }
        _ => (
            INTERNAL_ERROR.to_string(),
            "Unable to delete note".to_string(),
        ),
    }
}
