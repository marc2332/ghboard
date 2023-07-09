use crate::components::page::Page;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct HomeRouteProps {}

pub fn home_route(cx: Scope<HomeRouteProps>) -> Element {
    render!(
        Page {
            title: "gboard",
            div {
                h1 {
                    class: "text-2xl my-8",
                    "🦑 gboard"
                }
                h3 {
                    class: "text-2x",
                    "GitHub dashboard written in Rust🦀, made using Dioxus SSR 🧬, hosted in Shuttle 🚀 and powered by the GitHub GraphQL API 🦑."
                }
                b {
                    "Don't forget to star the ",
                    a {
                        class: "underline",
                        href: "https://github.com/marc2332/ghboard",
                        "repository ⭐😄"
                    }
                }
                div {
                    class: "flex justify-center p-10",
                    input {
                        class: "text-black p-2 rounded-md mx-2",
                        id: "username",
                        placeholder: "Your GitHub username"
                    }
                    button {
                        class: "text-black px-4 py-1 rounded-md bg-green-600 text-white",
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
