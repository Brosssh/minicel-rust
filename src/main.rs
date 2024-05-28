use std::fs::File;
use std::io::Read;
use std::cmp::max;

pub mod utils;
use crate::utils::StringExt;

pub mod structs;
use crate::structs::CellType;
use crate::structs::Cell;
use crate::structs::Table;
use crate::structs::SpecificCell;
use crate::structs::GenericFields;

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

            match cell_content.parse::<i32>() {
                Ok(n) => row_cells.push(Cell{
                    generics: { GenericFields {
                        string_content: cell_content,
                        pos_x: row,
                        pos_y: col,
                        cell_type: CellType::Numeric
                    }},
                    specs: SpecificCell::NumericCell(structs::NumericCell{
                        value: n
                    })
                }),
                Err(e) => row_cells.push(Cell{
                    generics: { GenericFields {
                        string_content: cell_content,
                        pos_x: row,
                        pos_y: col,
                        cell_type: CellType::Text
                    }},
                    specs: SpecificCell::TextCell{}   
                }),
              }
            
            last_col = col;
        }

        while last_col < size_x - 1 {
            row_cells.push(Cell{
                generics: { GenericFields {
                    string_content: String::new(),
                    pos_x: row,
                    pos_y: last_col,
                    cell_type: CellType::Empty
                }},
                specs: SpecificCell::EmptyCell{}
            });
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