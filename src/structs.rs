pub enum CellType {
    Numeric,
    Text,
    Expression,
    Empty
}

pub struct TextCell{

}

pub struct NumericCell{
    pub value: i32,
}

pub struct ExpressionCell{
    pub value: i32,
    pub evaluated: bool
}

pub struct EmptyCell{

}

pub struct GenericFields{
    pub pos_x: usize,
    pub pos_y: usize,
    pub string_content: String,
    pub cell_type: CellType
}

pub enum SpecificCell {
    TextCell(),
    NumericCell(NumericCell),
    ExpressionCell(ExpressionCell),
    EmptyCell()
  }

pub struct Cell{
    pub specs: SpecificCell,
    pub generics: GenericFields,
}

pub struct Table{
    pub cells: Vec<Vec<Cell>>,
    pub size_x: usize,
    pub size_y: usize,
}

pub trait TableExt {
    fn at(&self, x: usize, y: usize) -> &Cell;
}

impl TableExt for Table {

    fn at(&self, x: usize, y: usize) -> &Cell {
        return &self.cells[x][y];
    } 
}

pub fn new_empty_cell(x: usize, y: usize) -> Cell {
    return Cell{
        generics: { GenericFields {
            string_content: "".to_string(),
            pos_x: x,
            pos_y: y,
            cell_type: CellType::Empty
        }},
        specs: SpecificCell::EmptyCell{}   
    };
} 

pub fn new_numeric_cell(x: usize, y: usize, string_content: String, n: i32) -> Cell {
    return Cell{
        generics: { GenericFields {
            string_content: string_content,
            pos_x: x,
            pos_y: y,
            cell_type: CellType::Numeric
        }},
        specs: SpecificCell::NumericCell(NumericCell{
            value: n
        })
    };
} 

pub fn new_text_cell(x: usize, y: usize, string_content: String) -> Cell {
    return Cell{
        generics: { GenericFields {
            string_content: string_content,
            pos_x: x,
            pos_y: y,
            cell_type: CellType::Text
        }},
        specs: SpecificCell::TextCell{}   
    };
} 

impl ToString for CellType {
    fn to_string(&self) -> String {
      match self {
        CellType::Numeric => String::from("Numeric"),
        CellType::Text => String::from("Text"),  
        CellType::Expression => String::from("Expression"),
        CellType::Empty => String::from("Empty")
      }
    }
  }

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.generics.cell_type.to_string(), self.generics.string_content)
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.size_y{
            for j in 0..self.size_x{
                print!("{}\t\t\t", self.cells[i][j]);
            }
            println!("\n");
        }
        write!(f,"Dumped the content of a table {}x{}", self.size_x, self.size_y)
    }
}
