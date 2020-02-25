use std::cmp::min;
use std::convert::TryInto;
mod reader;

struct System {
    book_scores: Vec<usize>,
    libraries: Vec<Library>,
    days: usize,
}
struct Library {
    id: usize,
    signup_duration: usize,
    books_per_day: usize,
    books: Vec<usize>,
}
impl System {
    pub fn from_file(file_name: &str) -> System {
        let mut reader = reader::Reader::from_file(file_name);

        let books_count: usize = reader.next();
        let libraries_count: usize = reader.next();
        let days = reader.next();
        let book_scores = (0..books_count).map(|_| reader.next()).collect();
        let libraries = (0..libraries_count).map(|id| {
            let books_count = reader.next();
            let signup_duration = reader.next();
            let books_per_day = reader.next();
            let books = (0..books_count).map(|_| reader.next()).collect();
            Library{ id, signup_duration, books_per_day, books }
        }).collect();

        System{ book_scores, libraries, days, }
    }
}

fn main() {
    let file_names = [ "a_example.txt", "b_read_on.txt", "c_incunabula.txt", "d_tough_choices.txt", "e_so_many_books.txt",  "f_libraries_of_the_world.txt" ];

    for file_name in &file_names {
        let mut system = System::from_file(file_name);

        let mut count_libraries = 0;
        let mut current_day = 0;
        let mut result = String::from("");

        system.libraries.sort_unstable_by_key(|l| l.books_per_day);
        system.libraries.reverse();


        for library in &mut system.libraries {
            current_day += library.signup_duration;
            if current_day > system.days {
                break
            }

            let max_n_books = (system.days - current_day) * library.books_per_day;
            let books_count = library.books.len();
            let n_books = min(max_n_books, books_count.try_into().unwrap());

            let book_scores = &system.book_scores;
            library.books.sort_unstable_by_key(|b| book_scores[*b]);
            library.books.reverse();

            let fbooks = library.books[0..n_books].iter().map(ToString::to_string).collect::<Vec<_>>().join(" ");
            result = format!("{}{} {}\n{}\n", result, library.id, n_books, fbooks);

            count_libraries += 1;
        }

        println!("{}", count_libraries);
        println!("{}", result);
    }
}
