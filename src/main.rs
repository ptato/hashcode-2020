use std::collections::HashSet;
mod reader;

struct System {
    book_scores: Vec<u32>,
    libraries: Vec<Library>,
    days: u32,
}
struct Library {
    id: u32,
    signup_duration: u32,
    books_per_day: u32,
    books: HashSet<u32>,
}
impl System {
    pub fn from_file(file_name: &str) -> System {
        let mut reader = reader::Reader::from_file(file_name);

        let books_count: u32 = reader.next();
        let libraries_count: u32 = reader.next();
        let days = reader.next();
        let book_scores = (0..books_count).map(|i| reader.next()).collect();
        let libraries = (0..libraries_count).map(|id| {
            let books_count = reader.next();
            let signup_duration = reader.next();
            let books_per_day = reader.next();
            let mut books = (0..books_count).map(|i| reader.next()).collect();
            Library{ id, signup_duration, books_per_day, books }
        }).collect();

        System{ book_scores, libraries, days, }
    }
}

fn main() {
    let file_names = [ "a_example.txt", "b_read_on.txt", "c_incunabula.txt", "d_tough_choices.txt", "e_so_many_books.txt",  "f_libraries_of_the_world.txt" ];

    for file_name in &file_names {
        let system = System::from_file(file_name);
    }
}
