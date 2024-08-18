use zoon::*;

mod landing;
mod login;

pub fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .child_signal(crate::router::ROUTER.route().signal_cloned().map(
            |route| match route {
                NoRoute => None,
                UnknownRoute => El::new().child("404").unify_option(),
                KnownRoute(route) => match route {
                    crate::router::Route::Landing => landing::page().unify_option(),
                    crate::router::Route::Login => login::page().unify_option(),
                }

            }))
}