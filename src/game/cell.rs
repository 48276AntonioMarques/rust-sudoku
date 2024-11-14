use crate::exceptions::Exception;

pub enum CellValue {
    EMPTY,
    VALUE(u8)  // This is represented using two's complement
}

pub struct Cell {
    value: u16 // This is represented using position encoding
}

fn pos_to_number(pos: u16) -> u8 {
    let mut m_pos = pos & 0x03FE;
    let mut number = 0;
    loop {
        m_pos >>= 1;
        if m_pos <= 0 { break; }
        number += 1;
    }
    return number;
}

fn number_to_pos(number: u8, readonly: Option<bool>) -> u16 {
    let mut pos = 1;
    for _ in 0..number {
        pos <<= 1;
    }
    match readonly {
        Some(true) => { pos += 0x0400; }
        _ => {}
    }
    return pos
}

impl Cell {
    pub fn new(value: u16) -> Result<Cell, Exception> {
        // Check invalid bits
        let read_bit: u16 = 0x0400; // Read bit 10th position
        let inv_read: u16 = !read_bit;
        let non_read: u16 = value & inv_read;
        let last_val: u16 = 0x0200; // Number 9 bit 9th position
        if non_read > last_val {
            return Result::Err(Exception::InvalidCell)
        }

        // Check none selected
        if non_read == 0 {
            return Result::Err(Exception::NumberlessCell)
        }

        // Check multiple values


        return Result::Ok(Cell { value })
    }

    pub fn from_cell_value(value: CellValue, readonly: bool) -> Result<Cell, Exception> {
        match value {
            CellValue::EMPTY => return Cell::new(0x0400),
            CellValue::VALUE(number) => {
                let pos = number_to_pos(number, Option::Some(readonly));
                return Cell::new(pos)
            }
        }
    }

    pub fn get_code(&self) -> u16 {
        return self.value;
    }

    pub fn get_value(&self) -> CellValue {
        if self.value == 0x01 {
            return CellValue::EMPTY
        }
        else {
            // Mask the value (0b0000_0011_1111_1110)
            let number = pos_to_number(self.value.clone());
            return CellValue::VALUE(number)
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.value == 0x01;
    }

    pub fn is_readonly(&self) -> bool {
        // Check if is read only (0b0000_0100_0000_0000)
        return self.value == 0x0400;
    }
}
