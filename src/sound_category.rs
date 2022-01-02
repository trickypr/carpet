use orbtk::prelude::*;

use crate::{
    sound::{self, SoundMetadata},
    MainState, SoundCategory,
};

pub fn sound_display(
    ctx: &mut BuildContext,
    sound: &sound::SoundMetadata,
    parent_id: Entity,
    category_id: usize,
) -> Entity {
    let sound_id = sound.id.clone();

    Stack::new()
        .child(TextBlock::new().text(sound.name.clone()).build(ctx))
        .child(
            Slider::new()
                .id(format!("sound_{}", sound.id.clone()))
                .min(0.0)
                .val(sound.volume.clone())
                .max(1.0)
                .min_width(100)
                .on_changed("val", move |states, widget_id| {
                    states
                        .get_mut::<MainState>(parent_id)
                        .change_volume(((category_id, sound_id), widget_id));
                })
                .build(ctx),
        )
        .build(ctx)
}

pub fn display(
    ctx: &mut BuildContext,
    category: &SoundCategory,
    parent_id: Entity,
    category_id: usize,
) -> Entity {
    let available_sounds = category
        .sounds
        .iter()
        .map(|sound| sound.into())
        .collect::<Vec<SoundMetadata>>();

    Stack::new()
        .child(
            TextBlock::new()
                .text(category.name.clone())
                .style("header")
                .build(ctx),
        )
        .child(
            ItemsWidget::new()
                .id(format!("category_{}", category.id.clone()))
                .count(available_sounds.len())
                .items_builder(move |ctx, index| {
                    Container::new()
                        .child(sound_display(
                            ctx,
                            &available_sounds[index],
                            parent_id,
                            category_id,
                        ))
                        .padding(16)
                        .build(ctx)
                })
                .build(ctx),
        )
        .spacing(8)
        .build(ctx)
}
