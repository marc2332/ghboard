use dioxus::prelude::*;

#[derive(Props)]
pub struct PageProps<'a> {
    #[props(into)]
    title: String,
    children: Element<'a>,
    #[props(optional)]
    theme: Option<&'a str>,
    #[props(optional)]
    size: Option<&'a str>,
}

#[allow(non_snake_case)]
pub fn Page<'a>(cx: Scope<'a, PageProps<'a>>) -> Element<'a> {
    let theme = cx.props.theme.unwrap_or_default();
    let size = cx.props.size.unwrap_or_default();
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
                href: "/public/style.css"
            }
        }
        body {
            class: "{theme}-theme {size}-size",
            div {
                class: "body-content",
                &cx.props.children
            }
        }
    )
}
