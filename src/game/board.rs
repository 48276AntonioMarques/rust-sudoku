struct Board {
    content: [u16; 81]
}

struct Row {
    content: [u16; 9]
}

struct Column {
    content: [u16; 9]
}

struct Grid {
    content: [u16; 9]
}

impl Board {
    fn get_row(&self, index: u8) -> Result<Row> {
        if index > 8 {
            return Result::Err("Invalid row.")
        }
        let begin = index * 9;
        let end = (index + 1) * 9;
        return Result::Ok(&self.content.clone()[begin..end]);
    }

    fn get_column(&self, index: u8) -> Result<Column> {
        if index > 8 {
            return Result::Err("Invalid Column.")
        }
        let column: [u16; 9] = [0; 9];
        for r in 0..9 {
            column[r] = &self.content[r * 9 + index].clone();
        }
        return Result::Ok(column);
    }

    fn get_grid(&self, index: u8) -> Result<Grid> {
        if index > 8 {
            return Result::Err("Invalid grid.")
        }
        let grid: [u16; 9] = [0; 9];
        let offset = index % 3;
        let offset_end = offset + 3;
        for r in 0..3 {
            grid[r..r + 2] = &self.content[offset..offset_end].clone()
        }
        return Result::Ok(grid);
    }
}