use crate::*;

// fn on_value_change(args: OnFloatChangeArgs) -> FloatResult {
//     let OnFloatChangeArgs { val, prev } = args;

//     if val.is_empty() {
//         return FloatResult::ok_zero(prev.format);
//     }

//     match Float::from_str(prev.format, &val) {
//         Ok(f) => Ok(f),
//         Err(e) => Err(e.to_string()),
//     }
// }

#[function_component(App)]
pub fn app() -> Html {
    let float = use_state(|| Float::from_bits(
        Format::ieee_binary64(),
        BitPattern::from_value(0u32)
    ).unwrap());

    html! {
        <MainContainer>
            <p class="text-5xl font-bold my-8">{ "Custom Float Calculator" }</p>
            <p class="text-3xl font-bold my-4 break-all">{ float.to_string() }</p>
            <BitPatterns float={float.clone()} />
            <FormatInput float={float.clone()} />
            <UserInput float={float.clone()} />
        </MainContainer>
    }
}