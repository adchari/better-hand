use crate::types::PokerHand;
use prettytable::{Attr, Cell, Row, Table};
use std::collections::HashMap;

fn cell_color(i: u8, j: u8, prob: Option<f64>) -> u32 {
    if prob.is_none() {
        return 11;
    }

    if prob.unwrap() <= 0.75 {
        1
    } else {
        if i <= j {
            // off
            4
        } else {
            7
        }
    }
}

fn make_grid(probs: HashMap<PokerHand, f64>) -> [[(PokerHand, Option<f64>); 13]; 13] {
    let mut arr: [[(PokerHand, Option<f64>); 13]; 13] = [[(Default::default(), None); 13]; 13];
    for (hand, val) in probs {
        let (a, b) = hand.grid_pos();
        arr[a as usize][b as usize] = (hand, Some(val));
    }

    arr
}

fn grid_string(cell: (PokerHand, Option<f64>)) -> String {
    if cell.1.is_none() {
        return format!("Not\nPoss");
    }
    format!("{}:\n{:.2}", cell.0.to_string(), cell.1.unwrap())
}

pub fn pretty_print(probs: HashMap<PokerHand, f64>) -> Table {
    let grid = make_grid(probs);

    let mut table = Table::new();

    for i in 0..13 {
        let mut row_vec: Vec<Cell> = Vec::new();
        for j in 0..13 {
            let cell: (PokerHand, Option<f64>) = grid[i as usize][j as usize];
            row_vec.push(
                Cell::new(&grid_string(cell))
                    .with_style(Attr::ForegroundColor(cell_color(i, j, cell.1))),
            );
        }
        table.add_row(Row::new(row_vec));
    }

    table
}
