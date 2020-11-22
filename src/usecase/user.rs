// use crate::usecase::repository::UserRepository;
// use crate::application::requests::{CreateClientUseCaseRequest, GetClientUseCaseRequest};
use crate::entity::User;

pub struct GetUserUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> GetUserUseCase<'a> {
    pub fn new(client_repository: &dyn ClientRepository) -> GetUserUseCase {
        return GetUserUseCase { client_repository };
    }

    pub fn execute(&self, request: GetUserUseCaseRequest) -> Result<User, String> {
        return self.user_repository.by_id(String::from(request.user_id));
    }
}

pub struct GetUserUseCaseRequest<'a> {
    pub user_id: &'a str,
}

impl<'a> GetUserUseCaseRequest<'a> {
    pub fn new(user_id: &'a str) -> GetUserUseCaseRequest {
        return GetUserUseCaseRequest { user_id };
    }
}
