mod category;
mod prelude;
mod sound;

use prelude::*;

use crate::{get_holder, sounds::Category};

pub fn app(cx: Scope) -> Element {
    let holder = get_holder();
    let sounds = &holder.sounds;

    let categories = Category::get_all();
    let categories = categories.iter().map(move |category| {
        cx.render(rsx!(category::category {
            category: *category,
            sounds: sounds.iter().map(|sound| sound.into()).collect()
        }))
    });

    cx.render(rsx! {
        container {
            height: "100%",
            width: "100%",
            padding: "60",
            background: "black",

            ScrollView {
                categories
            }
        }
    })
}
