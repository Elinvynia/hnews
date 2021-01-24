macro_rules! test {
    ($name:ty, $file:expr) => {
        println!("Reading file...");
        let s = std::fs::read_to_string($file).unwrap();
        println!("Trying item...");
        let item: hnews::item::Item = miniserde::json::from_str(&s).unwrap();
        println!("Got item.");
        println!("{:#?}", item);
        let _: $name = item.try_into().unwrap();
    };
}

#[cfg(test)]
mod tests {
    use hnews::prelude::*;
    use std::convert::TryInto;

    #[test]
    fn test_ask() {
        test!(Ask, "tests/ask.json");
    }

    #[test]
    fn test_comment() {
        test!(Comment, "tests/comment.json");
    }

    #[test]
    fn test_job() {
        test!(Job, "tests/job.json");
    }

    #[test]
    fn test_poll() {
        test!(Poll, "tests/poll.json");
    }

    #[test]
    fn test_poll_option() {
        test!(PollOption, "tests/polloption.json");
    }

    #[test]
    fn test_story() {
        test!(Story, "tests/story.json");
    }
}
