use crate::*;

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
            <BinRep float={float.clone()} />
            <HexRep float={float.clone()} />
            <Help />
        </MainContainer>
    }
}