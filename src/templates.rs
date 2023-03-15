use ramhorns::{Content, Template};

#[derive(Content)]
struct TemplatePost<'a> {
    title: &'a str,
    description: &'a str,
}

#[derive(Content)]
struct TemplateFeed<'a> {
    posts: Vec<TemplatePost<'a>>,
}

pub fn render_html() -> String {
    // TODO: load template string from file
    let source = "";

    let tpl = Template::new(source).unwrap();

    tpl.render(&TemplateFeed {
        posts: vec![
            TemplatePost {
                title: "How I tried Ramhorns and found love 💖",
                description: "This can happen to you too",
            },
            TemplatePost {
                title: "Rust is kinda awesome",
                description: "Yes, even the borrow checker! 🦀",
            },
        ],
    })
}
