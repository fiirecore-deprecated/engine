use core::ops::Deref;

use pokedex::{
    engine::{
        graphics::{draw_cursor, draw_text_left, DrawParams},
        gui::Panel,
        input::controls::{pressed, Control},
        text::TextColor,
        Context,
    },
    pokemon::{owned::OwnablePokemon, Pokemon},
};

pub struct BattleOptions {
    buttons: [&'static str; 4],
    pokemon_do: String,
    pub cursor: usize,
}

impl BattleOptions {
    pub fn new() -> Self {
        Self {
            buttons: ["FIGHT", "BAG", "POKEMON", "RUN"],
            pokemon_do: String::new(),
            cursor: 0,
        }
    }

    pub fn setup<P: Deref<Target = Pokemon>, M, I, G, H>(&mut self, instance: &OwnablePokemon<P, M, I, G, H>) {
        self.pokemon_do = format!("{} do?", instance.name());
    }

    pub fn input(&mut self, ctx: &Context) {
        if pressed(ctx, Control::Up) && self.cursor >= 2 {
            self.cursor -= 2;
        } else if pressed(ctx, Control::Down) && self.cursor <= 2 {
            self.cursor += 2;
        } else if pressed(ctx, Control::Left) && self.cursor > 0 {
            self.cursor -= 1;
        } else if pressed(ctx, Control::Right) && self.cursor < 3 {
            self.cursor += 1;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        Panel::draw(ctx, 120.0, 113.0, 120.0, 47.0);

        draw_text_left(ctx, &1, "What will", 11.0, 123.0, DrawParams::color(TextColor::White.into()));
        draw_text_left(ctx, &1, &self.pokemon_do, 11.0, 139.0, DrawParams::color(TextColor::White.into()));

        for (index, string) in self.buttons.iter().enumerate() {
            draw_text_left(
                ctx,
                &0,
                string,
                138.0 + if index % 2 == 0 { 0.0 } else { 56.0 },
                123.0 + if index >> 1 == 0 { 0.0 } else { 16.0 },
                DrawParams::color(TextColor::Black.into())
            )
        }

        draw_cursor(
            ctx,
            131.0 + if self.cursor % 2 == 0 { 0.0 } else { 56.0 },
            126.0 + if (self.cursor >> 1) == 0 { 0.0 } else { 16.0 },
            Default::default(),
        );
    }
}