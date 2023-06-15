use common::model::weather::current::CurrentWeather;
use common::model::weather::forecast::Forecast;
use yew::{html, Component, Context, Html};

pub enum Msg {
    WeatherUpdated(CurrentWeather),
    ForecastUpdated(Forecast),
}

pub struct WeatherComponent {
    weather: Option<CurrentWeather>,
    forecast: Option<Forecast>,
}

impl Component for WeatherComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {
            weather: None,
            forecast: None,
        };
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Weather" }</h1>
            </div>
        }
    }
}
