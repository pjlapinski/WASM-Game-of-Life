use yew::prelude::*;

pub mod board;
pub mod cell;

const STYLE_FILE: &str = include_str!("style.css");

#[function_component(App)]
pub fn app() -> Html {
    let (width, height) = (69usize, 30usize);

    // changes with width and height
    let board_style = stylist::Style::new(format!(
        r#"
        .board {{
            grid-template-columns: repeat({}, var(--cell-size));
            grid-template-rows: repeat({}, var(--cell-size));
        }}
        "#,
        width, height
    ))
    .unwrap();

    html! {
        <>
            <stylist::yew::Global css={STYLE_FILE} />
            <main class={board_style}>
                <board::Board {height} {width} />
            </main>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
