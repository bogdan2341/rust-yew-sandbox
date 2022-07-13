use crate::console_log;
use crate::helpers::fetch::{fetch, Task};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct FeatureProps {
    pub id: u32,
}

#[function_component(Feature)]
pub fn feature(props: &FeatureProps) -> Html {
    let feature = use_state(Task::empty);

    {
        let feature_id = props.id;
        let feature = feature.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let main_url = "https://jsonplaceholder.typicode.com/posts".to_string();
                    let url = format!("{}/{}", main_url, feature_id);
                    let res = fetch::<Task>(url).await;
                    match res {
                        Ok(res) => feature.set(res),
                        Err(res) => console_log!("{:?}", res),
                    }
                });
                || {}
            },
            (),
        );
    }

    html! {
        <div class="p-5 mx-auto" style="max-width: 700px">
            <h2>{(*feature).title.clone()}</h2>
            <div>{(*feature).body.clone()}</div>
        </div>
    }
}
