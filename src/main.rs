use auto_variants::Variants;

#[derive(Variants, PartialEq, Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}
#[test]
fn unnested() {
    let correct_list = [
        Directions::Up,
        Directions::Down,
        Directions::Left,
        Directions::Right,
    ];

    assert_eq!(
        correct_list,
        Directions::variants(),
        "`correct_list` did not match with `variants()` "
    );
    assert_eq!(
        correct_list,
        Directions::VARIANTS,
        "`correct_list` did not match with `VARIANTS`"
    );
}

#[test]
fn nested() {
    todo!()
}

fn main() {}
