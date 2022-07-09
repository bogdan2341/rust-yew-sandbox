#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum Page {
    Home,
    Features,
    Pricing,
    FAQs,
    About,
}

impl Page {
    pub fn to_key (&self) -> &'static str {
        match &self {
            Page::Home => "home",
            Page::Features => "features",
            Page::About => "about",
            Page::FAQs => "faqs",
            Page::Pricing => "pricing",
        }
    }
    

    pub fn from_str(input: &str) -> Page {
        let input_lower = input.to_lowercase();

        match input_lower.as_str() {
            "home" => Page::Home,
            "features" => Page::Features,
            "pricing" => Page::Pricing,
            "faqs" => Page::FAQs,
            "about" => Page::About,
            _ => Page::Home,
        }
    }

}

