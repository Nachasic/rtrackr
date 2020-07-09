
use super::super::record_store::{
    ActivityRecord, Archetype
};

use super::config::{
    ClassifierConfig,
    Activity as RawActivity,
    Rule as RawRule
};

#[derive(Debug)]
pub enum Filter {
    TitleContainsAll(Vec<String>),
    TitleContainsAny(Vec<String>),
    TitleIs(String),
    TitleStartsWith(String),
    TitleEndsWith(String)
}

#[derive(Debug)]
pub enum RuleInternal {
    ForName(Vec<String>, Vec<Filter>),
    ForClass(Vec<String>, Vec<Filter>),
    ForTitle(Vec<String>, Vec<Filter>)
}

impl Default for RuleInternal {
    fn default() -> Self {
        RuleInternal::ForName(vec![], vec![])
    }
}

impl From<RawRule> for Option<RuleInternal> {
    fn from(raw: RawRule) -> Option<RuleInternal> {
        let mut filters: Vec<Filter> = vec![];

        raw.title_contains_all.map(|vars| {
            filters.push(Filter::TitleContainsAll(vars))
        });
        raw.title_contains_any.map(|vars| {
            filters.push(Filter::TitleContainsAny(vars))
        });
        raw.title_is.map(|val| {
            filters.push(Filter::TitleIs(val))
        });
        raw.title_starts_with.map(|val| {
            filters.push(Filter::TitleStartsWith(val))
        });
        raw.title_ends_with.map(|val| {
            filters.push(Filter::TitleEndsWith(val))
        });

        match raw.for_class {
            Some(classes) => Some(RuleInternal::ForClass(classes, filters)),
            _ => match raw.for_name {
                Some(names) => Some(RuleInternal::ForName(names, filters)),
                _ => match raw.for_title {
                    Some(titles) => Some(RuleInternal::ForTitle(titles, filters)),
                    _ => None
                }
            }
        }
    }
}

impl RuleInternal {
    pub fn apply(&self, r_name: &String, r_class: &String, r_title: &String) -> bool {
        match self {
            RuleInternal::ForClass(classes, filters) => 
                if classes.contains(r_class) {
                    Self::check_title(r_title, filters)
                } else {
                    false
                }
            RuleInternal::ForName(names, filters) => 
                if names.contains(r_name) {
                    Self::check_title(r_title, filters)
                } else {
                    false
                },
            RuleInternal::ForTitle(titles, _) => 
                if titles.contains(r_title) {
                    // Self::check_title(r_title, filters)
                    true
                } else {
                    false
                }
        }
    }

    fn check_title(title: &String, filters: &Vec<Filter>) -> bool {
        let mut result: bool = filters.len() == 0;

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
        let rule = RuleInternal::ForClass(vec![String::from("test-class")], vec![
            Filter::TitleContainsAll(vec![String::from("foo"), String::from("bar")]),
            Filter::TitleContainsAny(vec![String::from("baz")]),
            Filter::TitleEndsWith(String::from("end")),
            Filter::TitleStartsWith(String::from("start"))
        ]);

        let res1 = rule.apply(&String::from("r_name"), &String::from("no"), &String::from("no"));
        let res2 = rule.apply(&String::from("r_name"), &String::from("test-class"), &String::from("foo bar"));
        let res3 = rule.apply(&String::from("r_name"), &String::from("test-class"), &String::from("lolipop baz lightyear"));
        let res4 = rule.apply(&String::from("r_name"), &String::from("test-class"), &String::from("start BRRRAP"));
        let res5 = rule.apply(&String::from("r_name"), &String::from("test-class"), &String::from("BRRRAP end"));

        assert_eq!(res1, false);
        assert_eq!(res2, true);
        assert_eq!(res3, true);
        assert_eq!(res4, true);
        assert_eq!(res5, true);
    }

    #[test]
    fn for_title_test() {
        let rule = RuleInternal::ForTitle(vec![String::from("Title in question")], vec![]);
        let res = rule.apply(&String::from("r_name"), &String::from("test-class"), &String::from("Title in question"));

        assert_eq!(res, true);
    }
}