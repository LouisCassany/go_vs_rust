fn main() {
    let mut a = Matrix::new(3, 2);
    a.set_value(0, 0, 1.0);
    a.set_value(0, 1, 2.0);
    a.set_value(1, 0, 3.0);
    a.set_value(1, 1, 4.0);
    a.set_value(2, 0, 5.0);
    a.set_value(2, 1, 6.0);

    println!("A = {:#?}", a);
}

#[derive(Debug)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    fn set_value(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    fn add(&self, other: &Matrix) -> Matrix {
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

    fn multiply(&self, other: &Matrix) -> Matrix {
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

    fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        let _ = &self.data.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, value)| {
                result.data[j][i] = *value;
            })
        });
        result
    }

    fn show_matrix_dim_error(&self, b: &Matrix) {
        panic!(
            "Error in matrix operation, A is {}x{} and B is {}x{}",
            self.rows, self.cols, b.rows, b.cols
        );
    }
}
