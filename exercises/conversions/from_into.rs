// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results



impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // Step 1: Check if the length of the provided string is 0
        if s.is_empty() {
            return Person::default();
        }

        // Step 2: Split the string on commas
        let mut parts = s.split(',');

        // Step 3: Extract name from the split operation
        let name = parts.next().unwrap_or("").trim().to_string();

        // Step 4: If the name is empty, return the default Person
        if name.is_empty() {
            return Person::default();
        }

        // Step 5: Extract age and parse it into usize
        let age_result = parts.next().unwrap_or("").trim().parse::<usize>();

        // Step 6: Handle parsing errors and unexpected string formats
        let age = match age_result {
            Ok(age) => age,
            Err(_) => return Person::default(),
        };

        // Step 7: Return an instantiated Person object with results
        Person { name, age }
    }
}

fn main() {
    let p1 = Person::from("Mark,20");
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1); // Output: Person { name: "Mark", age: 20 }
    println!("{:?}", p2); // Output: Person { name: "Gerald", age: 70 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    // 其他测试用例可以仿照已有的写法进行添加
}
