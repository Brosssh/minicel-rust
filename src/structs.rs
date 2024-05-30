#[derive(Clone)]
pub enum CellType {
    Numeric,
    Text,
    Expression,
    Empty,
}

#[derive(Clone)]
pub struct NumericCell {
    pub value: i32,
}

#[derive(Clone)]
pub enum BaseCells {
    TextCell(),
    NumericCell(NumericCell),
}

#[derive(Clone)]
pub struct ExpressionCell {
    pub value: Option<i32>,
    pub evaluated: bool,
}

#[derive(Clone)]
pub struct GenericFields {
    pub pos_x: usize,
    pub pos_y: usize,
    pub string_content: String,
    pub cell_type: CellType,
}

#[derive(Clone)]
pub enum SpecificCell {
    BaseCells(BaseCells),
    ExpressionCell(ExpressionCell),
    EmptyCell(),
}

#[derive(Clone)]
pub struct Cell {
    pub specs: SpecificCell,
    pub generics: GenericFields,
}

#[derive(Clone)]
pub struct Table {
    pub cells: Vec<Vec<Cell>>,
    pub size_x: usize,
    pub size_y: usize,
}

pub trait TableExt {
    fn at(&self, x: usize, y: usize) -> &Cell;
    fn at_mut(&mut self, x: usize, y: usize) -> &mut Cell;
}

impl TableExt for Table {
    fn at(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y][x]
    }
}

pub fn new_empty_cell(x: usize, y: usize) -> Cell {
    Cell {
        generics: {
            GenericFields {
                string_content: "".to_string(),
                pos_x: x,
                pos_y: y,
                cell_type: CellType::Empty,
            }
        },
        specs: SpecificCell::EmptyCell {},
    }
}

pub fn new_numeric_cell(x: usize, y: usize, string_content: String, n: i32) -> Cell {
    Cell {
        generics: {
            GenericFields {
                string_content,
                pos_x: x,
                pos_y: y,
                cell_type: CellType::Numeric,
            }
        },
        specs: SpecificCell::BaseCells(BaseCells::NumericCell(NumericCell { value: n })),
    }
}

pub fn new_text_cell(x: usize, y: usize, string_content: String) -> Cell {
    Cell {
        generics: {
            GenericFields {
                string_content,
                pos_x: x,

                pos_y: y,
                cell_type: CellType::Text,
            }
        },
        specs: SpecificCell::BaseCells(BaseCells::TextCell {}),
    }
}

pub fn new_expression_cell(x: usize, y: usize, string_content: String) -> Cell {
    Cell {
        generics: {
            GenericFields {
                string_content,
                pos_x: x,
                pos_y: y,
                cell_type: CellType::Expression,
            }
        },
        specs: SpecificCell::ExpressionCell(ExpressionCell {
            evaluated: false,
            value: None,
        }),
    }
}

impl ToString for CellType {
    fn to_string(&self) -> String {
        match self {
            CellType::Numeric => String::from("Num"),
            CellType::Text => String::from("Text"),
            CellType::Expression => String::from("Expr"),
            CellType::Empty => String::from("Empty"),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let SpecificCell::ExpressionCell(v) = &self.specs {
            if let Some(result) = v.value {
                write!(
                    f,
                    "{}({})={}",
                    self.generics.cell_type.to_string(),
                    self.generics.string_content,
                    result
                )
            } else {
                write!(
                    f,
                    "{}({})=?",
                    self.generics.cell_type.to_string(),
                    self.generics.string_content
                )
            }
        } else {
            write!(
                f,
                "{}({})",
                self.generics.cell_type.to_string(),
                self.generics.string_content
            )
        }
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.size_y {
            for j in 0..self.size_x {
                print!("{}\t\t\t", self.cells[i][j]);
            }
            println!("\n");
        }
        write!(
            f,
            "Dumped the content of a table {}x{}",
            self.size_x, self.size_y
        )
    }
}
