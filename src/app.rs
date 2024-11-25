use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(not(target_arch = "wasm32"))]
fn setup() {
    use crate::parser::generate_static_pages;
    use std::path::Path;
    // Generate all static pages whenever server is started
    let posts_dir = Path::new("content/posts");
    let html_dir = Path::new("static/posts");
    let posts = generate_static_pages(posts_dir, html_dir);
    logging::log!("{:#?}", posts);
}

#[cfg(target_arch = "wasm32")]
fn setup() {}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    console_error_panic_hook::set_once();
    setup();

    view! {
        <Stylesheet id="leptos" href="/pkg/cache-is-king.css"/>
        <Title text="Cache is King"/>

        <Router>
            <Navbar/>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
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
            <ul>
                <li><a href="/">"Home"</a></li>
                <li><a href="/posts">"Posts"</a></li>
            </ul>
        </nav>
    }
}

#[component]
fn PostList() -> impl IntoView {
    let posts = vec!["Making a Tech Blog"];
    view! {
        <div class="post-list">
            <h1>"Posts"</h1>

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

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Cache is King"</h1>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
