use std::path::PathBuf;

use libmdbx::{
    orm::{table_info, Database, Table},
    table,
};

table!(
    /// Test table.
    (TestNodes) Vec<u8> => Vec<u8>
);

pub fn new_db_with_path<T: Table>(path: PathBuf) -> Database {
    let tables = [table_info!(T)].into_iter().collect();
    Database::create(Some(path), &tables).expect("Failed creating db with path")
}

pub fn new_db<T: Table>() -> Database {
    let tables = [table_info!(T)].into_iter().collect();
    Database::create(None, &tables).expect("Failed to create temp DB")
}

pub fn open_db<T: Table>(path: &str) -> Database {
    let tables = [table_info!(T)].into_iter().collect();
    Database::open(path, &tables).expect("Failed to open DB")
}

#[macro_export]
macro_rules! pmt_node {
    (
        @( $trie:expr )
        branch { $( $choice:expr => $child_type:ident { $( $child_tokens:tt )* } ),+ $(,)? }
        $( offset $offset:expr )?
    ) => {
        $crate::trie::node::BranchNode::new({
            #[allow(unused_variables)]
            let offset = true $( ^ $offset )?;
            let mut choices = [$crate::trie::node_ref::NodeRef::default(); 16];
            $(
                let child_node = pmt_node! { @($trie)
                    $child_type { $( $child_tokens )* }
                    offset offset
                }.into();
                let child_node = $trie.db.insert_node(child_node).unwrap();
                choices[$choice as usize] = child_node;
            )*
            choices
        })
    };
    (
        @( $trie:expr )
        branch { $( $choice:expr => $child_type:ident { $( $child_tokens:tt )* } ),+ $(,)? }
        with_leaf { $path:expr => $value:expr }
        $( offset $offset:expr )?
    ) => {{
        $crate::trie::node::BranchNode::new_with_value({
            #[allow(unused_variables)]
            let offset = true $( ^ $offset )?;
            let mut choices = [$crate::trie::node_ref::NodeRef::default(); 16];
            $(
                choices[$choice as usize] = $trie.db.insert_node(
                    pmt_node! { @($trie)
                        $child_type { $( $child_tokens )* }
                        offset offset
                    }.into()
                ).unwrap();
            )*
            choices
        }, $path, $value)
    }};

    (
        @( $trie:expr )
        extension { $prefix:expr , $child_type:ident { $( $child_tokens:tt )* } }
        $( offset $offset:expr )?
    ) => {{
        #[allow(unused_variables)]
        let offset = false $( ^ $offset )?;
        let prefix = $crate::trie::nibble::NibbleVec::from_nibbles(
            $prefix
                .into_iter()
                .map(|x: u8| $crate::trie::nibble::Nibble::try_from(x).unwrap()),
            offset
        );

        let offset = offset  ^ (prefix.len() % 2 != 0);
        $crate::trie::node::ExtensionNode::new(
            prefix,
            {
                let child_node = pmt_node! { @($trie)
                    $child_type { $( $child_tokens )* }
                    offset offset
                }.into();
                $trie.db.insert_node(child_node).unwrap()
            }
        )
    }};

    (
        @( $trie:expr)
        leaf { $path:expr => $value:expr }
        $( offset $offset:expr )?
    ) => {
        {
            $crate::trie::node::LeafNode::new($path, $value)
        }
    };
}