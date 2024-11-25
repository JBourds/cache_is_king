#![allow(non_snake_case)]
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Blog/> })
}

#[component]
fn Blog() -> impl IntoView {
    let posts = vec![("Making a Tech Blog", "/posts/making-a-tech-blog")];
    view! {
        <ul>
            {
                posts.into_iter()
                .map(|(file, url)| view! { <a href={url}>{file}</a> })
                .collect_view()
            }
        </ul>

    }
}
