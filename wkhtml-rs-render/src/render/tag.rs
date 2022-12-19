use seq_macro::seq;
use std::fmt::Write;
use virtual_node::{VElement, VirtualNode};

use crate::render::RenderError;

use super::ElementRenderer;
use super::Result;
use super::WkHtmlRender;

fn render_br(_element: &VElement, stream: &mut String) -> Result<()> {
    writeln!(stream, "")?;
    Ok(())
}

fn render_hn_base(element: &VElement, n: usize, stream: &mut String) -> Result<()> {
    let n_equal_signs = String::from_utf8(vec!['=' as u8; n]).unwrap();
    if element.attrs.contains_key("color") {
        writeln!(
            stream,
            r#"(% style="color:{}" %)"#,
            element.attrs.get("color").unwrap()
        )?;
    }
    writeln!(stream, "{} {} {}", n_equal_signs, "Hello", n_equal_signs)?;
    Ok(())
}

seq!(N in 1..=6 {
    fn render_h~N(element: &VElement, stream: &mut String) -> Result<()> {
        render_hn_base(element, N, stream)
    }
});

fn render_template(_element: &VElement, _stream: &mut String) -> Result<()> {
    for child in &_element.children {
        child.render(_stream)?;
    }
    Ok(())
}

pub(crate) fn get_tag_renderer(element: &VElement) -> Result<Box<ElementRenderer<()>>> {
    let renderer = match element.tag.as_str() {
        "br" => render_br,
        "h1" => render_h1,
        "h2" => render_h2,
        "h3" => render_h3,
        "h4" => render_h4,
        "h5" => render_h5,
        "h6" => render_h6,
        "template" => render_template,
        _ => Err(RenderError::UnsupportedTag(element.tag.clone()))?,
    };
    Ok(Box::new(renderer))
}
