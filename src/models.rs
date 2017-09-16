use super::schema::notes;

#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub text: Option<String>,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote<'n> {
    pub title: &'n str,
    pub text: &'n str,
}
