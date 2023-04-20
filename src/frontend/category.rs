use crate::sounds::{Category, SoundMetadata};

use super::sound::Sound;
use freya::prelude::*;

#[derive(Props, PartialEq)]
pub struct CategoryProps {
    category: Category,
    sounds: Vec<SoundMetadata>,
}

pub fn category(cx: Scope<CategoryProps>) -> Element {
    let category = cx.props.category;
    let sounds = &cx.props.sounds;

    render!(
        container {
            width: "100%",
            padding: "32",

            label {
                font_size: "30",
                "{&category}"
            },
            sounds
                .iter()
                .filter(|sound| sound.category == category)
                .map(|sound| {
                    render!(Sound {
                        sound: sound.clone(),
                    })
                })
        }
    )
}
