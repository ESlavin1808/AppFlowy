pub use auto_exit_block::*;
pub use auto_format::*;
pub use default_insert::*;
pub use preserve_inline_style::*;
pub use reset_format_on_new_line::*;

mod auto_exit_block;
mod auto_format;
mod default_insert;
mod preserve_inline_style;
mod reset_format_on_new_line;

use crate::{client::extensions::InsertExt, core::Delta};

pub struct PreserveBlockStyleOnInsertExt {}
impl InsertExt for PreserveBlockStyleOnInsertExt {
    fn ext_name(&self) -> &str { "PreserveBlockStyleOnInsertExt" }

    fn apply(
        &self,
        _delta: &Delta,
        _replace_len: usize,
        _text: &str,
        _index: usize,
    ) -> Option<Delta> {
        None
    }
}

pub struct PreserveLineStyleOnSplitExt {}
impl InsertExt for PreserveLineStyleOnSplitExt {
    fn ext_name(&self) -> &str { "PreserveLineStyleOnSplitExt" }

    fn apply(
        &self,
        _delta: &Delta,
        _replace_len: usize,
        _text: &str,
        _index: usize,
    ) -> Option<Delta> {
        None
    }
}

pub struct InsertEmbedsExt {}
impl InsertExt for InsertEmbedsExt {
    fn ext_name(&self) -> &str { "InsertEmbedsExt" }

    fn apply(
        &self,
        _delta: &Delta,
        _replace_len: usize,
        _text: &str,
        _index: usize,
    ) -> Option<Delta> {
        None
    }
}

pub struct ForceNewlineForInsertsAroundEmbedExt {}
impl InsertExt for ForceNewlineForInsertsAroundEmbedExt {
    fn ext_name(&self) -> &str { "ForceNewlineForInsertsAroundEmbedExt" }

    fn apply(
        &self,
        _delta: &Delta,
        _replace_len: usize,
        _text: &str,
        _index: usize,
    ) -> Option<Delta> {
        None
    }
}
