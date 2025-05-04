//! Attribute traits for various frameworks.
//!
//! To use these attributes, you need to enable the corresponding feature in
//! your `Cargo.toml` file. For example, to use [`HtmxAttributes`], you would
//! enable the `htmx` feature:
//!
//! ```toml
//! [dependencies]
//! hypertext = { version = "*", features = ["htmx"] }
//! ```
//!
//! Then you can use the attributes in your code after bringing the trait into
//! scope:
//!
//! ```rust
//! use hypertext::{Attribute, GlobalAttributes, frameworks::HtmxAttributes, html_elements, maud};
//!
//! # assert_eq!(hypertext::Renderable::render(&
//! maud! {
//!     a hx-get="/about" { "About" }
//! }
//! # ), hypertext::Rendered(r#"<a hx-get="/about">About</a>"#));
//! ```

#[cfg(feature = "htmx")]
pub use htmx::HtmxAttributes;

#[cfg(feature = "htmx")]
mod htmx {
    use crate::{Attribute, AttributeNamespace, GlobalAttributes};

    /// Attributes for use with [htmx](https://htmx.org/).
    #[allow(non_upper_case_globals, clippy::doc_markdown)]
    pub trait HtmxAttributes: GlobalAttributes {
        /// Issues a GET to the specified URL
        const hx_get: Attribute = Attribute;
        /// Issues a POST to the specified URL
        const hx_post: Attribute = Attribute;
        /// Handle events with inline scripts on elements
        const hx_on: AttributeNamespace = AttributeNamespace;
        /// Push a URL into the browser location bar to create history
        const hx_push_url: Attribute = Attribute;
        /// Select content to swap in from a response
        const hx_select: Attribute = Attribute;
        /// Select content to swap in from a response, somewhere other than the
        /// target (out of band)
        const hx_select_oob: Attribute = Attribute;
        /// Controls how content will swap in (outerHTML, beforeend, afterend,
        /// …)
        const hx_swap: Attribute = Attribute;
        /// Mark element to swap in from a response (out of band)
        const hx_swap_oob: Attribute = Attribute;
        /// Specifies the target element to be swapped
        const hx_target: Attribute = Attribute;
        /// Specifies the event that triggers the request
        const hx_trigger: Attribute = Attribute;
        /// Add values to submit with the request (JSON format)
        const hx_vals: Attribute = Attribute;
        /// Add progressive enhancement for links and forms
        const hx_boost: Attribute = Attribute;
        /// Shows a confirm() dialog before issuing a request
        const hx_confirm: Attribute = Attribute;
        /// Issues a DELETE to the specified URL
        const hx_delete: Attribute = Attribute;
        /// Disables htmx processing for the given node and any children nodes
        const hx_disable: Attribute = Attribute;
        /// Adds the disabled attribute to the specified elements while a
        /// request is in flight
        const hx_disabled_elt: Attribute = Attribute;
        /// Control and disable automatic attribute inheritance for child nodes
        const hx_disinherit: Attribute = Attribute;
        /// Changes the request encoding type
        const hx_encoding: Attribute = Attribute;
        /// Extensions to use for this element
        const hx_ext: Attribute = Attribute;
        /// Adds to the headers that will be submitted with the request
        const hx_headers: Attribute = Attribute;
        /// Prevent sensitive data being saved to the history cache
        const hx_history: Attribute = Attribute;
        /// The element to snapshot and restore during history navigation
        const hx_history_elt: Attribute = Attribute;
        /// Include additional data in requests
        const hx_include: Attribute = Attribute;
        /// The element to put the htmx-request class on during the request
        const hx_indicator: Attribute = Attribute;
        /// Control and enable automatic attribute inheritance for child nodes
        /// if it has been disabled by default
        const hx_inherit: Attribute = Attribute;
        /// Filters the parameters that will be submitted with a request
        const hx_params: Attribute = Attribute;
        /// Issues a PATCH to the specified URL
        const hx_patch: Attribute = Attribute;
        /// Specifies elements to keep unchanged between requests
        const hx_preserve: Attribute = Attribute;
        /// Shows a prompt() before submitting a request
        const hx_prompt: Attribute = Attribute;
        /// Issues a PUT to the specified URL
        const hx_put: Attribute = Attribute;
        /// Replace the URL in the browser location bar
        const hx_replace_url: Attribute = Attribute;
        /// Configures various aspects of the request
        const hx_request: Attribute = Attribute;
        /// Control how requests made by different elements are synchronized
        const hx_sync: Attribute = Attribute;
        /// Force elements to validate themselves before a request
        const hx_validate: Attribute = Attribute;
        /// Adds values dynamically to the parameters to submit with the request
        /// (deprecated, please use hx-vals)
        #[deprecated = "use `hx-vals` instead"]
        const hx_vars: Attribute = Attribute;
    }

    impl<T: GlobalAttributes> HtmxAttributes for T {}
}
