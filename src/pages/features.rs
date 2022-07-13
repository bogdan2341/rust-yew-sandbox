use crate::{
    console_log,
    helpers::{fetch::fetch, fetch::Task},
    routes::FeaturesRoutes,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct FeaturesItemProps {
    pub trimed_body: String,
    pub title: String,
    pub id: u32,
}

#[function_component(FeaturesItem)]
fn features_item(
    FeaturesItemProps {
        trimed_body,
        title,
        id,
    }: &FeaturesItemProps,
) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = {
        let id = id.clone();
        Callback::from(move |_| navigator.push(&FeaturesRoutes::Feature { id }))
    };

    html! {
        <div class="features-container__feature-item card p-2 m-3" onclick={onclick}>
            <div class="card-body">
                <h5 class="card-title">{title.clone()}</h5>
                <p class="card-text">
                    {trimed_body.clone()}
                </p>
            </div>
        </div>
    }
}

#[function_component(Features)]
pub fn features() -> Html {
    let features = use_state(Vec::<Task>::new);

    {
        let features = features.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let url = "https://jsonplaceholder.typicode.com/posts".to_string();
                    let res = fetch::<Vec<Task>>(url).await;
                    match res {
                        Ok(res) => features.set(res),
                        Err(res) => console_log!("{:?}", res),
                    }
                });
                || {}
            },
            (),
        )
    }

    //let tasks_value = *tasks;

    html! {
        <div class="features-container">
            {
                (*features).iter().map(|feature| html! {
                    <FeaturesItem
                        title={feature.title.clone()}
                        trimed_body={feature.get_trimed_body(70)}
                        id={feature.id.clone()}
                    />

                }).collect::<Html>()
            }
        </div>
    }
}
