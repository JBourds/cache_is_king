#![allow(non_snake_case)]
use leptos::*;
use leptos_router::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <Blog/> })
}

#[component]
fn Blog() -> impl IntoView {
    view! {
        <Router>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/posts" view=PostList>
                        <Route path=":name" view=Post/>
                        <Route path="" view=|| view! {
                            <div class="select-post">
                                "Select a post to start reading!"
                            </div>
                        }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <nav>
            <a href="/">"Home"</a>
            <a href="/posts">"Posts"</a>
        </nav>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <h1>"Home"</h1> }
}

#[component]
fn PostList() -> impl IntoView {
    let posts = vec!["Making a Tech Blog"];
    view! {
        <div class="post-list">
            <h3>"Posts"</h3>

            <ul>
                {
                posts.into_iter()
                    .map(|name| view! { <li><A href=name>{name}</A></li> })
                    .collect_view()
                }
            </ul>

            <Outlet/>
        </div>
    }
}

#[component]
fn Post() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|params| params.get("name").cloned().unwrap_or(String::new()));
    view! { <p>{name}</p> }
}
