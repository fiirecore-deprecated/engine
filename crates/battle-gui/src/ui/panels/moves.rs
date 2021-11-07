use core::ops::Deref;

use pokedex::{
    engine::{
        graphics::{draw_cursor, draw_text_left, DrawParams},
        gui::Panel,
        input::controls::{pressed, Control},
        text::TextColor,
        util::Reset,
        Context,
    },
    moves::{Move, owned::OwnedMove},
    pokemon::owned::OwnablePokemon,
};

pub struct MovePanel<M: Deref<Target = Move> + Clone> {
    pub cursor: usize,
    pub names: [Option<(M, TextColor)>; 4],
}

impl<M: Deref<Target = Move> + Clone> MovePanel<M> {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            names: Default::default(),
        }
    }

    pub fn update_names<P, MSET: Deref<Target = [OwnedMove<M>]>, I, G, H>(&mut self, instance: &OwnablePokemon<P, MSET, I, G, H>) {
        for (index, instance) in instance.moves.iter().enumerate() {
            self.names[index] = Some((
                instance.0.clone(),
                if instance.is_empty() {
                    TextColor::Red
                } else {
                    TextColor::Black
                },
            ));
        }
    }

    pub fn input(&mut self, ctx: &Context) -> bool {
        if {
            if pressed(ctx, Control::Up) {
                if self.cursor >= 2 {
                    self.cursor -= 2;
                    true
                } else {
                    false
                }
            } else if pressed(ctx, Control::Down) {
                if self.cursor <= 2 {
                    self.cursor += 2;
                    true
                } else {
                    false
                }
            } else if pressed(ctx, Control::Left) {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    true
                } else {
                    false
                }
            } else if pressed(ctx, Control::Right) {
                if self.cursor < 3 {
                    self.cursor += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } {
            if self.cursor >= self.names.len() {
                self.cursor = self.names.len() - 1;
            }
            true
        } else {
            false
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        Panel::draw(ctx, 0.0, 113.0, 160.0, 47.0);
        for (index, (pokemon_move, color)) in self.names.iter().flatten().enumerate() {
            let x_offset = if index % 2 == 1 { 72.0 } else { 0.0 };
            let y_offset = if index >> 1 == 1 { 17.0 } else { 0.0 };
            draw_text_left(
                ctx,
                &0,
                &pokemon_move.name,
                16.0 + x_offset,
                121.0 + y_offset,
                DrawParams::color((*color).into())
            );
            if index == self.cursor {
                draw_cursor(ctx, 10.0 + x_offset, 123.0 + y_offset, Default::default());
            }
        }
    }
}

impl<M: Deref<Target = Move> + Clone> Reset for MovePanel<M> {
    fn reset(&mut self) {
        self.cursor = 0;
    }
}