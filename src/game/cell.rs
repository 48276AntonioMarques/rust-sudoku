enum CellValue {
    EMPTY,
    VALUE(u8)
}

struct Cell {
    value: u16
}


impl Cell {
    fn get_value(&self) -> CellValue {
        if self::value && 0x01 {
            return CellValue::EMPTY
        }
        else {
            // Mask the value (0b0000_0011_1111_1110)
            let value: u16 = self::value & 0x03FE;
        }
    }

    fn is_empty(&self) -> bool {
        return self::value && 0x01;
    }

    fn is_readonly(&self) -> bool {
        return self::value && 0x0400;
    }
}
