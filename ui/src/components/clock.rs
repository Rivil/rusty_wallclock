use chrono::{DateTime, Local};
use futures::stream::{Stream, StreamExt};
use std::time::Duration;
use yew::platform::time::interval;
use yew::{html, AttrValue, Component, Context, Html};

pub enum Msg {
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
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_stream(stream_time().map(Msg::ClockTicked));
        Self { clock: None }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockTicked(time) => {
                self.clock = Some(AttrValue::from(time.to_rfc2822()));
            }
        }
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let display = self.clock.as_deref().unwrap_or("Loading...");
        html! {
            <div>
                <h1>{ display }</h1>
            </div>
        }
    }
}
