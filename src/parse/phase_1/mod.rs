mod block;
mod document;
mod traits;

use crate::parse::phase_1::{block::OpenBlock, document::OpenDocument};
use document::Document;
use std::ops::ControlFlow;

pub fn parse<'a>(text: &'a str) -> Document<'a> {
    let document = process_text(text);
    // TODO: extract link ref def.
    document.close()
}

fn process_text<'a>(text: &'a str) -> OpenDocument<'a> {
    let mut document = Document::open();
    let mut matched: Vec<&mut OpenBlock<'a>> = Vec::new();
    for line in text.split_inclusive("\n") {
        process_line(&mut matched, &mut document, line);
    }
    document
}

fn process_line<'a>(
    matched: &mut Vec<&mut OpenBlock<'a>>,
    document: &mut OpenDocument<'a>,
    line: &'a str,
) {
    matched.clear();
    let mut remaining = line;

    document.visit_current_branch(|block| match block.stage(remaining) {
        Ok(leftover) => {
            matched.push(block);
            remaining = leftover;
            ControlFlow::Continue(())
        }
        Err(_) => ControlFlow::Break(()),
    });

    // Handle special case for lazy continuation lines.
    let mut leaf = document.current_leaf();
    if let Some(leaf) = leaf {
        if leaf.is_paragraph() && leaf.stage(remaining).is_ok() {
            document.visit_current_branch(|block| block.commit());
            return;
        }
    }

    let mut root;
    let mut parent = None;
    while !remaining.is_empty() {
        let (leftover, block) = Block::open(remaining);
        remaining = leftover;
        if let Some(parent) = parent {
            parent.append_block(block);
        }
        parent = Some(&mut block);
    }

    match matched.last() {
        Some(node) => {
            // Ingests the staged input for all matched nodes.
            for node in matched {
                node.commit();
            }
            node.close_current_branch();
            node.append_branch(new_branch);
        }
        None => {
            document.close_current_branch();
            document.append_branch(new_branch);
        }
    }
}
