use std::fs::File;
use std::io::Read;
use std::cmp::max;
use std::collections::HashMap;
pub mod utils;
use crate::utils::StringExt;

pub mod structs;
use crate::structs::Table;
use crate::structs::Cell;
use crate::structs::TableExt;

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

    let table = Table{
        cells: table,
        size_x: size_x,
        size_y: size_y,
    };

    Ok((table, column_index))
}

fn eval_cell<'a>(left: &String,table:  &mut Table, col_index: &HashMap<String, usize>) -> i32{
    println!("Evaluating {left}");
    let (letter, n) = left.split_at(1);
    match col_index.get(letter){
        Some(&r) => {
            println!("Found letter {:?}, result {:?}", letter, r);
            let cell_x = n.parse::<usize>().unwrap();
            let cell = table.at(cell_x, r);
            let mut value = 0;
            
            match &cell.specs{
                structs::SpecificCell::BaseCells(structs::BaseCells::NumericCell(v)) => 
                {
                    println!("get_expr_value {}", v.value);
                    value = v.value;
                },
                structs::SpecificCell::ExpressionCell(v) => 
                {
                    if !v.value.is_none() {
                        println!("get_expr_value {}", v.value.unwrap());
                        value = v.value.unwrap();
                    }
                    else if !v.evaluated{
                        let expr_string = &cell.generics.string_content[1..].to_string();
                        value = eval_expr(cell_x ,r , expr_string , table, col_index);
                        println!("Setting value {} for cell",value);
                    }else{
                        //is evaluated but none value
                        //panic!("ERROR: could not evaluate cell {}", cell);
                    }
                },
                _ => ()
            }

            return value
        },
        None => {
            println!("Error: letter {} not found in hasmap", letter);
            return 0
        }
    }
}

fn eval_expr(cell_x: usize, cell_y: usize , e: &String, table: &mut Table, col_index: &HashMap<String, usize>) -> i32{
    let mut total = 0;
    println!("Evaluatiing expr {}", e);
    match e.split_once("+") { 
        Some((left, right)) => {
            println!("Splitting result : left {}, right {}",left, right);
            total += eval_cell(&left.to_string(), table, &col_index);
            total += eval_expr(cell_x, cell_y, &right.to_string(), table, &col_index);
    },
        None => {
            total += eval_cell(&e.to_string(), table, &col_index)
        }
    }

    let mut cell = table.at_mut(cell_x, cell_y);

    if let structs::SpecificCell::ExpressionCell(v) = &mut cell.specs{
        v.evaluated=true;
        v.value=Some(total);
    }
    println!("Set result for cell {}", cell);

    return total;
}


fn main() -> std::io::Result<()>{
    let content = read_file("input.csv").unwrap();
    let (mut table, col_index) = get_table_from_content(&content).unwrap();
    let mut table_clone = table.clone();
    for x in 0..table.size_x-1{
        for y in 0..table.size_y-1{
            let cell = table.at(x, y); 
            match &cell.specs {
                structs::SpecificCell::ExpressionCell(el) => 
                {
                    println!("Starting evaluation of cell {cell}, at pos {x},{y}");
                    let expr_string = &cell.generics.string_content[1..].to_string();
                    let r = eval_expr(x, y, expr_string, &mut table_clone, &col_index);
                    println!("Cell {}: {}", cell, r);
                },
                _ => ()            
            }
        }
    }

    println!("{}", table_clone);

    Ok(())
}