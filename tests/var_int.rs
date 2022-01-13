use binary_utils::*;

#[test]
fn read_write_var_int() {
    let one = VarInt::<u32>(2147483647);
    let two = VarInt::<u32>(255);
    let buf_one = one.parse().unwrap();
    let buf_two = two.parse().unwrap();

    assert_eq!(buf_one, vec![255, 255, 255, 255, 7]);
    assert_eq!(buf_two, vec![255, 1]);

    let buf_long_one = VarInt::<u64>(9223372036854775807).parse().unwrap();
    assert_eq!(
        buf_long_one,
        vec![255, 255, 255, 255, 255, 255, 255, 255, 127]
    );

    assert_eq!(
        one.0,
        VarInt::<u32>::compose(&buf_one[..], &mut 0).unwrap().0
    );

    assert_eq!(
        two.0,
        VarInt::<u32>::compose(&buf_two[..], &mut 0).unwrap().0
    )
}
