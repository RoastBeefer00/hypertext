use crate::views::nav::Nav;
use hypertext::prelude::*;

pub fn list(nav_oob: bool) -> impl Renderable {
    let list_items = vec!["Hypertext", "is", "fun!"];
    rsx! {
        @if nav_oob {
            <Nav selected=(String::from("/list")) oob=true />
        }
        <div class="flex flex-col items-center">
            <h1 class="text-4xl font-bold">"Loop through items using Rust code!"</h1>
            <ul class="mt-4 list-disc">
                @for item in &list_items {
                    <li class="mt-2">(item)</li>
                }
            </ul>
        </div>
    }
}
