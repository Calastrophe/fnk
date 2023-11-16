use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(TestInterface)]
pub fn test(props: &Props) -> Html {
    html! {}
}
