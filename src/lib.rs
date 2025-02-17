mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;
use js_sys::Math;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;
use web_sys::console;
extern crate web_sys;
extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet, // Use FixedBitSet to store cells efficiently
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
    
        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };
    
        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };
    
        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };
    
        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };
    
        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;
    
        let n = self.get_index(north, column);
        count += self.cells[n] as u8;
    
        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;
    
        let w = self.get_index(row, west);
        count += self.cells[w] as u8;
    
        let e = self.get_index(row, east);
        count += self.cells[e] as u8;
    
        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;
    
        let s = self.get_index(south, column);
        count += self.cells[s] as u8;
    
        let se = self.get_index(south, east);
        count += self.cells[se] as u8;
    
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
    
        let mut next = {
            let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };
    
        {
            let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells.contains(idx);
                    let live_neighbors = self.live_neighbor_count(row, col);
    
                    let next_cell = match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,  // Underpopulation
                        (true, 2) | (true, 3) => true,  // Survival
                        (true, x) if x > 3 => false,  // Overpopulation
                        (false, 3) => true,  // Reproduction
                        (otherwise, _) => otherwise,  // No change
                    };
    
                    next.set(idx, next_cell);
                }
            }
        }
    
        let _timer = Timer::new("free old cells");
        self.cells = next;
    }
    

    pub fn new() -> Universe {
        let width = 128;
        let height = 128;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, Math::random() < 0.5);
        }

        console::log_1(&"Universe created with random cells!".into());

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        let slice = self.cells.as_slice();
        let ptr = slice.as_ptr() as *const u32;
        ptr
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((width * self.height) as usize);
        console::log_1(&format!("Width set to: {}", width).into());
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * height) as usize);
        console::log_1(&format!("Height set to: {}", height).into());
    }
    
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        let current = self.cells.contains(idx);
        self.cells.set(idx, !current);
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let symbol = if self.cells[idx] { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Universe {
    pub fn get_cells(&self) -> Vec<bool> {
        (0..self.width * self.height)
            .map(|i| self.cells.contains(i as usize))
            .collect()
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for &(row, col) in cells {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
        console::log_1(&format!("{} cells set to alive.", cells.len()).into());
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}
