use crate::*;

#[function_component(Help)]
pub fn help() -> Html {
    html! {
        <div class="text-left m-4">
            <p class="m-1 italic">{ "Help:" }</p>
            <ul class="m-1">
                <li>{ "- Click on blank space to confirm input." }</li>
                <li>{ "- Uses IEEE Interpretation, e.g. special pattern for 0, inf, -inf, NaN, sNaN. May not work on unsigned." }</li>
            </ul>
        </div>
    }
}