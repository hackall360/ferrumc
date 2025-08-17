use ferrumc_macros::NBTSerialize;
use ferrumc_nbt::{FromNbt, NbtTape};

#[test]
fn deserialize_primitive_arrays() {
    #[derive(NBTSerialize)]
    struct Arrays {
        byte_array: Vec<i8>,
        int_array: Vec<i32>,
        long_array: Vec<i64>,
    }

    let arrays = Arrays {
        byte_array: vec![-1, 0, 1],
        int_array: vec![1, 2, 3],
        long_array: vec![4, 5, 6],
    };

    let buf = arrays.serialize_with_header();

    let mut tape = NbtTape::new(&buf);
    tape.parse();
    let root = tape.root.as_ref().map(|(_, b)| b).unwrap();

    let byte_el = root.get("byte_array").unwrap();
    let int_el = root.get("int_array").unwrap();
    let long_el = root.get("long_array").unwrap();

    let byte_out: Vec<i8> = FromNbt::from_nbt(&tape, byte_el).unwrap();
    let int_out: Vec<i32> = FromNbt::from_nbt(&tape, int_el).unwrap();
    let long_out: Vec<i64> = FromNbt::from_nbt(&tape, long_el).unwrap();

    assert_eq!(byte_out, arrays.byte_array);
    assert_eq!(int_out, arrays.int_array);
    assert_eq!(long_out, arrays.long_array);
}
