// use yew::prelude::*;

// struct Model {
//     value: i64
// }

// #[function_component(App)]
// fn app() -> Html {
//     let state = use_state(|| Model { value: 0 });

//     let onclick = {
//         let state = state.clone();

//         Callback::from(move |_| {
//             state.set(Model {
//                 value: state.value + 1
//             })
//         })
//     };

//     html! {
//         <div>
//             <button {onclick}>{"+1"}</button>
//             <p>{state.value}</p>
//         </div>
//     }
// }

// fn main() {
//     yew::start_app::<App>();
// }

use yew::prelude::*;

struct Model {
    price: f32,
    tax_rate: f32,
    total: f32,
}

fn setup() -> App<Model> {
    App::new().init_state(|| Model {
        price: 0.0,
        tax_rate: 0.0,
        total: 0.0,
    })
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::PriceChanged(new_price) => model.price = new_price,
        Msg::TaxRateChanged(new_tax_rate) => model.tax_rate = new_tax_rate,
        Msg::CalculateTotal => model.total = (1.0 + model.tax_rate) * model.price,
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <label>
                {"Price:"}
                <input type="text", value=model.price, oninput=|e| Msg::PriceChanged(e.value.parse().unwrap_or(0.0)) />
            </label>
            <br />
            <label>
                {"Tax Rate (%):"}
                <input type="text", value=model.tax_rate, oninput=|e| Msg::TaxRateChanged(e.value.parse().unwrap_or(0.0)) />
            </label>
            <br />
            <button onclick=|_| Msg::CalculateTotal,>{ "Calculate Total" }</button>
            <br />
            <label>{ "Total: " }{ model.total }</label>
        </div>
    }
}



// #[function_component(App)]
// fn app() -> Html {
//     html!(
//         <div>
//             <h1>{"bitch"}</h1>
//         </div>
//     )
// }

fn main() {
    yew::start_app::<App>();
}




// pub struct Model {
//     link: ComponentLink<Self>,
//     current_category: String,
//     categories: Vec<String>,
//     items: Vec<Vec<String>>,
// }

// pub enum Msg {
//     ChangeCategory(String),
//     NoOp,
// }

// impl Component for Model {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//         let categories = vec!["Category 1".to_string(), "Category 2".to_string(), "Category 3".to_string()];
//         let items = vec![
//             vec!["Item 1.1".to_string(), "Item 1.2".to_string()],
//             vec!["Item 2.1".to_string(), "Item 2.2".to_string()],
//             vec!["Item 3.1".to_string(), "Item 3.2".to_string()],
//         ];

//         Self {
//             link,
//             current_category: categories[0].clone(),
//             categories,
//             items,
//         }
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::ChangeCategory(new_category) => {
//                 self.current_category = new_category;
//                 true
//             }
//             Msg::NoOp => false,
//         }
//     }

//     fn change(&mut self, _: Self::Properties) -> ShouldRender {
//         false
//     }

//     fn view(&self) -> Html {
//         html! {
//             <div>
//                 <div>
//                     {for self.categories.iter().map(|category| {
//                         html! {
//                             <button
//                                 class=if self.current_category == *category { "selected" } else { "" }
//                                 onclick=self.link.callback(move |_| Msg::ChangeCategory(category.clone()))>
//                                 {category}
//                             </button>
//                         }
//                     })}
//                 </div>
//                 <div>
//                     {for self.items.iter().filter_map(|category_items| {
//                         if self.current_category == self.categories[self.items.iter().position(|i| i == category_items)?] {
//                             Some(html! {
//                                 <div>
//                                     {for category_items.iter().map(|item| {
//                                         html! { <div>{item}</div> }
//                                     })}
//                                 </div>
//                             })
//                         } else {
//                             None
//                         }
//                     })}
//                 </div>
//             </div>
//         }
//     }
// }
