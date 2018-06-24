use comrak::{markdown_to_html, ComrakOptions};
use std::fs::File;
use std::io::{self, Read};

pub fn get_page(file: &str) -> io::Result<String> {
    let mut file = File::open(file)?;
    let mut md = String::new();
    file.read_to_string(&mut md)?;

    Ok(markdown_to_html(&*md, &ComrakOptions::default()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use comrak::{markdown_to_html, ComrakOptions};
        let markdown = r#"
1. A list
2. With some
3. <span>Inline html</span>
        "#;

        let html = markdown_to_html(markdown, &ComrakOptions::default());
        assert_eq!(
            "<ol>\n<li>A list</li>\n<li>With some</li>\n<li><span>Inline html</span></li>\n</ol>\n",
            html
        );
    }
}
