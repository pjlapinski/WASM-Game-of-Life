use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CellProps {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
    pub click_cb: Callback<(usize, usize)>,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let onclick = props.click_cb.clone();
    let (x, y) = (props.x, props.y);
    let onclick = move |_| onclick.emit((x, y));

    html! {
        <div {onclick} class={if props.alive {"cell-alive"} else {"cell-dead"}}></div>
    }
}
