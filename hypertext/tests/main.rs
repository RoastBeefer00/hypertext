//! Tests for the `hypertext` crate.

#![allow(clippy::useless_vec)]

use hypertext::{Attribute, AttributeNamespace, GlobalAttributes, Renderable};

#[test]
fn readme() {
    use hypertext::{html_elements, GlobalAttributes, RenderIterator, Renderable};

    let shopping_list = vec!["milk", "eggs", "bread"];

    let shopping_list_maud = hypertext::maud! {
        div {
            h1 { "Shopping List" }
            ul {
                @for (&item, i) in shopping_list.iter().zip(1..) {
                    li.item {
                        input #{ "item-" (i) } type="checkbox";
                        label for={ "item-" (i) } { (item) }
                    }
                }
            }
        }
    }
    .render();

    // or, alternatively:

    let shopping_list_rsx = hypertext::rsx! {
        <div>
            <h1>Shopping List</h1>
            <ul>
                { shopping_list.iter().zip(1..).map(|(&item, i)| hypertext::rsx_move! {
                    <li class="item">
                        <input id=format!("item-{i}") type="checkbox">
                        <label for=format!("item-{i}")>{ item }</label>
                    </li>
                }).render_all() }
            </ul>
        </div>
    }
    .render();

    assert_eq!(shopping_list_maud, shopping_list_rsx);
}

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
trait HtmxAttributes: GlobalAttributes {
    const hx_post: Attribute = Attribute;
    const hx_on: AttributeNamespace = AttributeNamespace;
}

impl<T: GlobalAttributes> HtmxAttributes for T {}

#[test]
fn htmx() {
    use hypertext::{html_elements, Renderable};

    let htmx_maud = hypertext::maud! {
        div {
            form hx-post="/login" hx-on::after-request="this.reset()" {
                input type="text" name="username";
                input type="password" name="password";
                input type="submit" value="Login";
            }
        }
    }
    .render();

    assert_eq!(
        htmx_maud,
        r#"<div><form hx-post="/login" hx-on::after-request="this.reset()"><input type="text" name="username"><input type="password" name="password"><input type="submit" value="Login"></form></div>"#
    );
}

#[test]
fn elements_macro() {
    use hypertext::Renderable;

    mod html_elements {
        use hypertext::elements;
        pub use hypertext::html_elements::*;

        elements! {
            /// This is a test element
            my_element {
                /// This is a test attribute
                my_attribute
            }
        }
    }

    let custom_maud = hypertext::maud! {
        div {
            my_element my_attribute="test" {
                "Hello, world!"
            }
        }
    }
    .render();

    assert_eq!(
        custom_maud,
        r#"<div><my_element my_attribute="test">Hello, world!</my_element></div>"#
    );
}

#[test]
fn correct_attr_escape() {
    use hypertext::{html_elements, maud};

    let xss = r#""alert('XSS')"#;

    let test = maud! {
        div data-code=(xss) {}
    }
    .render();

    assert_eq!(test, r#"<div data-code="&quot;alert('XSS')"></div>"#);
}
