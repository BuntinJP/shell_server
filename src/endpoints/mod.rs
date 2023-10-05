pub trait Endpoint {
    fn path() -> &'static str;
    fn configure(cfg: &mut actix_web::web::ServiceConfig);
}

mod ep_utils;
mod hello_world;
mod keys_register;
mod sudo;
mod users;

pub use self::hello_world::HelloWorld;
pub use self::keys_register::KeysRegister;
pub use self::sudo::Sudo;
pub use self::users::Users;
