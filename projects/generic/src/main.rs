fn main() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
    }

    impl Summary for NewsArticle {
        // fn summarize(&self) -> String {
        //     format!("{}, by {}", headline, location)
        // Then it uses default trait.
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // is equivalent with, trait bound syntax
    pub fn notify<T: Summary>(item: &T) {
        ...
    }

    // Multiple trait bounds
    pub fn notify(item: &(impl Summary + Display)) {
        ...
    }

    // or
    pub fn notify<T: Summary + Display>(item: &T) {
        ...
    }

    fn some_function<T, U>(t: &T, u: &U) -> i32
        where T: Display + Clone, U: Clone + Debug
    {

    }

    // Lifetimes
    fn main() {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}


// summary
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
