use dioxus::prelude::*;

#[derive(Props)]
pub struct PageProps<'a> {
    #[props(into)]
    title: String,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element<'a> {
    render!(
        head {
            title {
                "{cx.props.title}"
            }
            link {
                rel: "icon",
                href: "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🦑</text></svg>"
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1.0"
            }
            meta {
                name: "description",
                content: "GitHub Dashbard to track contributions."
            }
            link {
                rel: "stylesheet",
                href: "/assets/style.css"
            }
        }
        div {
            class: "body-content",
            &cx.props.children
        }
    )
}
