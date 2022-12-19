use html_macro::html;
use render::Result;
use render::WkHtmlRender;
use virtual_node::View;
use virtual_node::VirtualNode;

pub mod render;

pub fn render_wkhtml(vnode: &VirtualNode) -> Result<String> {
    let mut doc = String::new();
    vnode.render(&mut doc)?;
    Ok(doc)
}

struct Headers {
    color: String,
}

impl View for Headers {
    fn render(&self) -> VirtualNode {
        html! {
            <template>
                <h1 color={self.color.clone()}>Hello</h1>
                <h2 color={self.color.clone()}>Hello</h2>
                <h3 color={self.color.clone()}>Hello</h3>
            </template>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Headers;
    use html_macro::html;
    use virtual_node::View;
    use virtual_node::VirtualNode;

    use crate::render_wkhtml;

    #[test]
    fn test_render() {
        let doc = html! {
            <template>
                <Headers color="red".to_string()/>
                <Headers color="cyan".to_string()/>
            </template>
        };
        println!("{}", render_wkhtml(&doc).unwrap());
    }
}
