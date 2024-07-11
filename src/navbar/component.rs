use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Navbar() -> Element {
    let links = [
        ("/home", "home"),
        ("/blog", "blog")
    ];

    rsx! {
        div {
            nav { class: "navbar",
                ul { class: "navbar-menu",
                    for (link , text) in links.iter() {
                        li {
                            a { class: "navbar-link", href: "{ link }", { text.to_uppercase() } }
                        }
                    }
                }
            }
        }
    }
}
