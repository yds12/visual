use visual::{vis, Visual, get_non_displayable_string, set_non_displayable_string};

#[test]
fn it_works() {
    assert_display_eq(vis!(6), "6");
    assert_display_eq(vis!("6"), "6");
    assert_display_eq(vis!("hello"), "hello");
    assert_display_eq(vis!(String::from("hello")), "hello");
    assert_display_eq(vis!(vec![1, 2, 3]), "[1, 2, 3]");

    assert_display_eq(vis!(&6), "6");
    assert_display_eq(vis!(&"6"), "6");
    assert_display_eq(vis!(&"hello"), "hello");

    let st = String::from("hello");
    assert_display_eq(vis!(&st), "hello");

    let vec = vec![1, 2, 3];
    assert_display_eq(vis!(&vec), "[1, 2, 3]");

    struct MyStruct;
    assert_display_eq(vis!(MyStruct), get_non_displayable_string());
}

#[test]
fn custom_default_works() {
    set_non_displayable_string("nothing").expect("Failed to initialize custom default string");
    assert_eq!(get_non_displayable_string(), "nothing");

    struct MyStruct;
    assert_display_eq(vis!(MyStruct), get_non_displayable_string());
}

fn assert_display_eq<T>(t: Visual<T>, other: &str) {
    assert_eq!(t.get_display(), other);
}
