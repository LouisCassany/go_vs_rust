fn main() {
    let u_ident: Matrix = load_csv_into_matrix("../model/u_ident.csv");
    let a: Matrix = load_csv_into_matrix("../model/A.csv");
    let b: Matrix = load_csv_into_matrix("../model/B.csv");
    let c: Matrix = load_csv_into_matrix("../model/C.csv");
    let d: Matrix = load_csv_into_matrix("../model/D.csv");
    let x0: Matrix = load_csv_into_matrix("../model/x0.csv");

    let system = System::new(a, b, c, d, x0);
    let (x, y) = system.simulate(u_ident);
    save_matrix_to_csv(&x.transpose(), "../results/rust/x_results.csv");
    save_matrix_to_csv(&y.transpose(), "../results/rust/y_results.csv");
}

struct System {
    a: Matrix,
    b: Matrix,
    c: Matrix,
    d: Matrix,
    x0: Matrix,
}

impl System {
    fn new(a: Matrix, b: Matrix, c: Matrix, d: Matrix, x0: Matrix) -> System {
        System { a, b, c, d, x0 }
    }

    fn simulate(&self, u: Matrix) -> (Matrix, Matrix) {
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

fn save_matrix_to_csv(matrix: &Matrix, file_path: &str) {
    let mut wtr = csv::Writer::from_path(file_path).unwrap();
    for i in 0..matrix.rows {
        let row = matrix.get_row(i);
        wtr.write_record(row.data[0].iter().map(|x| x.to_string()))
            .unwrap();
    }
    wtr.flush().unwrap();
}

fn load_csv_into_matrix(file_path: &str) -> Matrix {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)
        .unwrap();

    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let row: Vec<f64> = record.iter().map(|x| x.parse::<f64>().unwrap()).collect();
        data.push(row);
    }

    let rows = data.len();
    let cols = data[0].len();
    let mut matrix = Matrix::new(rows, cols);
    matrix.data = data;
    matrix
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

    fn copy(&self) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);
        result.data = self.data.clone();
        result
    }

    fn get_col(&self, col: usize) -> Matrix {
        let mut result = Matrix::new(self.rows, 1);
        for i in 0..self.rows {
            result.data[i][0] = self.data[i][col];
        }
        result
    }

    fn get_row(&self, row: usize) -> Matrix {
        Matrix {
            rows: 1,
            cols: self.cols,
            data: vec![self.data[row].clone()],
        }
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
        let mut result = Matrix::new(self.cols, self.rows);
        let _ = &self.data.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, value)| {
                result.data[j][i] = *value;
            })
        });
        result
    }

    fn append_col(&mut self, other: &Matrix) {
        if self.rows != other.rows {
            self.show_matrix_dim_error(other)
        }
        for i in 0..other.rows {
            self.data[i].push(other.data[i][0]);
        }
        self.cols += 1;
    }

    fn show_matrix_dim_error(&self, b: &Matrix) {
        panic!(
            "Error in matrix operation, A is {}x{} and B is {}x{}",
            self.rows, self.cols, b.rows, b.cols
        );
    }

    fn pretty_print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.data[i][j]);
            }
            println!();
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}
