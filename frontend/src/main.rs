use yew::prelude::*;

mod service;

pub enum Msg {
    GetData,
    ProcessData(Option<String>),
}

struct App {
    received_data: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            received_data: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetData => {
                log::debug!("Get data");
                ctx.link().send_future(async move {
                    let result = service::get_data().await;

                    Msg::ProcessData(result)
                })
            }
            Msg::ProcessData(data) => {
                log::debug!("Received data: {:?}", data);
                self.received_data = data;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_click = link.callback(|_e: MouseEvent| {
            log::debug!("Clicked");
            Msg::GetData
        });

        html! {
            <div>
            <h1>{"Ethereum transactions viewer frontend"}</h1>
            <div>
            <button onclick={on_click}>{"Get Data"}</button>
            </div>
            <div>
                { match self.received_data.clone() {
                    Some(data) => data,
                    _ => "No data received".to_string()
                }}
            </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
