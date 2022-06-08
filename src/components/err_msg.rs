use crate::*;

#[derive(Properties, PartialEq)]
pub struct ErrMsgProps {
    pub float: UseStateHandle<FloatResult>,
}

#[function_component(ErrMsg)]
pub fn err_msg(props: &ErrMsgProps) -> Html {
    html! {
        <div class="my-4 text-rose-500 font-bold italic">
            { if let FloatResult::Err(FloatErr { msg, .. }) = &*props.float {
                "Error: ".to_owned() + &msg
            } else {
                "".to_owned()
            }}
        </div>
    }
}