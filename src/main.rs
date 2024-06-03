use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
pub mod utils;

use structs::ExpressionCellValueType;

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
                row_cells.push(structs::new_empty_cell(col, row));
            } else if !cell_content.is_empty() && cell_content.starts_with('=') {
                row_cells.push(structs::new_expression_cell(col, row, cell_content));
            } else {
                match cell_content.parse::<i32>() {
                    Ok(n) => row_cells.push(structs::new_numeric_cell(col, row, cell_content, n)),
                    Err(_) => row_cells.push(structs::new_text_cell(col, row, cell_content)),
                }
            }
            last_col = col;
        }

        while last_col < size_x - 1 {
            row_cells.push(structs::new_empty_cell(last_col, row));
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

fn is_string_cell_static(cell_str: &str) -> Option<ExpressionCellValueType> {
    if cell_str.starts_with('"') && cell_str.ends_with('"') && cell_str.len() >= 2 {
        #[cfg(test)]
        println!("{cell_str} will be considered as a static str, not a cell reference");
        let content = cell_str[1..cell_str.len() - 1].to_string();
        return Some(ExpressionCellValueType::Str(content));
    }

    if let Ok(n) = cell_str.parse::<i32>() {
        println!("{cell_str} will be considered as a static number, not a cell reference");
        return Some(ExpressionCellValueType::Numeric(n));
    }

    None
}

fn eval_string_cell(
    left: &str,
    table: &mut Table,
    col_index: &HashMap<String, usize>,
) -> ExpressionCellValueType {
    #[cfg(test)]
    println!("Evaluating {left}");

    if let Some(result) = is_string_cell_static(left) {
        return result;
    }

    let cell = table.get_cell_by_str_ref(left, col_index);

    match &cell.specs {
        structs::SpecificCell::BaseCells(structs::BaseCells::NumericCell(v)) => {
            ExpressionCellValueType::Numeric(v.value)
        }
        structs::SpecificCell::BaseCells(structs::BaseCells::TextCell()) => {
            ExpressionCellValueType::Str(cell.generics.string_content.clone())
        }
        structs::SpecificCell::ExpressionCell(v) => {
            if let Some(x) = v.value.clone() {
                x
            } else if v.evaluated == structs::EvalutedType::ToEvaluate {
                evaluate(cell.generics.pos_x, cell.generics.pos_y, table, col_index)
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
}

fn eval_string_expr(
    expr: &str,
    table: &mut Table,
    col_index: &HashMap<String, usize>,
) -> ExpressionCellValueType {
    match expr.split_once('+') {
        Some((left, right)) => {
            #[cfg(test)]
            println!("Splitting result : left {}, right {}", left, right);
            let mut total = eval_string_cell(left, table, col_index);
            total += eval_string_expr(right, table, col_index);
            total
        }
        None => eval_string_cell(expr, table, col_index),
    }
}

fn evaluate(
    cell_x: usize,
    cell_y: usize,
    table: &mut Table,
    col_index: &HashMap<String, usize>,
) -> ExpressionCellValueType {
    let cell = table.at_mut(cell_x, cell_y);
    let c = cast!(&mut cell.specs, structs::SpecificCell::ExpressionCell);
    if let Some(r) = c.value.clone() {
        #[cfg(test)]
        println!("{cell} is already evaluated");
        return r;
    }
    c.evaluated = structs::EvalutedType::InProgress;

    let cell_content = &cell.generics.string_content[1..].to_string();

    let total = eval_string_expr(cell_content, table, col_index);

    let cell = table.at_mut(cell_x, cell_y);
    let base_cell = cast!(&mut cell.specs, structs::SpecificCell::ExpressionCell);

    base_cell.evaluated = structs::EvalutedType::Ok;
    base_cell.value = Some(total.clone());

    #[cfg(test)]
    println!("Set result for cell {}", cell);
    return total;
}

fn main() -> std::io::Result<()> {
    let content = read_file("input.csv").unwrap();
    let (table, col_index) = get_table_from_content(&content).unwrap();
    let mut evaluated_table = table.clone();
    for x in 0..table.size_x {
        for y in 0..table.size_y {
            let cell = table.at(x, y);
            if let structs::SpecificCell::ExpressionCell(_) = &cell.specs {
                evaluate(x, y, &mut evaluated_table, &col_index);
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
