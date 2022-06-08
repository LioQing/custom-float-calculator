use crate::*;

#[derive(Properties, PartialEq)]
pub struct BitPatternsProps {
    pub float: UseStateHandle<FloatResult>,
}

#[function_component(BitPatterns)]
pub fn bit_patterns(props: &BitPatternsProps) -> Html {
    let float = props.float.float();

    let comps = float.to_comps();
    let button_class = "bg-violet-600 hover:bg-violet-800 active:bg-violet-500 text-white font-bold p-2 mx-0.5 rounded";
    let all_set_class = "bg-gray-200 hover:bg-gray-300 active:bg-gray-100 text-black p-1.5 m-0.5 rounded";
    let comps_class = "mx-1";

    // get the callback for button to toggle the n-th bit
    let get_on_toggle = |n: usize| {
        let f = props.float.clone();
        Callback::from(move |_| {
            f.set(FloatResult::Ok({
                let mut f = f.float();
                let b = !f.bits.get(n).unwrap();
                f.bits.set(n, b);
                f
            }));
        })
    };

    // get the callback for button to set all the bits to b
    let get_on_all_set = |b: usize| {
        let f = props.float.clone();
        Callback::from(move |_| {
            f.set(FloatResult::Ok({
                let mut f = f.float();
                f.bits.set_elements(b);
                f
            }));
            log::info!("{:?}", *f);
        })
    };

    let (exp_i, mant_i) = float.get_start_indices();

    html! {
        <div class="my-4">
            <p class="my-2">
                <span class={comps_class}>
                    <button class={button_class} onclick={get_on_toggle(0)}>
                        { comps.get_sign().unwrap_or_default() }
                    </button>
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