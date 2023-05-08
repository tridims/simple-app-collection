#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
}
