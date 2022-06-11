use crate::*;

#[derive(Properties, PartialEq)]
pub struct BitPatternsProps {
    pub float: UseStateHandle<Float>,
}

#[function_component(BitPatterns)]
pub fn bit_patterns(props: &BitPatternsProps) -> Html {
    let float = (*props.float).clone();

    let comps = float.to_comps();

    let button_class = "\
        bg-gray-600 hover:bg-gray-800 active:bg-gray-500 \
        text-white font-bold p-2 m-0.5 \
        rounded drop-shadow-lg";

    let all_set_class = "\
        bg-gray-100 hover:bg-gray-300 active:bg-gray-50 \
        text-gray-700 font-medium p-2 m-0.5 \
        rounded-lg border-2 border-gray-500";

    let comps_class = "mx-2";

    // get the callback for button to toggle the n-th bit
    let get_on_toggle = |n: usize| {
        let f = props.float.clone();
        Callback::from(move |_| {
            f.set({
                let mut f = (*f).clone();
                let b = !f.bits.get(n).unwrap();
                f.bits.set(n, b);
                f
            });
        })
    };

    // get the callback for button to set all the bits to b
    let get_on_all_set = |b: usize| {
        let f = props.float.clone();
        Callback::from(move |_| {
            f.set({
                let mut f = (*f).clone();
                f.bits.set_elements(b);
                f
            });
        })
    };

    let (exp_i, mant_i) = float.get_start_indices();

    html! {
        <div class="my-4">
            <p class="my-2">
                <span class={comps_class}>
                    {
                        if comps.sign.is_some() {
                            html! {
                                <button class={button_class} onclick={get_on_toggle(0)}>
                                    { comps.get_sign().unwrap_or_default() }
                                </button>
                            }
                        } else {
                            html! {}
                        }
                    }
                </span>
                <span class={comps_class}>
                    { comps.exp
                        .to_bin_string()
                        .chars()
                        .enumerate()
                        .map(|(i, c)| html! {
                            <button class={button_class} onclick={get_on_toggle(exp_i + i)}>
                                { c }
                            </button>
                        })
                        .collect::<Html>()
                    }
                </span>
                <span class={comps_class}>
                    { comps.mant
                        .to_bin_string()
                        .chars()
                        .enumerate()
                        .map(|(i, c)| html! {
                            <button class={button_class} onclick={get_on_toggle(mant_i + i)}>
                                { c }
                            </button>
                        })
                        .collect::<Html>()
                    }
                </span>
            </p>
            <div class="my-4">
                <button class={all_set_class} onclick={get_on_all_set(0)}>
                    { "All set to 0" }
                </button>
                <button class={all_set_class} onclick={get_on_all_set(!0usize)}>
                    { "All set to 1" }
                </button>
            </div>
        </div>
    }
}