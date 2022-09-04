use crate::{sounds::SoundMetadata, ControlThreadCommand, RX};

use super::prelude::*;

#[derive(Props, PartialEq)]
pub struct SoundProps {
    sound: SoundMetadata,
}

pub fn Sound(cx: Scope<SoundProps>) -> Element {
    let sound = &cx.props.sound;

    let volume_change = move |vol| {
        let rx = unsafe { RX.as_ref().unwrap() };
        rx.send(ControlThreadCommand::ChangeVolume(sound.id, vol))
            .unwrap();
    };

    cx.render(rsx! {
        view {
            height: "49",
            width: "100%",

            text {
                "{&sound.name}"
            },
            Slider {
                min: 0.0,
                max: 1.0,
                val: sound.volume,
                onchange: volume_change
            }
        },
    })
}
