pub fn reply(msg: &str) -> &str {
    let msg = msg.trim();

    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = msg.ends_with('?');
    let is_yelling = msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic());

    match (is_question, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}