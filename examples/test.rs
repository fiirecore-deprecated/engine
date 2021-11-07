use engine::{ContextBuilder, DefaultContext, graphics::{self, scaling::ScreenScaler, Color}, gui::{MessageBox, Panel}, state::State, text::{Message, MessagePage, TextColor}, util::{Completable, Entity}};
use firecore_engine as engine;

fn main() {
    engine::run(
        ContextBuilder::new(
            "MessageBox",
            2 * engine::util::WIDTH as i32,
            (2.0 * engine::util::HEIGHT) as _,
        ),
        move |context| DefaultContext(context),
        |_| Game::new(),
    )
}

struct Game {
    messagebox: MessageBox,
}

impl Game {
    pub fn new() -> Self {
        Self {
            messagebox: MessageBox::new(Default::default(), 0),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl State<DefaultContext> for Game {
    async fn start(&mut self, ctx: &mut DefaultContext) {
        let fonts: Vec<engine::text::FontSheet<Vec<u8>>> =
            bincode::deserialize(include_bytes!("fonts.bin"))
                .unwrap();
            
        let mut audio: engine::context::audio::SerializedAudio = bincode::deserialize(include_bytes!("audio.bin")).unwrap();

        let id = "battle_wild".parse().unwrap();

        engine::audio::add_music(ctx, id, audio.0.remove(&id).unwrap()).await;

        // engine::context::audio::GameAudio::init(ctx, audio).await;

        engine::audio::play_music(ctx, &id);

        for font_sheet in fonts {
            engine::text::insert_font(ctx, &font_sheet).unwrap();
        }

        let scaler = ScreenScaler::with_size(
            ctx,
            engine::util::WIDTH as _,
            engine::util::HEIGHT as _,
            graphics::scaling::ScalingMode::ShowAllPixelPerfect,
        );

        engine::graphics::scaling::set_scaler(ctx, scaler);

        //-> Result {
        let page = MessagePage {
            lines: vec![
                "Test Page Test Page".to_owned(),
                "Page Test Page Test".to_owned(),
            ],
            wait: None,
        };
        let page2 = MessagePage {
            lines: page.lines.clone(),
            wait: Some(1.0),
        };
        self.messagebox.message = Message {
            pages: vec![page, page2],
            color: TextColor::Black,
        };
        self.messagebox.spawn();
        // Ok(())
    }

    fn update(&mut self, ctx: &mut DefaultContext, delta: f32) {
        //-> Result {
        if !self.messagebox.alive() {
            engine::quit(ctx)
        } else {
            self.messagebox.update(ctx, delta);
            if self.messagebox.finished() {
                self.messagebox.despawn();
            }
        }
        // Ok(())
    }

    fn draw(&mut self, ctx: &mut DefaultContext) {
        //-> Result<(), ()> {
        graphics::clear(ctx, Color::rgb(0.1, 0.2, 0.56));
        Panel::draw(ctx, 10.0, 10.0, engine::util::WIDTH - 20.0, engine::util::HEIGHT - 20.0);
        self.messagebox.draw(ctx);
        // Ok(())
    }
}