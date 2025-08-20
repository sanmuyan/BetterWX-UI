use super::errors::UPatchError;
use crate::errors::Result;
use crate::patch::types::Bytes;

pub fn calculate_jump_offset(
    current_addr: u64,
    target_addr: u64,
    offset_adjust: i64,
) -> Result<i32> {
    let next_instr_addr = current_addr as i64 + offset_adjust;
    let raw_offset = target_addr as i64 - next_instr_addr;
    
    let raw_offset = raw_offset as i32;
    if raw_offset < i32::MIN || raw_offset > i32::MAX {
        return Err(UPatchError::OffsetOutRangeError.into());
    }
    Ok(raw_offset)
}

pub fn calculate_jump_offset_bytes(
    current_addr: u64,
    target_addr: u64,
    offset_adjust: i64,
) -> Result<Bytes> {
    let raw_offset = calculate_jump_offset(current_addr, target_addr, offset_adjust)?;
    let bytes = if (i8::MIN as i32) <= raw_offset && raw_offset <= (i8::MAX as i32) {
        vec![raw_offset as u8]
    }else {
        raw_offset.to_le_bytes().to_vec()
    };
    let bytes = Bytes::new(bytes);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_address_jump() {
        let result =
            calculate_jump_offset_bytes(0x00007FF9B148C3B0, 0x7FF9B23F8935, 12).unwrap();
        println!("{:?}", result);
        println!("{}", result); 
    }
}
