
use super::super::record_store::{
    ActivityRecord, Archetype
};

use super::config::{
    Machine,
    Activity as RawActivity,
    Rule as RawRule
};
pub enum Filter {
    TitleContainsAll(Vec<String>),
    TitleContainsAny(Vec<String>),
    TitleIs(String),
    TitleStartsWith(String),
    TitleEndsWith(String)
}

pub enum RuleInternal {
    ForName(String, Vec<Filter>),
    ForClass(String, Vec<Filter>),
    ForTitle(String, Vec<Filter>)
}

impl RuleInternal {
    pub fn apply(&self, r_name: &String, r_class: &String, r_title: &String) -> bool {
        match self {
            RuleInternal::ForClass(class, filters) => 
                if class == r_class {
                    Self::check_title(r_title, filters)
                } else {
                    false
                }
            RuleInternal::ForName(name, filters) => 
                if name == r_name {
                    Self::check_title(r_title, filters)
                } else {
                    false
                },
            RuleInternal::ForTitle(title, filters) => 
                if title == r_title {
                    Self::check_title(r_title, filters)
                } else {
                    false
                }
        }
    }

    fn check_title(title: &String, filters: &Vec<Filter>) -> bool {
        let mut result: bool = false;

        for filter in filters {
            result = false;

            match filter {
                Filter::TitleContainsAll(substrings) => {
                    result = true;
                    for substring in substrings {
                        if !title.contains(substring) {
                            continue;
                        }
                    }
                },
                Filter::TitleContainsAny(subsctrings) =>
                    for substring in subsctrings {
                        if title.contains(substring) {
                            return true
                        }
                    },
                Filter::TitleIs(counterpart) => {
                    result = title == counterpart
                },
                Filter::TitleStartsWith(substring) => {
                    let mut title_chars = title.chars();

                    let beginning_length = substring.len();
                    let title_length = substring.len();

                    if beginning_length > title_length {
                        continue;
                    }

                    for char in substring.chars() {
                        if char == title_chars.next().unwrap() {
                            result = true
                        } else {
                            continue;
                        }
                    }
                },
                Filter::TitleEndsWith(substring) => {
                    let ending_length = substring.len();
                    let title_length = substring.len();
                    let mut title_chars = title.chars();
                    let mut ending_chars = substring.chars();

                    if title_length < ending_length {
                        continue;
                    }

                    for i in 1..ending_length {
                        if ending_chars.nth(ending_length - i) == title_chars.nth(title_length - 1) {
                            result = true
                        } else {
                            continue;
                        }
                    }
                }
            }
            if result {
                return result
            }
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_test() {
        let rule = RuleInternal::ForClass(String::from("test-class"), vec![
            Filter::TitleContainsAll(vec![String::from("foo"), String::from("bar")]),
            Filter::TitleContainsAny(vec![String::from("baz")]),
            Filter::TitleEndsWith(String::from("end")),
            Filter::TitleStartsWith(String::from("start"))
        ]);

        let res1 = rule.apply(&String::from("r_name"), &String::from("no"), &String::from("no"));
        let res2 = rule.apply(&String::from("r_name"), &String::from("test-class"), &String::from("foo bar"));
        let res3 = rule.apply(&String::from("r_name"), &String::from("test-class"), &String::from("lolipop baz lightyear"));

        assert_eq!(res1, false);
        assert_eq!(res2, true);
        assert_eq!(res3, true);
    }
}