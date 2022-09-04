// use orbtk::prelude::*;

use dioxus::prelude::*;
use elements_namespace as dioxus_elements;

use crate::{
    components_int::Slider,
    sound::{self, SoundMetadata},
    sounds::SoundCategoryLite,
    ControlThreadCommand,
    // MainState,
    SoundCategory,
    RX,
};

#[derive(PartialEq, Props)]
struct SoundProps {
    sound: SoundMetadata,
    category_id: usize,
    sound_index: usize,
    category_index: usize,
}

fn Sound<'a>(cx: Scope<'a, SoundProps>) -> Element {
    let sound = &cx.props.sound;
    let category_index = cx.props.category_index;
    let sound_index = cx.props.sound_index;

    let volume = cx.props.sound.volume;
    let volume_change = move |vol| {
        let rx = unsafe { RX.as_ref().unwrap() };
        rx.send(ControlThreadCommand::ChangeVolume(
            (category_index, sound_index),
            vol,
        ))
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
                val: volume,
                onchange: volume_change
            },
        },
    })
}

#[derive(PartialEq, Props)]
pub struct AllSoundsProps {
    category: SoundCategoryLite,
    category_index: usize,
}

pub fn AllSounds<'a>(cx: Scope<'a, AllSoundsProps>) -> Element {
    let category = &cx.props.category;
    let category_index = cx.props.category_index;

    let available_sounds = &category.sounds;
    let est_height = 49 * available_sounds.len() + 17;

    let sounds_els = available_sounds
        .iter()
        .map(|sound| sound.clone())
        .enumerate()
        .map(|(i, sound)| {
            rsx!(Sound {
                sound: sound,
                category_id: category.id,
                category_index: category_index,
                sound_index: i
            })
        });

    cx.render(rsx! {
        view {
            height: "{&est_height}",
            width: "100%",
            text {
                "{&category.name}"
            },
            sounds_els
        }
    })
}

// pub fn sound_display(
//     ctx: &mut BuildContext,
//     sound: &sound::SoundMetadata,
//     parent_id: Entity,
//     category_id: usize,
// ) -> Entity {
//     let sound_id = sound.id.clone();

//     Stack::new()
//         .child(TextBlock::new().text(sound.name.clone()).build(ctx))
//         .child(
//             Slider::new()
//                 .id(format!("sound_{}", sound.id.clone()))
//                 .min(0.0)
//                 .val(sound.volume.clone())
//                 .max(1.0)
//                 .min_width(100)
//                 .on_changed("val", move |states, widget_id| {
//                     states
//                         .get_mut::<MainState>(parent_id)
//                         .change_volume(((category_id, sound_id), widget_id));
//                 })
//                 .build(ctx),
//         )
//         .build(ctx)
// }

// pub fn display(
//     ctx: &mut BuildContext,
//     category: &SoundCategory,
//     parent_id: Entity,
//     category_id: usize,
// ) -> Entity {
//     let available_sounds = category
//         .sounds
//         .iter()
//         .map(|sound| sound.into())
//         .collect::<Vec<SoundMetadata>>();

//     Stack::new()
//         .child(
//             TextBlock::new()
//                 .text(category.name.clone())
//                 .style("header")
//                 .build(ctx),
//         )
//         .child(
//             ItemsWidget::new()
//                 .id(format!("category_{}", category.id.clone()))
//                 .count(available_sounds.len())
//                 .items_builder(move |ctx, index| {
//                     Container::new()
//                         .child(sound_display(
//                             ctx,
//                             &available_sounds[index],
//                             parent_id,
//                             category_id,
//                         ))
//                         .padding(16)
//                         .build(ctx)
//                 })
//                 .build(ctx),
//         )
//         .spacing(8)
//         .build(ctx)
// }
