use std::fs::File;
use std::io::Read;
use std::cmp::max;

pub mod utils;
use crate::utils::StringExt;

pub mod structs;
use crate::structs::CellType;
use crate::structs::Cell;
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
        let mut result = Vec::new();
        let mut last_col = 0;
        for (col, e) in el.split("|").enumerate(){
            result.push(Cell{
                string_content: e.to_string().trim_whitespaces(),
                pos_x: row,
                pos_y: col,
                cell_type: CellType::Text
            });
            last_col = col;
        }

        while last_col < size_x - 1 {
            result.push(Cell{
                string_content: String::new(),
                pos_x: row,
                pos_y: last_col,
                cell_type: CellType::Empty
            });
            last_col+=1;
        }
        table.push(result);
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