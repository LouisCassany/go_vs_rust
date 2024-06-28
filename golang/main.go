package main

import (
	"encoding/csv"
	"fmt"
	"os"
	"strconv"
)

type Data struct {
	Control   float64 `csv:"CTRL_man_command_Manual"`
	Lacet     float64 `csv:"KITE_lacet_wr_Manual"`
	Awa       float64 `csv:"N2K_wind_angle_Manual"`
	Elevation float64 `csv:"KITE_elevation_wr_Manual"`
	Azimuth   float64 `csv:"KITE_azimuth_wr_Manual"`
	Aws       float64 `csv:"N2K_wind_speed_Manual"`
}

func main() {
	A, _ := loadMatrixFromCSV("../model/A.csv")
	B, _ := loadMatrixFromCSV("../model/B.csv")
	C, _ := loadMatrixFromCSV("../model/C.csv")
	D, _ := loadMatrixFromCSV("../model/D.csv")
	x0, _ := loadMatrixFromCSV("../model/x0.csv")
	u_ident, _ := loadMatrixFromCSV("../model/u_ident.csv")

	sys := System{
		A:  A,
		B:  B,
		C:  C,
		D:  D,
		x0: x0,
	}

	x_results, y_results := sys.simulate(u_ident)
	x_results.saveToCSV("../results/golang/x_results.csv")
	y_results.saveToCSV("../results/golang/y_results.csv")
}

type System struct {
	A  *Matrix
	B  *Matrix
	C  *Matrix
	D  *Matrix
	x0 *Matrix
}

func (sys *System) simulate(u_ident *Matrix) (*Matrix, *Matrix) {
	// Create a new matrix with to store the state
	x := sys.x0
	y_results := &Matrix{
		rows: sys.C.rows,
		cols: 0,
		data: make([][]float64, sys.C.rows),
	}
	x_results := sys.x0

	// Compute the state and output at each time step
	for i := 0; i < u_ident.rows; i++ {
		u := u_ident.getRow(i).transpose()
		x = sys.A.multiply(x).add(sys.B.multiply(u))
		x_results.appendColumn(x)
		y_results.appendColumn(sys.C.multiply(x).add(sys.D.multiply(u)))
	}

	x_results = x_results.transpose()
	y_results = y_results.transpose()

	return x_results, y_results
}

type Matrix struct {
	rows int
	cols int
	data [][]float64
}

func loadMatrixFromCSV(filename string) (*Matrix, error) {
	// Open the CSV file
	file, err := os.Open(filename)
	if err != nil {
		return nil, fmt.Errorf("failed to open file: %v", err)
	}
	defer file.Close()

	// Create a new CSV reader
	reader := csv.NewReader(file)

	// Read all records from the CSV file
	records, err := reader.ReadAll()
	if err != nil {
		return nil, fmt.Errorf("failed to read csv data: %v", err)
	}

	// Determine the number of rows and columns
	rows := len(records)
	if rows == 0 {
		return nil, fmt.Errorf("csv file is empty")
	}
	cols := len(records[0])

	// Initialize the matrix data slice
	data := make([][]float64, rows)
	for i := range data {
		data[i] = make([]float64, cols)
	}

	// Fill the matrix data
	for i, record := range records {
		if len(record) != cols {
			return nil, fmt.Errorf("row %d has a different number of columns", i)
		}
		for j, value := range record {
			floatValue, err := strconv.ParseFloat(value, 64)
			if err != nil {
				return nil, fmt.Errorf("failed to parse float value at row %d, column %d: %v", i, j, err)
			}
			data[i][j] = floatValue
		}
	}

	// Create and return the Matrix
	matrix := &Matrix{
		rows: rows,
		cols: cols,
		data: data,
	}

	return matrix, nil
}

func (m *Matrix) getRow(i int) *Matrix {
	return &Matrix{
		rows: 1,
		cols: m.cols,
		data: [][]float64{m.data[i]},
	}
}

func (m *Matrix) getCol(i int) *Matrix {
	// create a new matrix with the same number of rows as the original matrix
	new_matrix := &Matrix{
		rows: m.rows,
		cols: 1,
		data: make([][]float64, m.rows),
	}

	// iterate over the rows of the original matrix
	for j := 0; j < m.rows; j++ {
		// append the jth column of the original matrix to the new matrix
		new_matrix.data[j] = append(new_matrix.data[j], m.data[j][i])
	}

	return new_matrix
}

// Implement the savetoCSV method that saves the matrix to a CSV file
func (m *Matrix) saveToCSV(filename string) error {
	// Open the CSV file
	file, err := os.Create(filename)
	if err != nil {
		return fmt.Errorf("failed to create file: %v", err)
	}
	defer file.Close()

	// Create a new CSV writer
	writer := csv.NewWriter(file)
	defer writer.Flush()

	// Write each row to the CSV file with comma as the delimiter and a newline at the end
	for _, row := range m.data {
		record := make([]string, m.cols)
		for j, value := range row {
			record[j] = fmt.Sprintf("%f", value)
		}
		if err := writer.Write(record); err != nil {
			return fmt.Errorf("failed to write record to csv: %v", err)
		}
	}
	return nil
}

// Implement the transpose method that returns the transpose of the matrix
func (m *Matrix) transpose() *Matrix {
	// Create a new matrix with the number of columns and rows swapped
	rows := len(m.data)
	cols := len(m.data[0])

	transposed := make([][]float64, cols)
	for i := range transposed {
		transposed[i] = make([]float64, rows)
	}

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			transposed[j][i] = m.data[i][j]
		}
	}

	return &Matrix{
		rows: cols,
		cols: rows,
		data: transposed,
	}

}

// Implement the append method that appends a new one column matrix to the right of the current matrix
func (m *Matrix) appendColumn(n *Matrix) {
	if m.rows != n.rows {
		panic("Matrix dimensions must match")
	}

	for i := 0; i < m.rows; i++ {
		m.data[i] = append(m.data[i], n.data[i][0])
	}
	m.cols++
}

func (m *Matrix) multiply(n *Matrix) *Matrix {
	if m.cols != n.rows {
		panic("Columns of the first matrix must be equal to rows of the second matrix")
	}

	result := Matrix{
		rows: m.rows,
		cols: n.cols,
		data: make([][]float64, m.rows),
	}

	for i := 0; i < m.rows; i++ {
		result.data[i] = make([]float64, n.cols)
		for j := 0; j < n.cols; j++ {
			var sum float64
			for k := 0; k < m.cols; k++ {
				sum += m.data[i][k] * n.data[k][j]
			}
			result.data[i][j] = sum
		}
	}

	return &result
}

func (m *Matrix) add(n *Matrix) *Matrix {
	if m.rows != n.rows || m.cols != n.cols {
		panic("Matrix dimensions must match")
	}

	result := Matrix{
		rows: m.rows,
		cols: m.cols,
		data: make([][]float64, m.rows),
	}

	for i := 0; i < m.rows; i++ {
		result.data[i] = make([]float64, m.cols)
		for j := 0; j < m.cols; j++ {
			result.data[i][j] = m.data[i][j] + n.data[i][j]
		}
	}

	return &result
}

func (m *Matrix) string() string {
	str := ""
	for i := 0; i < m.rows; i++ {
		str += "["
		for j := 0; j < m.cols; j++ {
			str += fmt.Sprintf("%f", m.data[i][j])
			if j < m.cols-1 {
				str += " "
			}
		}
		str += "]\n"
	}
	return str
}

func (m *Matrix) shape() string {
	return fmt.Sprintf("%dx%d", m.rows, m.cols)
}
