use std::fmt;
#[derive(Debug, PartialEq)]
pub struct RsMatrix {
    pub data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl RsMatrix {
    /// Creates a new matrix from a 2D array of floats.
    pub fn new(data: Vec<Vec<f64>>) -> Result<Self, String> {
        let row_len = data[0].len();
        if !data.iter().all(|row| row.len() == row_len) {
            return Err(String::from("All rows must have the same length"));
        }
        let rows = data.len();
        let cols = row_len;
        Ok(Self { data, rows, cols })
    }
    pub fn print_matrix(vec: &Vec<Vec<f64>>) {
        for row in vec {
            let cols_str: Vec<_> = row.iter().map(ToString::to_string).collect();
            let line = cols_str.join("\t");
            println!("{}", line);
        }
    }
    pub fn stringfy_matrix(vec: &Vec<Vec<f64>>) -> Option<String>{
        let mut line= None ;
        for row in vec {
            let cols_str: Vec<_> = row.iter().map(ToString::to_string).collect();
            line = Some(cols_str.join("\t"));
        }
        line
    }
}
