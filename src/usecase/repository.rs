use crate::domain::entity::User;

pub trait UserRepository {
    fn all(&self) -> Vec<User>;
    fn by_id(&self, id: String) -> Result<User, String>;
    // fn save(&self, client: User);
    // fn next_identity(&self) -> String;
}
