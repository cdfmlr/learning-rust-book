// Messenger 是个单词的，没写错，送信人
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = (self.value as f64) / (self.max as f64);

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: over quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Warning: you've used up over 90% of your quota")
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: you've used up over 75% of your quota")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { 
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));

            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from("1"));
            two_borrow.push(String::from("2"));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        
        // mock_messenger.sent_messages.borrow_mut().push(String::from("anything")); // 在外面调也可以的
        // mock_messenger.sent_messages.borrow().push(String::from("immut")); // CE: cannot borrow as mutable

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
