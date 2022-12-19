mod components;
mod wkhtml;

use crate::components::GraphType::Bitmap;
use crate::components::IGraph;
use html_macro::html;
use virtual_node::View;
use virtual_node::VirtualNode;

#[cfg(test)]
mod tests {
    use super::*;
    use virtual_node::IterableNodes;
    use wkhtml_rs_render::{self, render_wkhtml};

    #[test]
    fn test_render() {
        let html = html! {
            <template>
                <h1>Hello!</h1>
                <IGraph decorate_points={false} />
                <br />
                This is only the beginning {1 + 2}
            </template>
        };
        println!("{}", render_wkhtml(&html).unwrap());
    }
}
