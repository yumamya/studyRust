mod library {
    mod book {
        pub struct Book {
            title: String,
            author: String,
        }
        impl Book {
            fn new(title: &str, author: &str) -> Self {
                Self {
                    title: title.to_string(),
                    author: author.to_string(),
                }
            }
        }
    }
    mod magazine {
        pub struct Page {
            pub content: String,
        }
    }
    mod bookshelf {
        use super::book::Book;
        use super::magazine::Page as MagazinePage;

        fn some_function() {
            Page { content: "Hello".to_string() }
        }

        pub struct Bookshelf {
            books: Vec<Book>,
        }
        impl Bookshelf {
            pub fn new() -> Self {
                Self { books: Vec::new() }
            }

            // 追加
            pub fn add_book(&mut self, book: Book) {
                self.books.push(book);
            }

            // 検索
            pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
                todo!("Implement 'Bookshelf::search_books'");
            }

            // 取り出すメソッド
            pub fn remove_book(&mut self, book: &Book) -> Option<Book> {
                todo!("Implement 'Bookshelf::remove_book'");
            }

            // 全て取り出すメソッド
            pub fn take_all_books(&mut self) -> Vec<Book> {
                todo!("Implement 'Bookshelf::take_all_books'");
            }
        }
    }
}
