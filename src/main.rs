use std::fs::File;
use std::io::Read;
use std::cmp::max;
use std::collections::HashMap;
pub mod utils;
use crate::utils::StringExt;

pub mod structs;
use crate::structs::Table;
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

fn eval_cell(left: &String,table:  &mut Table, col_index: &HashMap<String, usize>) -> i32{
    println!("Evaluating {left}");
    let (letter, n) = left.split_at(1);
    match col_index.get(letter){
        Some(&r) => {
            let cell_y = n.parse::<usize>().unwrap();
            let cell = table.at(r, cell_y);
            let mut value = 0;
            
            match &cell.specs{
                structs::SpecificCell::BaseCells(structs::BaseCells::NumericCell(v)) => 
                {
                    value = v.value;
                },
                structs::SpecificCell::ExpressionCell(v) => 
                {
                    if !v.value.is_none() {
                        value = v.value.unwrap();
                    }
                    else if !v.evaluated{
                        let expr_string = &cell.generics.string_content[1..].to_string();
                        value = eval_expr(r ,cell_y , expr_string , table, col_index, true);
                        println!("Setting value {} for cell",value);
                    }else{
                        //is evaluated but none value
                        panic!("ERROR: could not evaluate cell {}", cell);
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

fn eval_expr(cell_x: usize, cell_y: usize , e: &String, table: &mut Table, col_index: &HashMap<String, usize>, set_result: bool) -> i32{
    println!("Evaluatiing expr {}", e);

    let cell = table.at(cell_x, cell_y);
    let c = cast!(&cell.specs, structs::SpecificCell::ExpressionCell);
    if let Some(r) = c.value{
        println!("{cell} is already evaluated");
        return r;
    }

    let mut total = 0;
    println!("Evaluatiing expr {}", e);
    match e.split_once("+") { 
        Some((left, right)) => {
            println!("Splitting result : left {}, right {}",left, right);
            total += eval_cell(&left.to_string(), table, &col_index);
            total += eval_expr(cell_x, cell_y, &right.to_string(), table, &col_index, false);
    }, 
        None => {
            total += eval_cell(&e.to_string(), table, &col_index)
        }
    }
    
    if set_result == true {
        let cell = table.at_mut(cell_x, cell_y);
        let base_cell =cast!(&mut cell.specs, structs::SpecificCell::ExpressionCell);

        base_cell.evaluated=true;
        base_cell.value=Some(total);

        println!("Set result for cell {}", cell);
    }
    
    return total;
}


fn main() -> std::io::Result<()>{
    let content = read_file("input.csv").unwrap();
    let (table, col_index) = get_table_from_content(&content).unwrap();
    let mut evaluated_table = table.clone();
    for x in 0..table.size_x{
        for y in 0..table.size_y{
            let cell = table.at(x, y); 
            if let structs::SpecificCell::ExpressionCell(_) = &cell.specs{
                let expr_string = &cell.generics.string_content[1..].to_string();
                eval_expr(x, y, expr_string, &mut evaluated_table, &col_index, true);
            }
        }
    } 

    println!("{}", evaluated_table);

    Ok(())
}