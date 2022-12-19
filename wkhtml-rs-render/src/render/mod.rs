mod error;
mod tag;

use std::fmt::Write;
use virtual_node::{VElement, VirtualNode};

use self::{error::RenderError, tag::get_tag_renderer};

pub type Result<T> = std::result::Result<T, RenderError>;
pub type ElementRenderer<T> = dyn Fn(&VElement, &mut String) -> Result<T>;

pub trait WkHtmlRender {
    fn render(&self, stream: &mut String) -> Result<()>;
}

impl WkHtmlRender for VirtualNode {
    fn render(&self, stream: &mut String) -> Result<()> {
        match self {
            VirtualNode::Element(element) => element.render(stream),
            VirtualNode::Text(text) => {
                write!(stream, "{}", text.text)?;
                Ok(())
            }
        }
    }
}

impl WkHtmlRender for VElement {
    fn render(&self, stream: &mut String) -> Result<()> {
        let renderer = get_tag_renderer(&self)?;
        renderer(&self, stream)?;
        Ok(())
    }
}
