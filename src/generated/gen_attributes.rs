// Warning: Generated code, code might be faulty

use attribute_types::{
    AnchorTarget, AreaShape, ButtonType, CrossOrigin, FormDialogMethod, FormEncodingType,
    FormMethod, HttpEquiv, ImageDecoding, InputType, LinkType, Metadata, OnOff, OrderedListType,
    Preload, ReferrerPolicy, TableHeaderScope, VideoKind, Wrap,
};

pub trait Hreflang: crate::html::HtmlElement {
    ///Language of the linked resource
    #[inline]
    fn hreflang<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "hreflang",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Usemap: crate::html::HtmlElement {
    ///Name of image map to use
    #[inline]
    fn usemap<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "usemap",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Autocomplete: crate::html::HtmlElement {
    ///Default setting for autofill feature for controls in the form
    #[inline]
    fn autocomplete<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "autocomplete",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AutocompleteOnoff: crate::html::HtmlElement {
    ///Default setting for autofill feature for controls in the form
    #[inline]
    fn autocomplete(mut self: Box<Self>, value: OnOff) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "autocomplete",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaDropEffect: crate::html::HtmlElement {
    ///[Deprecated in ARIA 1.1] Indicates what functions can be performed when a dragged object is released on the drop target.
    #[inline]
    fn aria_drop_effect<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-dropeffect",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait FormMethodTrait: crate::html::HtmlElement {
    ///Variant to use for form submission
    #[inline]
    fn form_method(mut self: Box<Self>, value: FormMethod) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "formmethod",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait FormDialogMethodTrait: crate::html::HtmlElement {
    ///Variant to use for form submission
    #[inline]
    fn form_method(mut self: Box<Self>, value: FormDialogMethod) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "formmethod",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaGrabbed: crate::html::HtmlElement {
    ///[Deprecated in ARIA 1.1] Indicates an element's "grabbed" state in a drag-and-drop operation.
    #[inline]
    fn aria_grabbed(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-grabbed",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Blocking: crate::html::HtmlElement {
    ///Whether the element is potentially render-blocking
    #[inline]
    fn blocking<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "blocking",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaColSpan: crate::html::HtmlElement {
    ///Defines the number of columns spanned by a cell or gridcell within a table, grid, or treegrid. See related aria-colindex and aria-rowspan.
    #[inline]
    fn aria_col_span(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-colspan",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaRelevant: crate::html::HtmlElement {
    ///Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified. See related aria-atomic.
    #[inline]
    fn aria_relevant<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-relevant",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Srcset: crate::html::HtmlElement {
    ///Images to use in different situations, e.g., high-resolution displays, small monitors, etc.
    #[inline]
    fn srcset<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "srcset",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Coords: crate::html::HtmlElement {
    ///Coordinates for the shape to be created in an image map
    #[inline]
    fn coords<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "coords",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaFlowToElements: crate::html::HtmlElement {
    ///Identifies the next element (or elements) in an alternate reading order of content which, at the user's discretion, allows assistive technology to override the general default of reading in document source order.
    #[inline]
    fn aria_flow_to_elements<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-flowto",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaReadOnly: crate::html::HtmlElement {
    /// Indicates that the element is not editable, but is otherwise operable. See related aria-disabled.
    #[inline]
    fn aria_read_only(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-readonly",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait ReferrerpolicyTrait: crate::html::HtmlElement {
    ///Referrer policy for fetches initiated by the element
    #[inline]
    fn referrerpolicy(mut self: Box<Self>, value: ReferrerPolicy) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "referrerpolicy",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaKeyShortcuts: crate::html::HtmlElement {
    ///Defines keyboard shortcuts that an author has implemented to activate or give focus to an element.
    #[inline]
    fn aria_key_shortcuts<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-keyshortcuts",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Sandbox: crate::html::HtmlElement {
    ///Security rules for nested content
    #[inline]
    fn sandbox<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "sandbox",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaBrailleLabel: crate::html::HtmlElement {
    ///Defines a string value that labels the current element, which is intended to be converted into Braille. See related aria-label.
    #[inline]
    fn aria_braille_label<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-braillelabel",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait HttpEquivTrait: crate::html::HtmlElement {
    ///Pragma directive
    #[inline]
    fn http_equiv(mut self: Box<Self>, value: HttpEquiv) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "http-equiv",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaDescription: crate::html::HtmlElement {
    ///Defines a string value that describes or annotates the current element. See related aria-describedby.
    #[inline]
    fn aria_description<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-description",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Fetchpriority: crate::html::HtmlElement {
    ///Sets the priority for fetches initiated by the element
    #[inline]
    fn fetchpriority<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "fetchpriority",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Integrity: crate::html::HtmlElement {
    ///Integrity metadata used in Subresource Integrity checks [SRI]
    #[inline]
    fn integrity<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "integrity",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Charset: crate::html::HtmlElement {
    ///Character encoding declaration
    #[inline]
    fn charset<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "charset",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaSelected: crate::html::HtmlElement {
    ///Indicates the current "selected" state of various widgets. See related aria-checked and aria-pressed.
    #[inline]
    fn aria_selected(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-selected",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait AriaControlsElements: crate::html::HtmlElement {
    ///Identifies the element (or elements) whose contents or presence are controlled by the current element. See related aria-owns.
    #[inline]
    fn aria_controls_elements<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-controls",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Decoding: crate::html::HtmlElement {
    ///Decoding hint to use when processing this image for presentation
    #[inline]
    fn decoding(mut self: Box<Self>, value: ImageDecoding) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "decoding",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Disabled: crate::html::HtmlElement {
    ///Whether the form control is disabled
    #[inline]
    fn disabled(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "disabled",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Color: crate::html::HtmlElement {
    ///Color to use when customizing a site's icon (for rel="mask-icon")
    #[inline]
    fn color<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "color",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Target: crate::html::HtmlElement {
    ///Navigable for hyperlink navigation
    #[inline]
    fn target<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "target",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AnchorTargetTrait: crate::html::HtmlElement {
    ///Navigable for hyperlink navigation
    #[inline]
    fn target(mut self: Box<Self>, value: AnchorTarget) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "target",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaModal: crate::html::HtmlElement {
    ///Indicates whether an element is modal when displayed.
    #[inline]
    fn aria_modal(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-modal",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Checked: crate::html::HtmlElement {
    ///Whether the control is checked
    #[inline]
    fn checked<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "checked",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaExpanded: crate::html::HtmlElement {
    ///Indicates whether a grouping element owned or controlled by this element is expanded or collapsed.
    #[inline]
    fn aria_expanded(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-expanded",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Rowspan: crate::html::HtmlElement {
    ///Number of rows that the cell is to span
    #[inline]
    fn rowspan<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "rowspan",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaRowIndexText: crate::html::HtmlElement {
    ///Defines a human readable text alternative of aria-rowindex. See related aria-colindextext.
    #[inline]
    fn aria_row_index_text<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-rowindextext",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Autoplay: crate::html::HtmlElement {
    ///Hint that the media resource can be started automatically when the page is loaded
    #[inline]
    fn autoplay<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "autoplay",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaValueNow: crate::html::HtmlElement {
    ///Defines the current value for a range widget. See related aria-valuetext.
    #[inline]
    fn aria_value_now(mut self: Box<Self>, value: f64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-valuenow",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Ismap: crate::html::HtmlElement {
    ///Whether the image is a server-side image map
    #[inline]
    fn ismap<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "ismap",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Cols: crate::html::HtmlElement {
    ///Maximum number of characters per line
    #[inline]
    fn cols(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "cols",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Rows: crate::html::HtmlElement {
    ///Number of lines to show
    #[inline]
    fn rows(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "rows",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Type: crate::html::HtmlElement {
    ///Hint for the type of the referenced resource
    #[inline]
    fn type_<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "type",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait ButtonTypeTrait: crate::html::HtmlElement {
    ///Hint for the type of the referenced resource
    #[inline]
    fn type_(mut self: Box<Self>, value: ButtonType) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "type",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait InputTypeTrait: crate::html::HtmlElement {
    ///Hint for the type of the referenced resource
    #[inline]
    fn type_(mut self: Box<Self>, value: InputType) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "type",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait OrderedListTypeTrait: crate::html::HtmlElement {
    ///Hint for the type of the referenced resource
    #[inline]
    fn type_(mut self: Box<Self>, value: OrderedListType) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "type",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaDetailsElements: crate::html::HtmlElement {
    ///Identifies the element (or elements) that provide additional information related to the object. See related aria-describedby.
    #[inline]
    fn aria_details_elements<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-details",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Headers: crate::html::HtmlElement {
    ///The header cells for this cell
    #[inline]
    fn headers<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "headers",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait DataTrait: crate::html::HtmlElement {
    ///Address of the resource
    #[inline]
    fn data_trait<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "data",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Shape: crate::html::HtmlElement {
    ///The kind of shape to be created in an image map
    #[inline]
    fn shape(mut self: Box<Self>, value: AreaShape) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "shape",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Ping: crate::html::HtmlElement {
    ///URLs to ping
    #[inline]
    fn ping<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "ping",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Accept: crate::html::HtmlElement {
    ///Hint for expected file type in file upload controls
    #[inline]
    fn accept<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "accept",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait SpanTrait: crate::html::HtmlElement {
    ///Number of columns spanned by the element
    #[inline]
    fn span_trait<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "span",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Min: crate::html::HtmlElement {
    ///Minimum value
    #[inline]
    fn min<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "min",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaPlaceholder: crate::html::HtmlElement {
    ///Defines a short hint (a word or short phrase) intended to aid the user with data entry when the control has no value. A hint could be a sample value or a brief description of the expected format.
    #[inline]
    fn aria_placeholder<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-placeholder",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Srclang: crate::html::HtmlElement {
    ///Language of the text track
    #[inline]
    fn srclang<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "srclang",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaRequired: crate::html::HtmlElement {
    ///Indicates that user input is required on the element before a form can be submitted.
    #[inline]
    fn aria_required(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-required",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Multiple: crate::html::HtmlElement {
    ///Whether to allow multiple values
    #[inline]
    fn multiple<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "multiple",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaAtomic: crate::html::HtmlElement {
    ///Indicates whether assistive technologies will present all, or only parts of, the changed region based on the change notifications defined by the aria-relevant attribute.
    #[inline]
    fn aria_atomic(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-atomic",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Role: crate::html::HtmlElement {
    ///Describes the role(s) the current element plays in the context of the document.
    #[inline]
    fn role<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "role",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaPosInSet: crate::html::HtmlElement {
    ///Defines an element's number or position in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM. See related aria-setsize.
    #[inline]
    fn aria_pos_in_set(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-posinset",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait FormEncTypeTrait: crate::html::HtmlElement {
    ///Entry list encoding type to use for form submission
    #[inline]
    fn form_enctype(mut self: Box<Self>, value: FormEncodingType) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "formenctype",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaRowSpan: crate::html::HtmlElement {
    ///Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid. See related aria-rowindex and aria-colspan.
    #[inline]
    fn aria_row_span(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-rowspan",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaValueText: crate::html::HtmlElement {
    ///Defines the human readable text alternative of aria-valuenow for a range widget.
    #[inline]
    fn aria_value_text<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-valuetext",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Scope: crate::html::HtmlElement {
    ///Specifies which cells the header cell applies to
    #[inline]
    fn scope(mut self: Box<Self>, value: TableHeaderScope) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "scope",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaDescribedByElements: crate::html::HtmlElement {
    ///Identifies the element (or elements) that describes the object. See related aria-labelledby and aria-description.
    #[inline]
    fn aria_described_by_elements<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-describedby",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AbbrTrait: crate::html::HtmlElement {
    ///Alternative label to use for the header cell when referencing the cell in other contexts
    #[inline]
    fn abbr_trait<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "abbr",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Content: crate::html::HtmlElement {
    ///Value of the element
    #[inline]
    fn content<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "content",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Placeholder: crate::html::HtmlElement {
    ///User-visible label to be placed within the form control
    #[inline]
    fn placeholder<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "placeholder",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait CrossOriginTrait: crate::html::HtmlElement {
    ///How the element handles crossorigin requests
    #[inline]
    fn crossorigin(mut self: Box<Self>, value: CrossOrigin) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "crossorigin",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Selected: crate::html::HtmlElement {
    ///Whether the option is selected by default
    #[inline]
    fn selected(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "selected",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait DateTime: crate::html::HtmlElement {
    ///Date and (optionally) time of the change
    #[inline]
    fn date_time<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "datetime",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Rel: crate::html::HtmlElement {
    ///Relationship between the location in the document containing the hyperlink and the destination resource
    #[inline]
    fn rel<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "rel",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait LinkRel: crate::html::HtmlElement {
    ///Relationship between the location in the document containing the hyperlink and the destination resource
    #[inline]
    fn rel(mut self: Box<Self>, value: LinkType) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "rel",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Start: crate::html::HtmlElement {
    ///Starting value of the list
    #[inline]
    fn start<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "start",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Allowfullscreen: crate::html::HtmlElement {
    ///Whether to allow the iframe's contents to use requestFullscreen()
    #[inline]
    fn allowfullscreen<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "allowfullscreen",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Imagesrcset: crate::html::HtmlElement {
    ///Images to use in different situations, e.g., high-resolution displays, small monitors, etc. (for rel="preload")
    #[inline]
    fn imagesrcset<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "imagesrcset",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaOwnsElements: crate::html::HtmlElement {
    ///Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship between DOM elements where the DOM hierarchy cannot be used to represent the relationship. See related aria-controls.
    #[inline]
    fn aria_owns_elements<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-owns",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Srcdoc: crate::html::HtmlElement {
    ///A document to render in the iframe
    #[inline]
    fn srcdoc<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "srcdoc",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaValueMax: crate::html::HtmlElement {
    ///Defines the maximum allowed value for a range widget.
    #[inline]
    fn aria_value_max(mut self: Box<Self>, value: f64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-valuemax",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait List: crate::html::HtmlElement {
    ///List of autocomplete options
    #[inline]
    fn list<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "list",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait EncTypeTrait: crate::html::HtmlElement {
    ///Entry list encoding type to use for form submission
    #[inline]
    fn enctype(mut self: Box<Self>, value: FormEncodingType) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "enctype",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Reversed: crate::html::HtmlElement {
    ///Number the list backwards
    #[inline]
    fn reversed<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "reversed",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaColIndexText: crate::html::HtmlElement {
    ///Defines a human readable text alternative of aria-colindex. See related aria-rowindextext.
    #[inline]
    fn aria_col_index_text<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-colindextext",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaActiveDescendantElement: crate::html::HtmlElement {
    ///Identifies the currently active element when DOM focus is on a composite widget, combobox, textbox, group, or application.
    #[inline]
    fn aria_active_descendant_element<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-activedescendant",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Height: crate::html::HtmlElement {
    ///Vertical dimension
    #[inline]
    fn height(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "height",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaMultiSelectable: crate::html::HtmlElement {
    ///Indicates that the user can select more than one item from the current selectable descendants.
    #[inline]
    fn aria_multi_selectable(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-multiselectable",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Popovertarget: crate::html::HtmlElement {
    ///Targets a popover element to toggle, show, or hide
    #[inline]
    fn popovertarget<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "popovertarget",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Minlength: crate::html::HtmlElement {
    ///Minimum length of value
    #[inline]
    fn minlength<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "minlength",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaSetSize: crate::html::HtmlElement {
    ///Defines the number of items in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM. See related aria-posinset.
    #[inline]
    fn aria_set_size(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-setsize",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Src: crate::html::HtmlElement {
    ///Address of the resource
    #[inline]
    fn src<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "src",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Low: crate::html::HtmlElement {
    ///High limit of low range
    #[inline]
    fn low(mut self: Box<Self>, value: f64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "low",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Action: crate::html::HtmlElement {
    ///URL to use for form submission
    #[inline]
    fn action<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "action",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaSort: crate::html::HtmlElement {
    ///Indicates if items in a table or grid are sorted in ascending or descending order.
    #[inline]
    fn aria_sort<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-sort",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaDisabled: crate::html::HtmlElement {
    ///Indicates that the element is perceivable but disabled, so it is not editable or otherwise operable. See related aria-hidden and aria-readonly.
    #[inline]
    fn aria_disabled(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-disabled",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait AriaAutoComplete: crate::html::HtmlElement {
    ///Indicates whether inputting text could trigger display of one or more predictions of the user's intended value for a combobox, searchbox, or textbox and specifies how predictions would be presented if they were made.
    #[inline]
    fn aria_auto_complete<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-autocomplete",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Colspan: crate::html::HtmlElement {
    ///Number of columns that the cell is to span
    #[inline]
    fn colspan<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "colspan",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaLevel: crate::html::HtmlElement {
    ///Defines the hierarchical level of an element within a structure.
    #[inline]
    fn aria_level(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-level",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Width: crate::html::HtmlElement {
    ///Horizontal dimension
    #[inline]
    fn width(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "width",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaColCount: crate::html::HtmlElement {
    ///Defines the total number of columns in a table, grid, or treegrid. See related aria-colindex.
    #[inline]
    fn aria_col_count(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-colcount",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaPressed: crate::html::HtmlElement {
    ///Indicates the current "pressed" state of toggle buttons. See related aria-checked and aria-selected.
    #[inline]
    fn aria_pressed<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-pressed",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait CiteTrait: crate::html::HtmlElement {
    ///Link to the source of the quotation or more information about the edit
    #[inline]
    fn cite_trait<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "cite",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait LabelTrait: crate::html::HtmlElement {
    ///User-visible label
    #[inline]
    fn label_trait<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "label",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Open: crate::html::HtmlElement {
    ///Whether the details are visible
    #[inline]
    fn open(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut()
            .insert_attribute("open", crate::ast::AttributeValue::BooleanAttribute(value));
        self
    }
}
pub trait Href: crate::html::HtmlElement {
    ///Address of the hyperlink
    #[inline]
    fn href<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "href",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Readonly: crate::html::HtmlElement {
    ///Whether to allow the value to be edited by the user
    #[inline]
    fn readonly<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "readonly",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Loop: crate::html::HtmlElement {
    ///Whether to loop the media resource
    #[inline]
    fn loop_<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "loop",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Optimum: crate::html::HtmlElement {
    ///Optimum value in gauge
    #[inline]
    fn optimum(mut self: Box<Self>, value: f64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "optimum",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaCurrent: crate::html::HtmlElement {
    ///Indicates the element that represents the current item within a container or set of related elements.
    #[inline]
    fn aria_current<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-current",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaMultiLine: crate::html::HtmlElement {
    ///Indicates whether a text box accepts multiple lines of input or only a single line.
    #[inline]
    fn aria_multi_line(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-multiline",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Defer: crate::html::HtmlElement {
    ///Defer script execution
    #[inline]
    fn defer<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "defer",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaLabelledByElements: crate::html::HtmlElement {
    ///Identifies the element (or elements) that labels the current element. See related aria-label and aria-describedby.
    #[inline]
    fn aria_labelled_by_elements<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-labelledby",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaRowIndex: crate::html::HtmlElement {
    ///Defines an element's row index or position with respect to the total number of rows within a table, grid, or treegrid. See related aria-rowindextext, aria-rowcount, and aria-rowspan.
    #[inline]
    fn aria_row_index(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-rowindex",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AcceptCharset: crate::html::HtmlElement {
    ///Character encodings to use for form submission
    #[inline]
    fn accept_charset<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "accept-charset",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Kind: crate::html::HtmlElement {
    ///The type of text track
    #[inline]
    fn kind(mut self: Box<Self>, value: VideoKind) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "kind",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Poster: crate::html::HtmlElement {
    ///Poster frame to show prior to video playback
    #[inline]
    fn poster<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "poster",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Async: crate::html::HtmlElement {
    ///Execute script when available, without blocking while fetching
    #[inline]
    fn async_<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "async",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Max: crate::html::HtmlElement {
    ///Maximum value
    #[inline]
    fn max<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "max",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait FormTrait: crate::html::HtmlElement {
    ///Associates the element with a form element
    #[inline]
    fn form_trait<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "form",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait NoValidate: crate::html::HtmlElement {
    ///Bypass form control validation for form submission
    #[inline]
    fn no_validate(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "novalidate",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Imagesizes: crate::html::HtmlElement {
    ///Image sizes for different page layouts (for rel="preload")
    #[inline]
    fn imagesizes<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "imagesizes",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait PlaysInline: crate::html::HtmlElement {
    ///Encourage the user agent to display video content within the element's playback area
    #[inline]
    fn plays_inline(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "playsinline",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait WrapTrait: crate::html::HtmlElement {
    ///How the value of the form control is to be wrapped for form submission
    #[inline]
    fn wrap(mut self: Box<Self>, value: Wrap) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "wrap",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaInvalid: crate::html::HtmlElement {
    ///Indicates the entered value does not conform to the format expected by the application. See related aria-errormessage.
    #[inline]
    fn aria_invalid<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-invalid",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Dirname: crate::html::HtmlElement {
    ///Name of form control to use for sending the element's directionality in form submission
    #[inline]
    fn dirname<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "dirname",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Download: crate::html::HtmlElement {
    ///Whether to download the resource instead of navigating to it, and its filename if so
    #[inline]
    fn download<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "download",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaBusy: crate::html::HtmlElement {
    ///Indicates an element is being modified and that assistive technologies could wait until the modifications are complete before exposing them to the user.
    #[inline]
    fn aria_busy(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-busy",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait AriaColIndex: crate::html::HtmlElement {
    ///Defines an element's column index or position with respect to the total number of columns within a table, grid, or treegrid. See related aria-colindextext, aria-colcount, and aria-colspan.
    #[inline]
    fn aria_col_index(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-colindex",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Name: crate::html::HtmlElement {
    ///Name of the element to use for form submission and in the form.elements API
    #[inline]
    fn name<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "name",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait NameMetadata: crate::html::HtmlElement {
    ///Name of the element to use for form submission and in the form.elements API
    #[inline]
    fn name(mut self: Box<Self>, value: Metadata) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "name",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Required: crate::html::HtmlElement {
    ///Whether the control is required for form submission
    #[inline]
    fn required<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "required",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Controls: crate::html::HtmlElement {
    ///Show user agent controls
    #[inline]
    fn controls<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "controls",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaRowCount: crate::html::HtmlElement {
    ///Defines the total number of rows in a table, grid, or treegrid. See related aria-rowindex.
    #[inline]
    fn aria_row_count(mut self: Box<Self>, value: i64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-rowcount",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait Size: crate::html::HtmlElement {
    ///Size of the control
    #[inline]
    fn size<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "size",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Loading: crate::html::HtmlElement {
    ///Used when determining loading deferral
    #[inline]
    fn loading<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "loading",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaBrailleRoleDescription: crate::html::HtmlElement {
    ///Defines a human-readable, author-localized abbreviated description for the role of an element, which is intended to be converted into Braille. See related aria-roledescription.
    #[inline]
    fn aria_braille_role_description<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-brailleroledescription",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Maxlength: crate::html::HtmlElement {
    ///Maximum length of value
    #[inline]
    fn maxlength<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "maxlength",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait PreloadTrait: crate::html::HtmlElement {
    ///Hints how much buffering the media resource will likely need
    #[inline]
    fn preload(mut self: Box<Self>, value: Preload) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "preload",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaHasPopup: crate::html::HtmlElement {
    ///Indicates the availability and type of interactive popup element, such as menu or dialog, that can be triggered by an element.
    #[inline]
    fn aria_has_popup<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-haspopup",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Value: crate::html::HtmlElement {
    ///Value to be used for form submission
    #[inline]
    fn value<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "value",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Sizes: crate::html::HtmlElement {
    ///Image sizes for different page layouts
    #[inline]
    fn sizes<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "sizes",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaHidden: crate::html::HtmlElement {
    ///Indicates whether the element is exposed to an accessibility API. See related aria-disabled.
    #[inline]
    fn aria_hidden(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-hidden",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait AriaValueMin: crate::html::HtmlElement {
    ///Defines the minimum allowed value for a range widget.
    #[inline]
    fn aria_value_min(mut self: Box<Self>, value: f64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "aria-valuemin",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait AriaChecked: crate::html::HtmlElement {
    ///Indicates the current "checked" state of checkboxes, radio buttons, and other widgets. See related aria-pressed and aria-selected.
    #[inline]
    fn aria_checked<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-checked",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaLive: crate::html::HtmlElement {
    ///Indicates that an element will be updated, and describes the types of updates the user agents, assistive technologies, and user can expect from the live region.
    #[inline]
    fn aria_live<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-live",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait FormTarget: crate::html::HtmlElement {
    ///Navigable for form submission
    #[inline]
    fn form_target<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "formtarget",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaLabel: crate::html::HtmlElement {
    ///Defines a string value that labels the current element. See related aria-labelledby.
    #[inline]
    fn aria_label<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-label",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaErrorMessageElements: crate::html::HtmlElement {
    ///Identifies the element (or elements) that provides an error message for an object. See related aria-invalid and aria-describedby.
    #[inline]
    fn aria_error_message_elements<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-errormessage",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaRoleDescription: crate::html::HtmlElement {
    ///Defines a human-readable, author-localized description for the role of an element.
    #[inline]
    fn aria_role_description<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-roledescription",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Alt: crate::html::HtmlElement {
    ///Replacement text for use when images are not available
    #[inline]
    fn alt<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "alt",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Pattern: crate::html::HtmlElement {
    ///Pattern to be matched by the form control's value
    #[inline]
    fn pattern<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "pattern",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait For: crate::html::HtmlElement {
    ///Associate the label with form control
    #[inline]
    fn for_<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "for",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait FormNoValidate: crate::html::HtmlElement {
    ///Bypass form control validation for form submission
    #[inline]
    fn form_no_validate(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "formnovalidate",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait High: crate::html::HtmlElement {
    ///Low limit of high range
    #[inline]
    fn high(mut self: Box<Self>, value: f64) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "high",
            crate::ast::AttributeValue::KeyValuePair(value.to_string().into()),
        );
        self
    }
}
pub trait FormAction: crate::html::HtmlElement {
    ///URL to use for form submission
    #[inline]
    fn form_action<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "formaction",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Step: crate::html::HtmlElement {
    ///Granularity to be matched by the form control's value
    #[inline]
    fn step<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "step",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Default: crate::html::HtmlElement {
    ///Enable the track if no other text track is more suitable
    #[inline]
    fn default(mut self: Box<Self>, value: bool) -> Box<Self> {
        self.get_dom_element_mut().insert_attribute(
            "default",
            crate::ast::AttributeValue::BooleanAttribute(value),
        );
        self
    }
}
pub trait Allow: crate::html::HtmlElement {
    ///Permissions policy to be applied to the iframe's contents
    #[inline]
    fn allow<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "allow",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Media: crate::html::HtmlElement {
    ///Applicable media
    #[inline]
    fn media<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "media",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Muted: crate::html::HtmlElement {
    ///Whether to mute the media resource by default
    #[inline]
    fn muted<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "muted",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Popovertargetaction: crate::html::HtmlElement {
    ///Indicates whether a targeted popover element is to be toggled, shown, or hidden
    #[inline]
    fn popovertargetaction<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "popovertargetaction",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Method: crate::html::HtmlElement {
    ///Variant to use for form submission
    #[inline]
    fn method<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "method",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait AriaOrientation: crate::html::HtmlElement {
    ///Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    #[inline]
    fn aria_orientation<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "aria-orientation",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait Nomodule: crate::html::HtmlElement {
    ///Prevents execution in user agents that support module scripts
    #[inline]
    fn nomodule<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut().insert_attribute(
            "nomodule",
            crate::ast::AttributeValue::KeyValuePair(value.into()),
        );
        self
    }
}
pub trait As: crate::html::HtmlElement {
    ///Potential destination for a preload request (for rel="preload" and rel="modulepreload")
    #[inline]
    fn as_<T>(mut self: Box<Self>, value: T) -> Box<Self>
    where
        T: Into<::smartstring::alias::String>,
    {
        self.get_dom_element_mut()
            .insert_attribute("as", crate::ast::AttributeValue::KeyValuePair(value.into()));
        self
    }
}
