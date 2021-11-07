use core::ops::Deref;

use pokedex::{
    engine::{
        gui::MessageBox,
        input::controls::{pressed, Control},
        text::{MessagePage, TextColor},
        util::{Completable, Entity},
        Context,
    },
    moves::{owned::OwnedMove, set::OwnedMoveSet, Move},
    pokemon::{owned::OwnablePokemon, Pokemon},
};

use super::moves::MovePanel;

pub struct LevelUpMovePanel<M: Deref<Target = Move> + Clone> {
    state: LevelUpState,

    move_panel: MovePanel<M>,

    moves: Vec<M>,
}

enum LevelUpState {
    NotAlive,
    Text,
    Moves,
}

impl<M: Deref<Target = Move> + Clone> LevelUpMovePanel<M> {
    pub fn new() -> Self {
        Self {
            state: LevelUpState::NotAlive,
            move_panel: MovePanel::new(),
            moves: Vec::new(),
        }
    }

    pub fn spawn<P, MSET: Deref<Target = [OwnedMove<M>]>, I, G, H>(
        &mut self,
        instance: &OwnablePokemon<P, MSET, I, G, H>,
        text: &mut MessageBox,
        moves: Vec<M>,
    ) {
        self.state = LevelUpState::Text;
        self.moves = moves;
        self.move_panel.update_names(instance);
        text.despawn();
    }

    pub fn update<P: Deref<Target = Pokemon>, I, G, H>(
        &mut self,
        ctx: &Context,
        text: &mut MessageBox,
        delta: f32,
        pokemon: &mut OwnablePokemon<P, OwnedMoveSet<M>, I, G, H>,
    ) -> Option<(usize, M)> {
        match self.state {
            LevelUpState::Text => match text.alive() {
                true => {
                    text.update(ctx, delta);
                    if text.finished() {
                        self.state = LevelUpState::Moves;
                        text.despawn();
                    }
                    None
                }
                false => match self.moves.first() {
                    Some(move_ref) => {
                        text.spawn();
                        text.push(MessagePage {
                            lines: vec![
                                format!("{} is trying to", pokemon.name()),
                                format!("learn {}", move_ref.name),
                            ],
                            wait: None,
                        });
                        self.update(ctx, text, delta, pokemon)
                    }
                    None => {
                        self.state = LevelUpState::NotAlive;
                        None
                    }
                },
            },
            LevelUpState::Moves => {
                self.move_panel.input(ctx);
                let a = pressed(ctx, Control::A);
                if pressed(ctx, Control::B) || a {
                    self.state = LevelUpState::Text;
                    let pokemon_move = self.moves.remove(0);
                    if a {
                        self.move_panel.names[self.move_panel.cursor] =
                            Some((pokemon_move.clone(), TextColor::Black));
                        pokemon
                            .moves
                            .add(Some(self.move_panel.cursor), pokemon_move.clone());
                        return Some((self.move_panel.cursor, pokemon_move));
                    }
                }
                None
            }
            LevelUpState::NotAlive => None,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        match self.state {
            LevelUpState::Moves => self.move_panel.draw(ctx),
            LevelUpState::Text | LevelUpState::NotAlive => (),
        }
    }

    pub fn alive(&self) -> bool {
        !matches!(self.state, LevelUpState::NotAlive)
    }
}