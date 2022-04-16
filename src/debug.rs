#[cfg(debug_assertions)]
pub fn debug(s: String) {
    println!("DEBUG: {}", s);
}

#[cfg(not(debug_assertions))]
pub fn debug(_s: String) {

}