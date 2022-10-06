use common::models as c_models;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod service;

pub enum Msg {
    GetData,
    ProcessData(Option<c_models::AccountData>),
}

struct App {
    input_ref: NodeRef,
    received_data: Option<c_models::AccountData>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
            received_data: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetData => {
                log::debug!("Get data");
                let input = self.input_ref.cast::<HtmlInputElement>().unwrap();
                let address = if input.value().trim().is_empty() {
                    "".to_string()
                } else {
                    input.value().trim().to_string()
                };

                input.set_value("");
                let params = c_models::QueryParams {
                    address: address,
                    page: 1,
                    offset: 1000,
                    sort: "asc".to_string(),
                };

                ctx.link().send_future(async move {
                    let result = service::get_account_data(&params).await;

                    Msg::ProcessData(result)
                });
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
                    <input style="width:350px;" type="text" ref={self.input_ref.clone()} />
                    <button onclick={on_click}>{"Get Account Data"}</button>
                </div>
                <div>
                { match self.received_data.clone() {
                    Some(val) => {
                        show_account(&val)
                    },
                    _ => html! {
                        <div>
                            {"Please enter an address"}
                        </div>
                    }
                }}
                </div>
            </div>
        }
    }
}

fn show_account(account: &c_models::AccountData) -> Html {
    let normal_tx = account.normal_transactions.clone();
    let tx_html = normal_tx.into_iter().map(|tx| {
        html! {
            <tr key={tx.hash}>
                <td>{tx.from}</td>
                <td>{tx.value}</td>
                <td>{tx.gas_price}</td>
                <td>{tx.contract_address}</td>
                <td>{tx.function_name}</td>
            </tr>
        }
    });

    html! {
        <div>
            <div>
            { format!("Address: {}, Balance: {}, Normal tx: {}",
            account.address, account.balance, account.normal_transactions.len()
            ) }
            </div>
            <div>
                <table>
                    <thead>
                        <tr>
                        <th>{"From"}</th>
                        <th>{"Value"}</th>
                        <th>{"Gas Price"}</th>
                        <th>{"Contract"}</th>
                        <th>{"Function"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for tx_html}
                    </tbody>
                </table>
            </div>
        </div>

    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
