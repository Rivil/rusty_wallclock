use chrono::{DateTime, Duration, Local};
use futures::stream::Stream;
use yew::platform::time::interval;
use yew::{html, AttrValue, Component, Context, Html};

enum Msg {
    ClockTicked(DateTime<Local>),
}

pub struct ClockComponent {
    clock: Option<AttrValue>,
}

pub fn stream_time() -> impl Stream<Item = DateTime<Local>> {
    interval(Duration::from_secs(1)).map(|_| Local::now())
}

impl Component for ClockComponent {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self { clock: None }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockTicked(time) => {
                self.clock = Some(time.format("%H:%M:%S").to_string());
                ctx.redraw();
            }
        }
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ self.clock.clone().unwrap_or_default() }</h1>
            </div>
        }
    }
}
