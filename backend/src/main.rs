use actix_web::{middleware::Logger, App, HttpServer};
#[cfg(feature = "ui")]
use actix_web_static_files;

#[cfg(feature = "ui")]
use {{crate_name}}_frontend::generate;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        let mut app = App::new().wrap(Logger::default());
        #[cfg(feature = "ui")]
        {
            let generated = generate();
            app = app.service(actix_web_static_files::ResourceFiles::new(
                "/", generated,
            ));
        }
        app
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
