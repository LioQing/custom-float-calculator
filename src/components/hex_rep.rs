use crate::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct HexRepProps {
    pub float: UseStateHandle<Float>,
}

#[function_component(HexRep)]
pub fn hex_rep(props: &HexRepProps) -> Html {
    let val = use_state(|| "".to_owned());
    let err = use_state(|| None);

    {
        let float = props.float.clone();
        let dep = props.float.clone();
        let val = val.clone();

        use_effect_with_deps(
            move |_| {
                val.set(float.bits.to_hex_string());

                || {}
            },
            dep,
        );
    }

    let handle_on_focus_out = {
        let float = props.float.clone();
        let err = err.clone();

        Callback::from(move |e: FocusEvent| {
            let s = e
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .value()
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_ascii_lowercase())
                .collect::<String>();

            if s.is_empty() {
                err.set(None);
                return;
            }

            if s.len() != float.format.len() {
                err.set(Some("invalid length".to_owned()));
                return;
            }

            if !Regex::new("^[0-9a-f]*$").unwrap().is_match(&s) {
                err.set(Some("invalid characters".to_owned()));
                return;
            }

            match Float::from_bits(float.format.clone(), BitPattern::from_hex_str(&s)) {
                Ok(f) => {
                    err.set(None);
                    float.set(f);
                },
                Err(e) => {
                    err.set(Some(e.to_string()));
                },
            };
        })
    };

    html! {
        <>
            <span class="my-0.5 text-2xl">{ "Hexadecimal Representation" }</span>
            <br />
            <TextBox
                err={(*err).clone()}
                val={(*val).clone()}
                on_focus_out={handle_on_focus_out.clone()}
            />
            <br />
            <ErrMsg err={(*err).clone()} />
        </>
    }
}