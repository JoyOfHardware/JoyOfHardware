use zoon::*;

pub static ROUTER: Lazy<Router<Route>> = lazy::default();

#[route]
#[derive(Clone)]
pub enum Route {
    #[route()]
    Landing,
    #[route("login")]
    Login
}