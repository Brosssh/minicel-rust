use std::fs::File;
use std::io::Read;
use std::cmp::max;

pub mod utils;
use crate::utils::StringExt;

pub mod structs;
use crate::structs::Table;

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

fn get_table_from_content(content :&str) -> std::io::Result<Table>{
    if content.len()==0 {
        panic!("ERROR: file is empty");
    }   
    let (size_x, size_y) = get_table_size(&content);

    let mut table = Vec::new();

    for (row, el) in content.split("\n").enumerate(){
        let mut row_cells = Vec::new();
        let mut last_col = 0;
        for (col, e) in el.split("|").enumerate(){

            let cell_content = e.to_string().trim_whitespaces();

            if cell_content.len() == 0{
                row_cells.push(structs::new_empty_cell(row, col));
            }else if cell_content.len() > 0 && cell_content.starts_with("="){
                row_cells.push(structs::new_text_cell(row, col, cell_content));
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
    Ok(Table{
            cells: table,
            size_x: size_x,
            size_y: size_y,
    })
}

fn main() -> std::io::Result<()>{
    let content = read_file("input.csv").unwrap();
    let result = get_table_from_content(&content);
    println!("{}", result.unwrap());
    Ok(())
}