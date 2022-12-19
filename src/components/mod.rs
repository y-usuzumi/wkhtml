use html_macro::html;
use virtual_node::View;
use virtual_node::VirtualNode;

pub enum GraphType {
    Bitmap,
}

pub struct IGraph {
    pub decorate_points: bool,
}

impl View for IGraph {
    fn render(&self) -> virtual_node::VirtualNode {
        html! {
            {format!("{{{{igraph decorate_points={}}}}}", self.decorate_points)}
        }
    }
}
