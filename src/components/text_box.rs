use crate::*;

#[derive(Properties, PartialEq)]
pub struct TextBoxProps {
    #[prop_or("".to_owned())]
    pub class: String,

    #[prop_or(None)]
    pub err: Option<String>,

    #[prop_or(None)]
    pub val: Option<String>,

    #[prop_or(Callback::from(move |_|{}))]
    pub on_input: Callback<InputEvent>,
    
    #[prop_or(Callback::from(move |_|{}))]
    pub on_focus_out: Callback<FocusEvent>,
}

#[function_component(TextBox)]
pub fn text_box(props: &TextBoxProps) -> Html {
    let TextBoxProps {
        class,
        err,
        val,
        on_input,
        on_focus_out,
    } = props;

    html! {
        <input
            class={classes!(
                "w-full", "px-4", "py-2", "m-0.5", "text-center", "text-gray-700",
                "bg-gray-100", "rounded-lg", "border-2", "font-medium", class,
                if err.is_some() { "focus:border-rose-500 focus:ring-rose-500 border-rose-500" } else { "" },
            )}
            type="text"
            value={val.clone()}
            oninput={on_input}
            onfocusout={on_focus_out}
        />
    }
}