struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let ie = ImportantExcerpt { part: "Almost done!" };
    println!("{}", ie.level());
    let p = ie.announce_and_return_part("it works");
    println!("{p}");
    
}
