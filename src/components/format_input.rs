use crate::*;
use wasm_bindgen::JsCast;

enum Component {
    Exponent,
    Mantissa,
    Excess,
}

#[derive(Properties, PartialEq)]
pub struct FormatInputProps {
    pub float: UseStateHandle<Float>,
}

#[function_component(FormatInput)]
pub fn format_input(props: &FormatInputProps) -> Html {
    let err = use_state(|| None);
    let format = (*props.float).format.clone();

    let handle_on_ieee32_click = {
        let float = props.float.clone();

        Callback::from(move |_| {
            float.set(Float::from_bits(Format::ieee_binary32(), BitPattern::from_value(0u32)).unwrap());
        })
    };

    let handle_on_ieee64_click = {
        let float = props.float.clone();

        Callback::from(move |_| {
            float.set(Float::from_bits(Format::ieee_binary64(), BitPattern::from_value(0u32)).unwrap());
        })
    };

    let handle_on_sign_change = {
        let float = props.float.clone();

        Callback::from(move |e: Event| {
            let signed = e
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .checked();
            
            float.set(Float::from_bits(
                Format {
                    signed,
                    ..float.format
                },
                BitPattern::from_value(0u32)
            ).unwrap());
        })
    };

    let get_handle_on_focus_out = |c: Component| {
        let float = props.float.clone();
        let err = err.clone();

        Callback::from(move |e: FocusEvent| {
            let s = e
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();

            match c {
                Component::Exponent => {
                    if !Regex::new("[0-9]+").unwrap().is_match(&s) {
                        err.set(Some("invalid format".to_owned()));
                        return;
                    }

                    match u32::from_str_radix(&s, 10) {
                        Ok(e) if e > 31 => {
                            err.set(Some("exponent cannot be greater than 31".to_owned()));
                        }
                        Ok(exp) => {
                            err.set(None);
                            float.set(Float::from_bits(
                                Format {
                                    exp: exp as u8,
                                    ..float.format
                                },
                                BitPattern::from_value(0u32)
                            ).unwrap());
                        },
                        Err(e) => {
                            err.set(Some(e.to_string()));
                        },
                    }
                },
                Component::Mantissa => {
                    if !Regex::new("[0-9]+").unwrap().is_match(&s) {
                        err.set(Some("invalid format".to_owned()));
                        return;
                    }

                    match usize::from_str_radix(&s, 10) {
                        Ok(m) if m > 1024 => {
                            err.set(Some("try not to make the mantissa to big, it will take too long to calculate".to_owned()));
                        }
                        Ok(mant) => {
                            err.set(None);
                            float.set(Float::from_bits(
                                Format {
                                    mant,
                                    ..float.format
                                },
                                BitPattern::from_value(0u32)
                            ).unwrap());
                        },
                        Err(e) => {
                            err.set(Some(e.to_string()));
                        },
                    }
                },
                Component::Excess => {
                    if !Regex::new("[0-9]+").unwrap().is_match(&s) {
                        err.set(Some("invalid format".to_owned()));
                        return;
                    }

                    match u32::from_str_radix(&s, 10) {
                        Ok(excess) => {
                            err.set(None);
                            float.set(Float::from_bits(
                                Format {
                                    excess,
                                    ..float.format
                                },
                                float.bits.clone()
                            ).unwrap());
                        },
                        Err(e) => {
                            err.set(Some(e.to_string()));
                        },
                    }
                },
            }
        })
    };

    let button_class = "\
        bg-gray-100 hover:bg-gray-300 active:bg-gray-50 \
        text-gray-700 font-medium p-2 m-0.5 \
        rounded-lg border-2 border-gray-500";

    html! {
        <div class="my-6">
            <span class="my-0.5 text-2xl">{ "format" }</span>
            <div class="my-2 flex gap-1 items-center justify-center">
                <div class="p-2">
                    <span>{ "signed" }</span><br />
                    <input
                        type="checkbox"
                        id="signed-checkbox"
                        class="m-1 rounded w-6 h-6 border-2"
                        checked={format.signed}
                        onchange={handle_on_sign_change}
                    />
                </div>
                <div class="w-1/6">
                    <span>{ "exponent" }</span><br />
                    <TextBox
                        val={format.exp.to_string()}
                        err={(*err).clone()}
                        on_focus_out={get_handle_on_focus_out(Component::Exponent)}
                    />
                </div>
                <div class="w-1/6">
                    <span>{ "mantissa" }</span><br />
                    <TextBox
                        val={format.mant.to_string()}
                        err={(*err).clone()}
                        on_focus_out={get_handle_on_focus_out(Component::Mantissa)}
                    />
                </div>
                <div class="w-1/6">
                    <span>{ "excess" }</span><br />
                    <TextBox
                        val={format.excess.to_string()}
                        err={(*err).clone()}
                        on_focus_out={get_handle_on_focus_out(Component::Excess)}
                    />
                </div>
            </div>
            <div class="my-2">
                <button class={button_class} onclick={handle_on_ieee32_click}>{ "IEEE Binary32" }</button>
                <button class={button_class} onclick={handle_on_ieee64_click}>{ "IEEE Binary64" }</button>
            </div>
            <ErrMsg err={(*err).clone()} />
        </div>
    }
}