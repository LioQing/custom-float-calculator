use crate::*;

#[derive(Properties, PartialEq)]
pub struct MainContainerProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MainContainer)]
pub fn main_container(props: &MainContainerProps) -> Html {
    html! {
        <div class="md:container md:mx-auto md:px-36 px-10 py-16 text-center">
            { props.children.clone() }
        </div>
    }
}
