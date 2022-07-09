use yew::prelude::*;

use crate::helpers::*;

use crate::components::Header;

use crate::pages::{
    Home, 
    Pricing, 
    About, 
    Faq, 
    Features,
};

use crate::structs::Page;

const PAGE_LS_KEY: &str = "active-page";

#[function_component(App)]
pub fn app () -> Html {
    let current_page_from_ls = LocalStorage::get(PAGE_LS_KEY);

    let current_page = use_state(|| {
        match current_page_from_ls {
            Some(page) => Page::from_str(&page[..]),
            None => Page::Home
        }
    });

    let on_click = {
        let current_page = current_page.clone();
        Callback::from(move |page: Page| {
            if *current_page != page {
                LocalStorage::set("active-page", page.to_key());
                current_page.set(page);
            }
        })
    };

    //console_log!("APP :: {:?}", *active_page);

    html! {
        <>
            <Header 
                on_menu_click={on_click.clone()}
                active_page={*current_page}
            />
            {
                match *current_page {
                    Page::Home => html! {<Home/>},
                    Page::Features => html! {<Features/>},
                    Page::About => html! {<About/>},
                    Page::FAQs => html! {<Faq/>},
                    Page::Pricing => html! {<Pricing/>},
                }
            }
        </>
    }
}