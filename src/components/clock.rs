use std::ops::Deref;
use yew::{AttrValue, Component, Context, function_component, html, Html};
use yew::platform::time::interval;
use std::time::Duration;

use futures::{Stream, StreamExt};
use chrono::{DateTime, Local};

enum Msg {
    Tick,
}

struct AsyncComponent {
    clock: Option<AttrValue>,
}

pub fn stream_time() -> impl Stream<Item=DateTime<Local>> {
    interval(Duration::from_secs(1)).map(|_| Local::now())
}

impl Component for AsyncComponent {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let time = ctx.link().get_stream(stream_time());
        html! {
            <p>{ time }</p>
            }
        }

    fn Clock() -> Html {
        todo!()
    }
}
}