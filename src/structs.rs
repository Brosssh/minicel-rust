pub enum CellType {
    Numeric,
    Text,
    Expression,
    Empty
}

pub struct Cell{
    pub pos_x: usize,
    pub pos_y: usize,
    pub string_content: String,
    pub cell_type: CellType,
}

pub struct Table{
    pub cells: Vec<Vec<Cell>>,
    pub size_x: usize,
    pub size_y: usize,
}

pub trait TableExt {
    fn at(&self, x: usize, y: usize) -> &Cell;
    //fn dump_table(&self) -> None;
}

impl TableExt for Table {

    fn at(&self, x: usize, y: usize) -> &Cell {
        return &self.cells[x][y];
    }  
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
        write!(f, "{}({})", self.cell_type.to_string(), self.string_content)
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
