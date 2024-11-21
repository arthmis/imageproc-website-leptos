use leptos::{component, view, IntoView};

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <a class="navbar-item navbar-end" href="https://github.com/arthmis/imageproc-website">
                <i class="fa fa-github" aria-hidden="true" style="font-size:1.4em;"></i>
                "Github"
            </a>
        </nav>
    }
}
