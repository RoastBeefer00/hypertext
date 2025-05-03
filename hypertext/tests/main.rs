//! Tests for the `hypertext` crate.

#[test]
fn readme() {
    use hypertext::{html_elements, GlobalAttributes, RenderIterator, Renderable};

    let shopping_list = ["milk", "eggs", "bread"];

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

#[test]
fn htmx() {
    use hypertext::{html_elements, Attribute, AttributeNamespace, GlobalAttributes, Renderable};

    #[allow(non_upper_case_globals)]
    #[allow(dead_code)]
    trait HtmxAttributes: GlobalAttributes {
        const hx_post: Attribute = Attribute;
        const hx_on: AttributeNamespace = AttributeNamespace;
    }

    impl<T: GlobalAttributes> HtmxAttributes for T {}

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
fn can_render_arc() {
    use hypertext::{html_elements, Renderable};

    let value = std::sync::Arc::new("arc");
    let result = hypertext::maud!(span { (value) }).render();

    assert_eq!(result, "<span>arc</span>");
}

#[test]
fn can_render_box() {
    use hypertext::{html_elements, Renderable};

    let value = Box::new("box");
    let result = hypertext::maud!(span { (value) }).render();

    assert_eq!(result, "<span>box</span>");
}

#[test]
fn can_render_rc() {
    use hypertext::{html_elements, Renderable};

    let value = std::rc::Rc::new("rc");
    let result = hypertext::maud!(span { (value) }).render();

    assert_eq!(result, "<span>rc</span>");
}

#[test]
fn can_render_cow() {
    use hypertext::{html_elements, Renderable};

    let value = std::borrow::Cow::from("cow");
    let result = hypertext::maud!(span { (value) }).render();

    assert_eq!(result, "<span>cow</span>");
}

#[test]
fn can_render_vec() {
    use hypertext::{html_elements, maud_move, Renderable};

    let groceries = ["milk", "eggs", "bread"]
        .into_iter()
        .map(|s| maud_move! { li { (s) } })
        .collect::<Vec<_>>();

    let result = hypertext::maud! {
        ul { (groceries) }
    }
    .render();

    assert_eq!(result, "<ul><li>milk</li><li>eggs</li><li>bread</li></ul>");
}
