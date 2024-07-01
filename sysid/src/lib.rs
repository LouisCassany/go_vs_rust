pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn copy(&self) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        result.data = self.data.clone();
        result
    }

    pub fn get_col(&self, col: usize) -> Matrix {
        let mut result = Matrix::new(self.rows, 1);
        for i in 0..self.rows {
            result.data[i][0] = self.data[i][col];
        }
        result
    }

    pub fn get_row(&self, row: usize) -> Matrix {
        Matrix {
            rows: 1,
            cols: self.cols,
            data: vec![self.data[row].clone()],
        }
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        // Check if the matrices have the same dimensions
        if self.rows != other.rows || self.cols != other.cols {
            self.show_matrix_dim_error(other)
        }

        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        // Check if the matrices can be multiplied
        if self.cols != other.rows {
            self.show_matrix_dim_error(other)
        }
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        let _ = &self.data.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, value)| {
                result.data[j][i] = *value;
            })
        });
        result
    }

    pub fn append_col(&mut self, other: &Matrix) {
        if self.rows != other.rows {
            self.show_matrix_dim_error(other)
        }
        for i in 0..other.rows {
            self.data[i].push(other.data[i][0]);
        }
        self.cols += 1;
    }

    pub fn show_matrix_dim_error(&self, b: &Matrix) {
        panic!(
            "Error in matrix operation, A is {}x{} and B is {}x{}",
            self.rows, self.cols, b.rows, b.cols
        );
    }

    pub fn pretty_print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.data[i][j]);
            }
            println!();
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

pub struct System {
    pub a: Matrix,
    pub b: Matrix,
    pub c: Matrix,
    pub d: Matrix,
    pub x0: Matrix,
}

impl System {
    pub fn new(a: Matrix, b: Matrix, c: Matrix, d: Matrix, x0: Matrix) -> System {
        System { a, b, c, d, x0 }
    }

    pub fn simulate(&self, u: Matrix) -> (Matrix, Matrix) {
        let mut current_x: Matrix = self.x0.copy();
        let mut x: Matrix = self.x0.copy();
        let mut y: Matrix = Matrix::new(self.c.rows, 0);
        let (n, _) = u.size();
        for i in 0..n {
            let x_next = self
                .a
                .multiply(&current_x)
                .add(&self.b.multiply(&u.get_row(i).transpose()));
            let y_next = self
                .c
                .multiply(&current_x)
                .add(&self.d.multiply(&u.get_row(i).transpose()));
            current_x = x_next;
            y.append_col(&y_next);
            x.append_col(&current_x);
        }
        (x, y)
    }
}
