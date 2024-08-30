pub trait ContainsGlobalAttributes: crate::html::HtmlElement {
    /// Provides a hint for generating a keyboard shortcut for the current element
    #[inline]
    fn access_key<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "accesskey",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// Controls whether and how text input is automatically capitalized as it is entered/edited by the user
    #[inline]
    fn auto_capitalize<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "autocapitalize",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// Indicates that an element should be focused on page load, or when the <dialog> that it is part of is displayed
    #[inline]
    fn autofocus(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "autofocus",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }

    /// A space-separated list of the case-sensitive classes of the element
    #[inline]
    fn class<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "class",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// Indicates if the element should be editable by the user
    #[inline]
    fn content_editable<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "contenteditable",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// Indicates the directionality of the element's text
    #[inline]
    fn direction<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "dir",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// Indicates whether the element can be dragged, either with native browser behavior or the HTML Drag and Drop API.
    #[inline]
    fn draggable(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "draggable",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }

    /// Defines what action label (or icon) to present for the enter key on virtual keyboards
    #[inline]
    fn enter_key_hint<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "enterkeyhint",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The exportparts global attribute allows you to select and style elements existing in nested shadow trees, by exporting their part names
    #[inline]
    fn export_parts<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "exportparts",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// Indicates that the browser should not render the contents of the element
    #[inline]
    fn hidden<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "hidden",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// Defines an identifier (ID) which must be unique in the whole document
    #[inline]
    fn id<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut()
            .insert_attribute("id", crate::ast::AttributeValue::KeyValuePair(value.into()));
        self
    }

    /// indicating that the browser will ignore the element
    #[inline]
    fn inert(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut()
            .insert_attribute("inert", crate::ast::AttributeValue::BooleanAttribute(value));
        self
    }

    /// hints at the type of data that might be entered by the user while editing the element or its contents
    #[inline]
    fn input_mode<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "inputmode",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// allows you to specify that a standard HTML element should behave like a defined custom built-in element
    #[inline]
    fn is_<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut()
            .insert_attribute("is", crate::ast::AttributeValue::KeyValuePair(value.into()));
        self
    }

    /// The itemid global attribute provides microdata in the form of a unique, global identifier of an item
    #[inline]
    fn item_id<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "itemid",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The itemprop global attribute is used to add properties to an item
    #[inline]
    fn item_prop<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "itemprop",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// Properties that are not descendants of an element with the itemscope attribute can be associated with an item using the global attribute itemref
    #[inline]
    fn item_ref<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "itemref",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// itemscope is a boolean global attribute that defines the scope of associated metadata
    #[inline]
    fn item_scope<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "itemscope",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The global attribute itemtype specifies the URL of the vocabulary that will be used to define itemprop's (item properties) in the data structure
    #[inline]
    fn item_type<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "itemtype",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The lang global attribute helps define the language of an element: the language that non-editable elements are written in, or the language that the editable elements should be written in by the user
    #[inline]
    fn lang<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "lang",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The nonce global attribute is a content attribute defining a cryptographic nonce ("number used once") which can be used by Content Security Policy to determine whether or not a given fetch will be allowed to proceed for a given element
    #[inline]
    fn nonce<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "nonce",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The part global attribute contains a space-separated list of the part names of the element
    #[inline]
    fn part<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "part",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The slot global attribute assigns a slot in a shadow DOM shadow tree to an element: An element with a slot attribute is assigned to the slot created by the <slot> element whose name attribute's value matches that slot attribute's value
    #[inline]
    fn slot<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "slot",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The spellcheck global attribute is an enumerated attribute that defines whether the element may be checked for spelling errors
    #[inline]
    fn spellcheck<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "spellcheck",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The style global attribute contains CSS styling declarations to be applied to the element
    #[inline]
    fn style<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "style",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The tabindex global attribute allows developers to make HTML elements focusable, allow or prevent them from being sequentially focusable (usually with the Tab key, hence the name) and determine their relative ordering for sequential focus navigation
    #[inline]
    fn tab_index(mut self: Box<Self>, value: i64) -> Box<Self> {
        let mut buffer = itoa::Buffer::new();
        let value = buffer.format(value).to_string();
        self.get_dom_element_mut().insert_attribute(
            "tabindex",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The title global attribute contains text representing advisory information related to the element it belongs to
    #[inline]
    fn title<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::std::borrow::Cow<'static, str>>,
    {
        self.get_dom_element_mut().insert_attribute(
            "title",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }

    /// The translate global attribute is an enumerated attribute that is used to specify whether an element's translatable attribute values and its Text node children should be translated when the page is localized, or whether to leave them unchanged
    #[inline]
    fn translate(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "translate",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
