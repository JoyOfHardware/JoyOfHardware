use zoon::*;

mod landing;
mod login;

pub fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .on_viewport_size_change(|width, _| {
            crate::WINDOW_SIZE.set_neq(width as u32);
        })
        .child_signal(crate::router::ROUTER.route().signal_cloned().map(
            |route| match route {
                NoRoute => None,
                UnknownRoute => El::new().child("404").unify_option(),
                KnownRoute(route) => match route {
                    crate::router::Route::Landing => landing::page().unify_option(),
                    crate::router::Route::Login => login::LoginPage::new().unify_option(),
                }

            }))
}