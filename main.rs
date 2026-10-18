use colored::Colorize;
use std::io;
enum BookStatus {
    Available,
    Borrowed,
    Discontinued,
}
struct Book {
    title: String,
    author: String,
    status: BookStatus,
}

impl Book {
    fn new(title: &str, author: &str) -> Book {
        let status = BookStatus::Available;
        Book {
            title: title.to_string(),
            author: author.to_string(),
            status,
        }
    }

    fn remove(&mut self) {
        self.status = BookStatus::Discontinued;
        println!("{} removed!", self.title.red().bold());
    }

    fn display_info(&self) {
        let status = match self.status {
            BookStatus::Available => "Available".green(),
            BookStatus::Borrowed => "Borrowed".red(),
            BookStatus::Discontinued => "Discontinued".blue(),
        };
        println!("Title: {}, Author: {}, Status: {}", self.title, self.author, status);
    }

    fn borrow(&mut self) {
        self.status = BookStatus::Borrowed;
        println!("{} borrowed!", self.title.red().bold());
    }

    fn return_book(&mut self) {
        self.status = BookStatus::Available;
        println!("{} returned!", self.title.red().bold());
    }

    fn search_by_title(title: &str, books: &Vec<Book>) -> bool {
        for book in books {
            if book.title.to_lowercase().contains(&title.to_lowercase()) {
                return true;
            }
        }
        false
    }

    fn search_by_author(author: &str, books: &Vec<Book>) -> bool {
        for book in books {
            if book.author.contains(&author) {
                return true;
            }
        }
        false
    }

    fn print_all_books(books: &Vec<Book>) {
        for book in books {
            let status = match book.status {
                BookStatus::Available => "Available".green(),
                BookStatus::Borrowed => "Borrowed".red(),
                BookStatus::Discontinued => "Discontinued".blue(),
            };
            println!("{} : {} , {} : {} , {} : {}" ,"Title".blue() ,book.title,"Author".yellow() ,book.author,"Status".red(), status);
        }
    }

    fn sort_by_alpha(books: &mut Vec<Book>) {
        books.sort_by(|a, b| a.title.cmp(&b.title));
        println!("{}", "Books sorted!".green().bold());
        let mut count=0;
        for book in books {
            count+=1;
            println!("Title {} : {}",count,book.title);
        }
    }
}

fn main() {
    println!();
    println!( "                        ~~~~~~~~~~~~~~~~~~~~~~~~>{}<~~~~~~~~~~~~~~~~~~~~~~~~","Welcome to our Library!".white().on_blue().bold());
    let mut books: Vec<Book> = Vec::new();

    loop {
        println!();
        println!("{}", "Select a number from the list below:".green());
        println!("{}", "1.Add book".yellow());
        println!("{}", "2.Remove book".yellow());
        println!("{}", "3.Display info".yellow());
        println!("{}", "4.Borrow".yellow());
        println!("{}", "5.Return book".yellow());
        println!("{}", "6.Search by title".yellow());
        println!("{}", "7.Search by author".yellow());
        println!("{}", "8.Print all books".yellow());
        println!("{}", "9.Sort by alpha".yellow());
        println!("{}", "10.Exit".red());

        let mut my_str = String::new();
        io::stdin().read_line(&mut my_str).unwrap();
        let n: i32 = my_str.trim().parse().unwrap();

        match n {
            1 => {
                println!("{}", "Enter title book:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let title = my_str.trim();

                println!("{}", "Enter author book:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let author = my_str.trim();

                let book = Book::new(title, author);
                books.push(book);
            }
            2 => {
                println!("{}", "Enter title book to remove:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let title = my_str.trim();

                let mut found = false;
                for i in books.iter_mut() {
                    if i.title == title {
                        i.remove();
                       found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}", "Book not found!".red().bold());
                }
            }
            3 => {
                println!("{}", "Enter title book to display:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let title = my_str.trim();

                let mut found = false;
                for i in &books {
                    if i.title == title {
                        i.display_info();
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}", "Book not found!".red().bold());
                }
            }
            4 => {
                println!("{}", "Enter title book to borrow:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let title = my_str.trim();

                let mut found = false;
                for i in books.iter_mut() {
                    if i.title == title {
                        i.borrow();
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}", "Book not found!".red().bold());
                }
            }
            5 => {
                println!("{}", "Enter title book to return:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let title = my_str.trim();

                let mut found = false;
                for i in books.iter_mut() {
                    if i.title == title {
                        i.return_book();
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("{}", "Book not found!".red().bold());
                }
            }
            6 => {
                println!("{}", "Enter title book to search:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let title = my_str.trim();

                if Book::search_by_title(title, &books) {
                    println!("{}", "Book found!".green().bold());
                } else {
                    println!("{}", "Book not found!".red().bold());
                }
            }
            7 => {
                println!("{}", "Enter author book to search:".white());
                let mut my_str = String::new();
                io::stdin().read_line(&mut my_str).unwrap();
                let author = my_str.trim();

                if Book::search_by_author(author, &books) {
                    println!("{}", "Author found!".green().bold());
                } else {
                    println!("{}", "Author not found!".red().bold());
                }
            }
            8 => {
                if books.is_empty() {
                    println!("{}", "No book!".red().bold());
                } else {
                    Book::print_all_books(&books);
                }
            }
            9 => {
                if books.is_empty() {
                    println!("{}", "No book!".red().bold());
                } else {
                    Book::sort_by_alpha(&mut books);
                }
            }
            10 => {
                println!("                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~>{}<~~~~~~~~~~~~~~~~~~~~~~~~~~~","Have good time!".white().on_blue().bold());
                break;
            }
            _ => println!("{}", "Wrong!!".red().bold()),
        }
    }
}