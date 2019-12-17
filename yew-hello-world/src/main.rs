extern crate yew;
use yew::prelude::*;

struct App {
    value: i64,
}

enum Msg {
    Plus,
    Minus,
    Reset,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App { 
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Plus => self.value += 1,
            Msg::Minus => self.value -= 1,
            Msg::Reset => self.value = 0
        }

        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button 
                    onclick=|_| Msg::Plus,
                    class=("button")>
                        { "+1" }
                </button>

                <button 
                    onclick=|_| Msg::Reset,
                    class=("button")>
                        { "Reset" }
                </button>

                <button 
                    onclick=|_| Msg::Minus,
                    class=("button")>
                        { "-1" }
                </button>
                
                <h1>{ "The final result is: " } { self.value }</h1>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}