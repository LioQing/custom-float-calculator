use crate::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub header: String,
    pub float: UseStateHandle<FloatResult>,
    pub on_change: FnPtr<String, FloatResult>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let val = use_state(|| props.float.float().to_string());

    let handle_on_change = {
        let val = val.clone();
        let on_change = props.on_change.0.clone();
        let float = props.float.clone();

        Callback::from(move |e: InputEvent| {
            let s = e
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();

            float.set(on_change(s.clone()));
            val.set(s.clone());
        })
    };

    {
        let val = val.clone();
        let float = props.float.clone();
        use_effect_with_deps(
            move |_| {
                match (*float).clone() {
                    Ok(f) => {
                        val.set(f.to_string());
                    },
                    Err(_) => {}
                }

                ||{}
            },
            props.float.clone(),
        );
    }

    html! {
        <form onsubmit={Callback::from(move |e: FocusEvent| e.prevent_default())}>
            <span class="my-0.5">{ props.header.clone() }</span>
            <br />
            <input
                class="w-full px-4 py-2 text-center text-gray-700 bg-gray-100 rounded-lg"
                type="text"
                value={(*val).clone()}
                oninput={handle_on_change}
            />
            <br />
        </form>
    }
}