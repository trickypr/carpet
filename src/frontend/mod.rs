mod category;
mod sound;

use freya::prelude::*;

use crate::{get_holder, sounds::Category};

pub fn app(cx: Scope) -> Element {
    let holder = get_holder();
    let sounds = &holder.sounds;

    let categories = Category::get_all();

    render!(
        container {
            height: "100%",
            width: "100%",
            padding: "60",

            ScrollView {
                categories.iter().map(move |category| {
                    render!(category::category {
                        category: *category,
                        sounds: sounds.iter().map(|sound| sound.into()).collect()
                    })
                })
            }
        }
    )
}
