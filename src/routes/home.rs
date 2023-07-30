use crate::components::page::Page;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct HomeRouteProps {}

pub fn home_route(cx: Scope<HomeRouteProps>) -> Element {
    render!(
        Page {
            title: "ghboard",
            div {
                h1 {
                    class: "title",
                    "🦑 ghboard"
                }
                p {
                    class: "subtitle",
                    "GitHub dashboard written in Rust🦀, made using Dioxus SSR 🧬, hosted in Shuttle 🚀 and powered by the GitHub GraphQL API 🦑."
                }
                b {
                    "Don't forget to star the ",
                    a {
                        href: "https://github.com/marc2332/ghboard",
                        "repository ⭐😄"
                    }
                }
                div {
                    class: "centered-box",
                    input {
                        id: "username",
                        placeholder: "Your GitHub username"
                    }
                    button {
                        id: "continue",
                        "Continue"
                    }
                }
            }
            script {
                "
                    const button = document.getElementById(\'continue\');
                    const input = document.getElementById(\'username\');
                    button.onclick = () => {{
                        location.pathname = 'user/' + input.value
                    }}
                "
            }
        }
    )
}
