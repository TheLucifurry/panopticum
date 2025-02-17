use std::path::PathBuf;

use panopticum_schemas::PathNodes;

pub fn path_nodes_to_path_buf(path_nodes: &PathNodes) -> PathBuf {
  let mut path = PathBuf::new();
  for node in path_nodes {
      path.push(node.as_value())
  }
  path
}