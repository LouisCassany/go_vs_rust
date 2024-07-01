use sysid::{Matrix, System};

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
