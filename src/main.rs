use std::fs::File;
use std::io::Read;
use std::cmp::max;
use std::collections::HashMap;
pub mod utils;
use crate::utils::StringExt;

pub mod structs;
use crate::structs::Table;
use crate::structs::TableExt;
use crate::structs::CellExt;

fn read_file(path :&str) -> std::io::Result<String> {
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_table_size(content :&str) -> (usize, usize){
    let mut x = 1;

    for c in content.split("\n"){
        let m: Vec<_> = c.split("|").collect::<Vec<_>>();
        x = max(x, m.len());
    }

    return (x, content.split("\n").collect::<Vec<_>>().len());
}

fn get_table_from_content(content :&str) -> std::io::Result<(Table, HashMap<String, usize>)>{
    if content.len()==0 {
        panic!("ERROR: file is empty");
    }   
    let (size_x, size_y) = get_table_size(&content);

    let mut column_index = HashMap::new();
    let mut table = Vec::new();

    for (row, el) in content.split("\n").enumerate(){
        let mut row_cells = Vec::new();
        let mut last_col = 0;
        for (col, e) in el.split("|").enumerate(){

            if row == 0 {
                column_index.insert(e.to_string().trim_whitespaces(), col);
            }

            let cell_content = e.to_string().trim_whitespaces();

            if cell_content.len() == 0{
                row_cells.push(structs::new_empty_cell(row, col));
            }else if cell_content.len() > 0 && cell_content.starts_with("="){
                row_cells.push(structs::new_expression_cell(row, col, cell_content));
            } else {     
                match cell_content.parse::<i32>() {
                    Ok(n) => row_cells.push(structs::new_numeric_cell(row, col, cell_content, n)),
                    Err(_) => row_cells.push(structs::new_text_cell(row, col, cell_content)),
                }
                
            }
            last_col = col;
        }

        while last_col < size_x - 1 {
            row_cells.push(structs::new_empty_cell(row, last_col));
            last_col+=1;
        }
        table.push(row_cells);
    }
    Ok((Table{
            cells: table,
            size_x: size_x,
            size_y: size_y,
    }, column_index))
}

fn eval_cell(left: String, table: &Table, col_index: &HashMap<String, usize>) -> i32{
    println!("Evaluating {left}");
    let (letter, n) = left.split_at(1);
    match col_index.get(letter){
        Some(&r) => {
            println!("Found letter {:?}, result {:?}", letter, r);
            let cell = table.at(n.parse::<usize>().unwrap(), r);
            let value = cell.get_expr_value();
            println!("Found cell {}", cell);
            return value
        },
        None => {
            println!("Error: letter {} not found in hasmap", letter);
            return 0
        }
    }
}

fn eval_expr(e: String, table: &Table, col_index: &HashMap<String, usize>) -> i32{
    println!("Evaluatiing expr {}", e);
    let mut total = 0;
    match e.split_once("+") {
        Some((left, right)) => {
            println!("Splitting result : left {}, right {}",left, right);
            total += eval_cell(left.to_string(), &table, &col_index);
            total += eval_expr(right.to_string(), &table, &col_index);
    },
        None => {
            total += eval_cell(e.to_string(), &table, &col_index)
        }
    }
    println!("Result of expr {}: {}", e, total);
    return total;
}

fn evaluate_expressions(table: Table, col_index: HashMap<String, usize>){    
    for el in table.cells.iter(){
        for e in el.iter(){
            if matches!(e.generics.cell_type, structs::CellType::Expression){
                let r = eval_expr(e.generics.string_content[1..].to_string(), &table, &col_index);
            }
        }
    }
}

fn main() -> std::io::Result<()>{
    let content = read_file("input.csv").unwrap();
    let (table, col_index) = get_table_from_content(&content).unwrap();
    println!("{}", table);
    evaluate_expressions(table, col_index);
    Ok(())
}