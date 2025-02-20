pub fn strip_backslashes(body: &[u8]) -> String {
    let str = String::from_utf8(Vec::<u8>::from(body)).unwrap();
    let _ = str.replace("//", "");
    str
}
