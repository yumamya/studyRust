use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use super::book::Book;

pub struct Bookshelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}

impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self { books: Vec::new(), matcher: matcher }
    }
    // 追加
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    // 検索
    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        self.books.iter().filter(|book| self.matcher.fuzzy_match(&book.title, title_query).is_some()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Book, Bookshelf};
    #[test]
    fn test_bookshelf() {
        let mut shelf = Bookshelf::new();
        let book1 = Book::new("すごいぞChatGPT！１AIを使って学ぼうRUST！", "山田太郎");
        let book2 = Book::new("Pythonプログラミング入門", "山田花子");
        shelf.add_book(book1);
        shelf.add_book(book2);

        let found_books = shelf.search_books("chatgpt");
        println!("{:?}", found_books)
    }
}