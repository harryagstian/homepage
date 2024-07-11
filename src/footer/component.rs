#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsGithub, BsLinkedin},
    Icon,
};

fn get_current_version() -> String {
    let app_version = std::option_env!("CARGO_PKG_VERSION").unwrap_or_else(|| "unknown");
    let build_method = std::option_env!("BUILD_METHOD").unwrap_or_else(|| "unknown");

    format!("{}-{}", app_version, build_method)
}

pub fn Footer() -> Element {
    let version = get_current_version();

    rsx! {
        div {
            footer { class: "footer",
                div { class: "footer-links",
                    span {
                        a {
                            href: "https://github.com/harryagstian/",
                            target: "_blank",
                            Icon { width: 24, height: 24, fill: "white", icon: BsGithub }
                        }
                    }

                    span {
                        a {
                            href: "https://www.linkedin.com/in/harryagustian/",
                            target: "_blank",
                            Icon { width: 24, height: 24, fill: "white", icon: BsLinkedin }
                        }
                    }
                }

                div { class: "footer-notes",
                    span {
                        a {
                            class: "source-code",
                            href: "https://github.com/harryagstian",
                            target: "_blank",
                            { format!("Version: {}", version) }
                        }
                    }
                    span { { "Made with ❤️" } }
                    span { { "Fueled by 🍣🍣🍣" } }
                }
            }
        }
    }
}
