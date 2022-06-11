use crate::*;

#[derive(Properties, PartialEq)]
pub struct ErrMsgProps {
    pub err: Option<String>,
}

#[function_component(ErrMsg)]
pub fn err_msg(props: &ErrMsgProps) -> Html {
    html! {
        <div class="m-2 text-rose-500 font-bold italic">
            { if let Some(msg) = &props.err {
                "Error: ".to_owned() + &msg
            } else {
                "".to_owned()
            }}
        </div>
    }
}