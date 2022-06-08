use crate::*;

fn on_value_change(s: String) -> FloatResult {
    if s.is_empty() {
        return FloatResult::ok_zero(Format::ieee_binary32());
    }

    match Float::from_str(Format::ieee_binary32(), &s) {
        Ok(f) => {
            Ok(f)
        },
        Err(e) => {
            Err(FloatErr {
                format: Format::ieee_binary32(),
                msg: e.to_string(),
            })
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let float = use_state(|| FloatResult::ok_zero(Format::ieee_binary32()));

    html! {
        <MainContainer>
            <p>{ format!("{:?}", float.format()) }</p>
            <BitPatterns float={float.clone()} />
            <Input header="value" float={float.clone()} on_change={FnPtr(on_value_change)} />
            <ErrMsg float={float.clone()} />
        </MainContainer>
    }
}