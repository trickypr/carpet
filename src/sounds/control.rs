use std::sync::{mpsc::Receiver, MutexGuard};

use crate::{config::Config, ControlThreadCommand, SOUND};

use super::Holder;

fn get_holder<'a>() -> MutexGuard<'a, Holder> {
    unsafe { SOUND.as_ref().unwrap().lock().unwrap() }
}

fn sync_config(config: &Config) {
    let mut holder = get_holder();

    holder.is_playing = config.is_playing.unwrap_or(true);

    for sound in &mut holder.sounds {
        if let Some(config_volume) = config.sound_volume.get(&sound.name) {
            sound.volume = *config_volume;
        }
    }

    holder.correct_volume();
}

pub fn control(tx: Receiver<ControlThreadCommand>) {
    let mut config = Config::load();
    sync_config(&config);

    loop {
        if let Ok(command) = tx.recv() {
            let mut holder = get_holder();

            match command {
                crate::ControlThreadCommand::ChangeVolume(id, volume) => {
                    let sound = holder.get_sound_mut(id);

                    if let Some(sound) = sound {
                        sound.volume = volume;
                        config.sound_volume.insert(sound.name.clone(), volume);
                        holder.correct_volume();
                    }
                }
                crate::ControlThreadCommand::SetPlaying(local_is_playing) => {
                    holder.is_playing = local_is_playing;
                    holder.correct_volume();

                    config.is_playing = Some(local_is_playing);
                }
            }

            config.save();
        }
    }
}
