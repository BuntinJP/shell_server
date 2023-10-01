pub trait Endpoint {
    fn path() -> &'static str;
    fn configure(cfg: &mut actix_web::web::ServiceConfig);
}

mod hello_world;
mod keys_register;

pub use self::hello_world::HelloWorld;
pub use self::keys_register::KeysRegister;
