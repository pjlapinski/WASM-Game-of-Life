use crate::cell::Cell;
use yew::prelude::*;

#[derive(Properties, Clone, Copy, PartialEq)]
pub struct BoardProps {
    pub width: usize,
    pub height: usize,
}

fn do_step_on_board(board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_board = board.to_vec();
    let get_neighbors = |i: usize, j: usize| {
        let mut out_vec = Vec::<bool>::new();
        if i != 0 {
            out_vec.push(board[i - 1][j]);
        }
        if i != board.len() - 1 {
            out_vec.push(board[i + 1][j]);
        }
        if j != 0 {
            out_vec.push(board[i][j - 1]);
        }
        if j != board[i].len() - 1 {
            out_vec.push(board[i][j + 1]);
        }

        if i != 0 && j != 0 {
            out_vec.push(board[i - 1][j - 1]);
        }
        if i != 0 && j != board[i].len() - 1 {
            out_vec.push(board[i - 1][j + 1]);
        }
        if i != board.len() - 1 && j != 0 {
            out_vec.push(board[i + 1][j - 1]);
        }
        if i != board.len() - 1 && j != board[i].len() - 1 {
            out_vec.push(board[i + 1][j + 1]);
        }

        out_vec
    };

    for x in 0..board.len() {
        for y in 0..board[x].len() {
            let neighbors = get_neighbors(x, y);
            let alive_num = neighbors.iter().filter(|cell| **cell).count();

            // Any live cell with two or three live neighbours survives.
            if new_board[x][y] && (alive_num < 2 || alive_num > 3) {
                new_board[x][y] = false;
            }
            // Any dead cell with three live neighbours becomes a live cell.
            if !new_board[x][y] && alive_num == 3 {
                new_board[x][y] = true;
            }
            // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
        }
    }
    new_board
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let (height, width) = (props.height, props.width);
    let board = use_state(|| {
        let mut v = Vec::<Vec<bool>>::new();

        for _ in 0..height {
            let mut row = Vec::<bool>::new();
            for _ in 0..width {
                row.push(false);
            }
            v.push(row);
        }
        v
    });

    let change_cell = {
        let board = board.clone();
        Callback::from(move |(x, y)| {
            let mut b = (*board).to_vec();
            let row: &mut Vec<bool> = b.get_mut(x).unwrap();
            let cell: &mut bool = (*row).get_mut(y).unwrap();
            *cell = !*cell;
            board.set(b);
        })
    };

    let do_step = {
        let board = board.clone();
        Callback::from(move |_| board.set(do_step_on_board(&*board)))
    };

    let randomize = {
        let board = board.clone();
        Callback::from(move |_| {
            let mut brd = Vec::<Vec<bool>>::new();

            for _ in 0..height {
                let mut row = Vec::<bool>::new();
                for _ in 0..width {
                    row.push(rand::random::<bool>());
                }
                brd.push(row);
            }
            board.set(brd);
        })
    };

    html! {
        <div class="board-wrapper">
            <nav class="board-nav">
                <button class="nav-btn" onclick={do_step}>{"Do Step"}</button>
                <button class="nav-btn" onclick={randomize}>{"Randomize"}</button>
            </nav>
            <div class="board">
                {
                    (0..board.len()).map(|x| {
                        (0..board[x].len()).map(|y| {
                            html! {
                                <Cell {x} {y} alive={board[x][y]} click_cb={change_cell.clone()} />
                            }
                        }).collect::<Html>()
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
