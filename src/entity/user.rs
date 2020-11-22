#[derive(Debug, Clone)]
pub struct User {
    id: String,   // ユーザID
    name: String, //ユーザ名
}

impl User {
    pub fn new(id: String, name: String) -> User {
        User { id, name }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
