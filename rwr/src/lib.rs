mod utils;
use std::fmt;
extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;

// impl fmt::Display for Universe{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == Cell::Dead {'◻'} else {'◼'};
//                 write!(f, "{}", symbol);
//             }
//             write!(f, "\n");
//         }
//         Ok(())
//     }
// }

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
    cells: FixedBitSet,
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
                next.set(idx, match (cell, live_neighbors) {
                    //rule 1 less than 2 neightbors lead to die
                    (true, x) if x < 2 => false,
                    //rule 2 2 or 3 neibours let live the next gen
                    (true, 2) | (true, 3) => true,
                    //rule 3 more than 3 neightbors lead to die
                    (true, x) if x > 3 => false,
                    //rule 4 dead with 3 neightbors coms back to life
                    (false, 3) => true,
                    //all other cells remain in the same state
                    (otherwise, _) => otherwise,
                });
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
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        
        for i in 0..size {
            cells.set(i, if js_sys::Math::random() < 0.5 {false} else {true});
        }

        Universe {
            width,
            height,
            cells,
        }
    }
    // pub fn render(&self) -> String{
    //     self.to_string()
    // }

    //add more getter functions for width, height and pointer to cells array and expose to JS
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}