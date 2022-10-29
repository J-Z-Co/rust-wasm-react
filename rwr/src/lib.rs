mod utils;
use std::fmt;

impl fmt::Display for Universe{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {'◻'} else {'◼'};
                write!(f, "{}", symbol);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rwr!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
#[wasm_bindgen]//Public methods, exported to JS
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors) {
                    //rule 1 less than 2 neightbors lead to die
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    //rule 2 2 or 3 neibours let live the next gen
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    //rule 3 more than 3 neightbors lead to die
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    //rule 4 dead with 3 neightbors coms back to life
                    (Cell::Dead, 3) => Cell::Alive,
                    //all other cells remain in the same state
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    fn get_index(&self, row: u32, column: u32) -> usize{
        (row*self.width+column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height-1, 0 , 1].iter().cloned() {
            for delta_col in [self.width-1, 0 , 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neightbor_row = (row + delta_row) % self.height;
                let neightbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neightbor_row, neightbor_col);
                count += self.cells[idx] as u8;
            }
        };
        count
    }
    pub fn new() -> Universe{
        let width = 640;
        let height = 640;

        let cells = (0..width*height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            }).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String{
        self.to_string()
    }

}