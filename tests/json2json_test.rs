use std::fs;
use jsonwith::json2json;

fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}

#[test]
fn json2json_sample1() {
    let sample1 = read("./tests/assets/sample1.json");
    // todo fix last line existence. 
    assert_eq!(json2json(&sample1, 2), sample1);
}

#[test]
fn json2json_sample2() {
    let sample2 = read("./tests/assets/sample2.json");
    assert_eq!(json2json(&sample2, 2), sample2);
}

#[test]
fn json2json_empty_dict() {
    let emptydict = read("./tests/assets/emptydict.json");
    assert_eq!(json2json(&emptydict, 2), emptydict);
}

#[test]
fn json2json_empty_list() {
    let emptylist = read("./tests/assets/emptylist.json");
    assert_eq!(json2json(&emptylist, 2), emptylist);
}
