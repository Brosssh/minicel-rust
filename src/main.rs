use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
pub mod utils;

use crate::utils::StringExt;

pub mod structs;
use crate::structs::Table;
use crate::structs::TableExt;

fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_table_size(content: &str) -> (usize, usize) {
    let mut x = 1;

    for c in content.split('\n') {
        x = max(x, c.split('|').count());
    }

    return (x, content.split('\n').count());
}

fn get_table_from_content(content: &str) -> std::io::Result<(Table, HashMap<String, usize>)> {
    if content.is_empty() {
        panic!("ERROR: file is empty");
    }
    let (size_x, size_y) = get_table_size(content);
    let valid_header = "A".to_string().."Z".to_string();

    let mut column_index = HashMap::new();
    let mut table = Vec::new();

    for (row, el) in content.split('\n').enumerate() {
        let mut row_cells = Vec::new();
        let mut last_col = 0;
        for (col, e) in el.split('|').enumerate() {
            let mut cell_content = e.to_string().trim_whitespaces();

            if row == 0 {
                cell_content = cell_content.to_uppercase();
                if cell_content.is_empty() {
                    panic!("ERROR: the file header should not contain empty values.");
                }
                if !valid_header.contains(&cell_content) {
                    panic!("ERROR: {cell_content} is not a valid value for an header.");
                } else {
                    column_index.insert(cell_content.clone(), col);
                }
            }

            if cell_content.is_empty() {
                row_cells.push(structs::new_empty_cell(row, col));
            } else if !cell_content.is_empty() && cell_content.starts_with('=') {
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
            last_col += 1;
        }
        table.push(row_cells);
    }

    let table = Table {
        cells: table,
        size_x,
        size_y,
    };

    Ok((table, column_index))
}

fn eval_cell(left: &str, table: &mut Table, col_index: &HashMap<String, usize>) -> i32 {
    #[cfg(test)]
    println!("Evaluating {left}");
    let (letter, n) = left.split_at(1);
    match col_index.get(&letter.to_uppercase()) {
        Some(&r) => {
            let cell_y = n.parse::<usize>().unwrap();
            let cell = table.at(r, cell_y);
            let value;

            match &cell.specs {
                structs::SpecificCell::BaseCells(structs::BaseCells::NumericCell(v)) => {
                    value = v.value;
                }
                structs::SpecificCell::ExpressionCell(v) => {
                    if v.value.is_some() {
                        value = v.value.unwrap();
                    } else if v.evaluated == structs::EvalutedType::ToEvaluate {
                        let expr_string = &cell.generics.string_content[1..].to_string();
                        value = eval_expr(r, cell_y, expr_string, table, col_index, true);
                    } else if v.evaluated == structs::EvalutedType::InProgress {
                        panic!("ERROR: infinite loop detected at cell {}", cell);
                    } else {
                        //is evaluated but none value
                        panic!("ERROR: could not evaluate cell {}", cell);
                    }
                }
                _ => {
                    panic!("ERROR: could not evaluate cell {}", cell);
                }
            }
            value
        }
        None => {
            panic!(
                "ERROR: letter {} not found in header",
                &letter.to_uppercase()
            );
        }
    }
}

fn eval_expr(
    cell_x: usize,
    cell_y: usize,
    e: &String,
    table: &mut Table,
    col_index: &HashMap<String, usize>,
    set_result: bool,
) -> i32 {
    #[cfg(test)]
    println!("Evaluation expr {}", e);

    let cell = table.at_mut(cell_x, cell_y);
    let c = cast!(&mut cell.specs, structs::SpecificCell::ExpressionCell);
    if let Some(r) = c.value {
        #[cfg(test)]
        println!("{cell} is already evaluated");
        return r;
    }

    c.evaluated = structs::EvalutedType::InProgress;

    let mut total = 0;
    match e.split_once('+') {
        Some((left, right)) => {
            #[cfg(test)]
            println!("Splitting result : left {}, right {}", left, right);
            total += eval_cell(left, table, col_index);
            total += eval_expr(cell_x, cell_y, &right.to_string(), table, col_index, false);
        }
        None => total += eval_cell(&e.to_string(), table, col_index),
    }

    if set_result {
        let cell = table.at_mut(cell_x, cell_y);
        let base_cell = cast!(&mut cell.specs, structs::SpecificCell::ExpressionCell);

        base_cell.evaluated = structs::EvalutedType::Ok;
        base_cell.value = Some(total);

        #[cfg(test)]
        println!("Set result for cell {}", cell);
    }

    total
}

fn main() -> std::io::Result<()> {
    let content = read_file("input.csv").unwrap();
    let (table, col_index) = get_table_from_content(&content).unwrap();
    let mut evaluated_table = table.clone();
    for x in 0..table.size_x {
        for y in 0..table.size_y {
            let cell = table.at(x, y);
            if let structs::SpecificCell::ExpressionCell(_) = &cell.specs {
                let expr_string = &cell.generics.string_content[1..].to_string();
                eval_expr(x, y, expr_string, &mut evaluated_table, &col_index, true);
            }
        }
    }

    println!("{}", evaluated_table);

    Ok(())
}

#[test]
fn test_main() -> std::io::Result<()> {
    main()
}
