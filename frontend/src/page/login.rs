use chrono::format::Pad;
use zoon::*;
use zoon::eprintln;
use std::sync::Arc;

#[derive(Clone)]
pub struct LoginPage {
    username: Mutable<Arc<String>>,
    password: Mutable<Arc<String>>
}


impl LoginPage {
    pub fn new() -> impl Element {
        Self {
            username: Mutable::new(Arc::new("".to_string())),
            password: Mutable::new(Arc::new("".to_string()))
        }.page()
    }

    fn login_form(&self) -> impl Element {
        let is_mobile = crate::WINDOW_SIZE.signal().map(crate::theme::is_mobile);
        
        let adventure = El::new()
            .s(Font::new().size(50).weight(FontWeight::ExtraBold))
            .s(Align::new().left())
            .s(Font::new().color(color!("gray")))
            .child("Adventure");

        let awaits = El::new()
            .s(Padding::new().bottom(10))
            .s(Font::new().size(50).weight(FontWeight::ExtraBold))
            .s(Align::new().right())
            .s(Font::new().color(color!("gray")))
            .child("Awaits!");

        let title = Column::new()
            .s(Align::new().center_x())
            .s(Width::fill())
            .s(Padding::new().bottom(25))
            .items(vec![
                adventure.unify(),
                awaits.unify(),
            ]);
        
            
            let login_and_sign_up_button_row = 
            {
                let login_button = crate::assets::button_impact(
                    "Let Me In!!", 
                    crate::theme::fill_green, 
                    crate::theme::border_green, 
                    || {}
                );
        
                let sign_up_button = crate::assets::button_impact(
                    "Sign Me Up!!", 
                    crate::theme::fill_orange, 
                    crate::theme::border_orange, 
                    || {}
                );

                Row::new()
                    .s(Align::new().center_x())
                    .s(AlignContent::new().center_x())
                    .s(Padding::new().top(50))
                    .s(Width::fill())
                    // .s(Height::fill())
                    .s(Gap::new().x(50))
                    // .multiline()
                    .items(vec![
                        sign_up_button.unify(),
                        login_button.unify()
                    ])
            };

        let column = Column::new()
            .s(Width::fill())
            .s(Height::fill())
            .s(Padding::new().y(25))
            .s(Gap::new().y(25))
            .items(vec![
                title.unify(),
                crate::assets::form_input_field(
                    "Username",
                    self.username.clone(),
                    InputType::text(),
                    clone!((self => s) move |name| s.username.set(Arc::new(name)))
                ).unify(),
                crate::assets::form_input_field(
                    "Password",
                    self.password.clone(),
                    InputType::password(),
                    clone!((self => s) move |name| s.password.set(Arc::new(name)))
                )
                .unify(),
                login_and_sign_up_button_row.unify()
            ]);

        El::new()
            .s(Align::center())
            .s(Width::fill())
            .s(Height::fill())
            .s(Padding::new()
                .x_signal(
                    crate::WINDOW_SIZE.signal().map(crate::theme::is_mobile)
                        .map_bool(|| 15, || 35)
                    )
                )
            .s(Padding::new().x(25))
            .s(RoundedCorners::all(25))
            // show a rounded shadow on desktop
            .s(Shadows::with_signal(
                crate::WINDOW_SIZE.signal().map(crate::theme::is_mobile)
                    .map_bool(
                        || [Shadow::new()],
                        || [Shadow::new().blur(50).color("#dddddd")],
                    )
                )
            )
            .child(column)
    }

    pub fn page(&self) -> impl Element {

        El::new()
            .s(Align::center())
            .s(Width::percent(90).max(400))
            .s(Height::percent(90).max(500))
            .child(self.login_form())
    }

}
