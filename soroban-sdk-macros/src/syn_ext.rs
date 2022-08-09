use syn::{ImplItem, ImplItemMethod, ItemImpl, Visibility};

/// Gets methods from the implementation that have public visibility. For
/// methods that are inherently implemented this is methods that have a pub
/// visibility keyword. For methods that are implementing a trait the pub is
/// assumed and so all methods are returned.
pub fn impl_pub_methods(imp: &ItemImpl) -> impl Iterator<Item = &ImplItemMethod> {
    imp.items
        .iter()
        .filter_map(|i| match i {
            ImplItem::Method(m) => Some(m),
            _ => None,
        })
        .filter(|m| imp.trait_.is_some() || matches!(m.vis, Visibility::Public(_)))
}
