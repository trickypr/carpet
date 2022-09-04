use crate::sounds::{Category, Holder, Sound};

type SimpleSound<'a> = (&'a str, &'a str);

impl Holder {
    fn add_sound(&mut self, sound: Sound) {
        self.sounds.push(sound);
    }

    fn category(
        &mut self,
        category: Category,
        sounds: &[SimpleSound],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (name, path) in sounds {
            let sound = Sound::new(self, category, name, &format!("./sounds/{}.ogg", path))?;
            self.add_sound(sound);
        }

        Ok(())
    }

    #[inline]
    pub fn register_sounds(&mut self) {
        self.category(
            Category::Water,
            &[
                ("Rain", "rain"),
                ("Thunder", "storm"),
                ("Stream", "stream"),
                ("Waves", "waves"),
                ("Boat", "boat"),
            ],
        )
        .expect("Error loading sound category: Water");

        self.category(
            Category::Nature,
            &[
                ("Birds", "birds"),
                ("Wind", "wind"),
                ("Summer Night", "summer-night"),
            ],
        )
        .expect("Error loading sound category: Nature");

        self.category(
            Category::Humans,
            &[
                ("City", "city"),
                ("Coffee Shop", "coffee-shop"),
                ("Fireplace", "fireplace"),
                ("Train", "train"),
            ],
        )
        .expect("Error loading sound category: Humans");

        self.category(
            Category::Artificial,
            &[("Pink Noise", "pink-noise"), ("White Noise", "white-noise")],
        )
        .expect("Error loading sound category: Artificial");
    }
}
