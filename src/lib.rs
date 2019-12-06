mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
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

impl Universe {
    fn get_index(&self,row:u32,column:u32) -> usize {
        (row*self.width + column) as usize
    }

    fn live_neighbor_count(&self,row:u32,column:u32) ->u8 {
         let mut count = 0;
         for delta_row in [self.height-1,0,1].iter().cloned() {
             for delta_col in [self.width-1,0,1].iter().cloned() {

                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row+delta_row) % self.height;
                 let neighbor_col = (column+delta_col) % self.width;

                 let neighbor_index = self.get_index(neighbor_row,neighbor_col);
                 count += self.cells[neighbor_index] as u8;


             }
         }
        count

    }


    pub fn set_cells(&mut self,cells:&[(u32,u32)]) {

        for (row,col) in cells.iter().cloned() {

            let idx = self.get_index(row,col);
            self.cells[idx] = Cell::Alive;

        }

    }





}

extern crate  js_sys;
#[wasm_bindgen]
impl Universe {

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 1..self.height {
            for col in 1..self.width {
                let idx = self.get_index(row,col);
                let cell = self.cells[idx];

                let live_neighbors = self.live_neighbor_count(row,col);


                let next_cell = match (cell,live_neighbors) {
                    //any live cell with less than 2 neighbors dies due to underpopulation
                    (Cell::Alive,x) if x < 2 => Cell::Dead,
                    (Cell::Alive,2) | (Cell::Alive,3) => Cell::Alive,
                    //any live cell with more than 3 neighbors dies due to overpopulation
                    (Cell::Alive,x) if x > 3 => Cell::Dead,

                    //any dead cell with exactly 3 neighbors lives again due to reproduction
                    (Cell::Dead,3) => Cell::Alive,
                    //otherwise, they remain in the same state
                    (otherwise,_) => otherwise,


                };
                next[idx] = next_cell;


            }

        }

        self.cells = next;


    }



    pub fn new() -> Universe {
        let width = 32;
        let height = 32;



        let cells = (0..width*height)
            .map( |_i| {
             if js_sys::Math::random() < 0.5 {
                 Cell::Alive
             } else {
                 Cell::Dead
             }

            } ).collect();

       let mut unv =  Universe {
            width,
            height,
            cells,
        };

        //unv.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
        unv

    }



    pub fn render(&self) -> String {
        self.to_string()
    }



}

use std::fmt;

impl fmt::Display for Universe {

    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result {

        for line in self.cells.as_slice().chunks(self.width as usize) {
             for &cell in line {
                   let symbol = if cell == Cell::Dead { '◻'  } else { '◼' };
                 write!(f,"{}",symbol)?;
             }
             write!(f,"\n")?

        }
       Ok(())
    }

}


