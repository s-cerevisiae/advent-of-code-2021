use petgraph::graphmap::UnGraphMap;

pub type Map<'n> = UnGraphMap<&'n str, ()>;
pub type Edge<'n> = (&'n str, &'n str);

pub fn is_big(node: &str) -> bool {
    node.find(char::is_uppercase).is_some()
}
