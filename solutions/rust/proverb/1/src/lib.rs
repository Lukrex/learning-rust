pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() { return String::new(); }
    
    let mut res = String::new();
    
    for win in list.windows(2) {
        let now_word = win[0];
        let next_word = win[1];
        res += &format!("For want of a {now_word} the {next_word} was lost.\n");
    }
    res += &format!("And all for the want of a {}.", list[0]);
    res
}
