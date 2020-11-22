pub struct GetAllUser<'a> {
    client_repository: &'a dyn ClientRepository
}

impl<'a> GetAllUser<'a> {
    pub fn new(client_repository: &dyn ClientRepository) -> GetClientUseCaseHandler {
        return GetClientUseCaseHandler {
            client_repository
        }
    }

    pub fn execute(&self, request: GetClientUseCaseRequest) -> Result<Client, String>{
        return self.client_repository.by_id(String::from(request.client_id));
    }
}
