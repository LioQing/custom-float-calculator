use crate::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct UserInputProps {
    pub float: UseStateHandle<Float>,
}

#[function_component(UserInput)]
pub fn user_input(props: &UserInputProps) -> Html {
    let err = use_state(|| None);

    let handle_on_focus_out = {
        let float = props.float.clone();
        let err = err.clone();

        Callback::from(move |e: FocusEvent| {
            let s = e
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();

            if s.is_empty() {
                err.set(None);
                float.set(Float::from_bits((*float).clone().format, BitPattern::from_value(0u32)).unwrap());
                return;
            }

            if !Regex::new("[+-]?[0-9]*\\.?[0-9]*").unwrap().is_match(&s) {
                err.set(Some("invalid format".to_owned()));
                return;
            }

            let new_f = match Float::from_str(float.format.clone(), &s) {
                Ok(f) => {
                    err.set(None);
                    f
                },
                Err(e) => {
                    err.set(Some(e.to_string()));
                    Float::from_bits((*float).clone().format, BitPattern::from_value(0u32)).unwrap()
                }
            };

            float.set(new_f);
        })
    };

    html! {
        <>
            <span class="my-0.5 text-2xl">{ "Your Input" }</span>
            <br />
            <TextBox
                err={(*err).clone()}
                on_focus_out={handle_on_focus_out.clone()}
            />
            <br />
            <ErrMsg err={(*err).clone()} />
        </>
    }
}