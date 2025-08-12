use std::fmt;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Cell::Alive => "0",
            Cell::Dead => "X",
        };

        write!(f, "{}", symbol)
    }
}

pub struct Matrix {
    flat_matrix: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            flat_matrix: vec![Cell::Dead; width * height],
            width,
            height,
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        let idx = self.transform_2d_coordinate_into_flat_idx(x, y);
        self.flat_matrix[idx] = cell;
    }

    pub fn get_cell(&mut self, x: usize, y: usize) -> Cell {
        let idx = self.transform_2d_coordinate_into_flat_idx(x, y);
        self.flat_matrix[idx]
    }

    pub fn calculate_next_generation(&self) -> Self {
        let mut next_generation = Matrix::new(self.width, self.height);

        let mut x: usize;
        let mut y: usize;
        let mut alive_neighbors: u8;
        let mut current_cell_and_alive_neighbors: (&Cell, u8);
        let mut next_cell: Cell;

        let mut starting_column: usize;
        let mut ending_column: usize;

        let mut starting_row: usize;
        let mut ending_row: usize;

        for (cell_idx, cell) in self.flat_matrix.iter().enumerate() {
            alive_neighbors = 0;
            x = cell_idx % self.width;
            y = (cell_idx - x) / self.width;

            starting_column = if x > 0 { x - 1 } else { 0 };
            ending_column = if x < self.width - 1 {
                x + 1
            } else {
                self.width - 1
            };

            starting_row = if y > 0 { y - 1 } else { 0 };
            ending_row = if y < self.height - 1 {
                y + 1
            } else {
                self.height - 1
            };

            for row in starting_row..=ending_row {
                for column in starting_column..=ending_column {
                    if let Cell::Alive =
                        self.flat_matrix[self.transform_2d_coordinate_into_flat_idx(column, row)]
                    {
                        alive_neighbors += 1;
                    }
                }
            }

            if let Cell::Alive = cell {
                alive_neighbors -= 1;
            }

            current_cell_and_alive_neighbors = (cell, alive_neighbors);

            next_cell = match current_cell_and_alive_neighbors {
                (Cell::Alive, 2) => Cell::Alive,
                (Cell::Alive, 3) => Cell::Alive,
                (Cell::Dead, 3) => Cell::Alive,
                (_, _) => Cell::Dead,
            };

            next_generation.set_cell(x, y, next_cell);
        }

        next_generation
    }

    fn transform_2d_coordinate_into_flat_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (cell_idx, cell) in self.flat_matrix.iter().enumerate() {
            if cell_idx % self.width == 0 {
                writeln!(f);
            }
            write!(f, "{cell} ");
        }
        writeln!(f);
        Ok(())
    }
}
