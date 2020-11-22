use crate::driver::server;

mod driver;

fn main() {
    server.
}
// use crate::entity::user::User;
// use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
// 
// mod entity;
// 
// async fn index() -> impl Responder {
//     let hoge = User::new("4".to_string(), "佐々木".to_string());
//     HttpResponse::Ok().body(User::get_name(&hoge))
// }
// 
// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             // .wrap(middleware::Logger::default())
//             .route("/", web::get().to(index))
//         // .route("/again", web::get().to(index2))
//     })
//     .bind("127.0.0.1:9000")?
//     .run()
//     .await
// }
