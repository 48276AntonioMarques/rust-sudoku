use crate::exceptions::Exception;

use crate::game::cell;

pub struct Board {
    content: [u16; 81]
}

pub struct Row {
    index: u8,
    pub content: [u16; 9]
}

pub struct Column {
    index: u8,
    content: [u16; 9]
}

pub struct Grid {
    index: u8,
    content: [u16; 9]
}

impl Board {
    pub fn new(values: [u8; 81]) -> Result<Board, Exception> {
        // When creating the board all the numbers will be marked as
        let mut content = [0; 81];
        let mut i = 0;
        for number in values {
            // For each number create a cell value and then a cell from it
            // This converts the input number into the position encoding 
            let cell_value = cell::CellValue::VALUE(number);
            match cell::Cell::from_cell_value(cell_value, true) {
                Ok(c) => {
                    content[i] = cell::Cell::get_code(&c);
                }
                Err(_) => return Result::Err(Exception::InvalidBoard),
            }
            i += 1;
        }
        return Result::Ok(Board { content })
    }

    pub fn get_row(&self, index: u8) -> Result<Row, Exception> {
        if index > 8 {
            return Result::Err(Exception::RowOutOfBounds)
        }
        let begin = (index * 9) as usize;
        let end = ((index + 1) * 9) as usize;
        let mut content: [u16; 9] = [0; 9];
        content.copy_from_slice(&self.content[begin..end]);
        let row = Row { index, content };
        return Result::Ok(row);
    }

    pub fn get_column(&self, index: u8) -> Result<Column, Exception> {
        if index > 8 {
            return Result::Err(Exception::ColumnOutOfBounds)
        }
        let mut content: [u16; 9] = [0; 9];
        let idx = index as usize;
        for r in 0..9 {
            content[r] = self.content[r * 9 + idx].clone();
        }
        let column = Column { index, content };
        return Result::Ok(column);
    }

    pub fn get_grid(&self, index: u8) -> Result<Grid, Exception> {
        if index > 8 {
            return Result::Err(Exception::GridOutOfBounds)
        }
        let mut content: [u16; 9] = [0; 9];
        let offset = (index % 3) as usize;
        let offset_end = offset + 3;
        
        for r in 0..3 {
            content[r..r + 2].copy_from_slice(&self.content[offset..offset_end]);
        }

        let grid = Grid { index, content };
        return Result::Ok(grid);
    }
}