pub enum Response {
    Question,
    Yell,
    YellQuestion,
    Silence,
    Whatever,
}

impl Response {
    pub fn input_type(input: &str) -> Self {
        if input.trim_end().as_bytes().last() == Some(&b'?') {
            if input
                .trim_end()
                .strip_suffix('?')
                .unwrap()
                .chars()
                .filter(|x| x.is_ascii_alphanumeric())
                .all(|x| x.is_ascii_uppercase())
            {
                return Response::YellQuestion;
            }
            Response::Question
        } else if input.is_empty() || input.chars().all(|x| x.is_whitespace()) {
            Response::Silence
        } else if input
            .chars()
            .all(|x| !x.is_alphabetic() || x.is_uppercase())
        {
            Response::Yell
        } else {
            Response::Whatever
        }
    }

    pub fn reply(input: &str) -> &str {
        match Self::input_type(input) {
            Response::Question => "Sure.",
            Response::Yell => "Whoa, chill out!",
            Response::YellQuestion => "Calm down, I know what I'm doing!",
            Response::Silence => "Fine. Be that way!",
            Response::Whatever => "Whatever.",
        }
    }
}

pub fn is_yelling(message: &str) -> bool {
    message.to_uppercase() == message && message.chars().any(|x| x.is_alphabetic())
}
pub fn am_asking(message: &str) -> bool {
    message.ends_with('?')
}

pub fn reply(message: &str) -> &str {
    // Response::reply(message)
    match message.trim() {
        m if am_asking(m) && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if am_asking(m) => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        m if m.trim().len() == 0 => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn process_response_case(phrase: &str, expected_response: &str) {
        assert_eq!(reply(phrase), expected_response);
    }

    #[test]
    fn stating_something() {
        process_response_case("Tom-ay-to, tom-aaaah-to.", "Whatever.");
    }
    #[test]
    fn ending_with_whitespace() {
        process_response_case("Okay if like my    spacebar quite a bit?   ", "Sure.");
    }
    #[test]
    fn shouting_numbers() {
        process_response_case("1,2,3 GO!", "Whoa, chill out!");
    }
    #[test]
    fn other_whitespace() {
        process_response_case("\r\r    ", "Fine. Be that way!");
    }
    #[test]
    fn shouting_with_special_characters() {
        process_response_case(
            "ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!",
            "Whoa, chill out!",
        );
    }
    #[test]
    fn talking_forcefully() {
        process_response_case("Hi there!", "Whatever.");
    }
    #[test]
    fn prattling_on() {
        process_response_case("Wait! Hang on. Are you going to be OK?", "Sure.");
    }
    #[test]
    fn forceful_question() {
        process_response_case("WHAT'S GOING ON?", "Calm down, I know what I'm doing!");
    }
    #[test]
    fn shouting_with_no_exclamation_mark() {
        process_response_case("I HATE THE DENTIST", "Whoa, chill out!");
    }
}
