#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub text: Option<String>,
    pub published: bool,
}
