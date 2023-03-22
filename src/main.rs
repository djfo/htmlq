extern crate html5ever;
extern crate kuchiki;

#[macro_use]
extern crate lazy_static;

mod config;
mod link;
mod pretty_print;

use config::get_config;
use kuchiki::ElementData;
use kuchiki::NodeDataRef;
use kuchiki::NodeRef;
use kuchiki::traits::*;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;
use std::str;
use url::Url;

fn select_attributes(node: &NodeRef, attributes: &[String], output: &mut dyn io::Write) {
    if let Some(as_element) = node.as_element() {
        for attr in attributes {
            if let Ok(elem_atts) = as_element.attributes.try_borrow() {
                if let Some(val) = elem_atts.get(attr.as_str()) {
                    output.write_all(format!("{}\n", val).as_ref()).unwrap();
                }
            }
        }
    }
}

fn serialize_text(node: &NodeRef, ignore_whitespace: bool) -> String {
    let mut result = String::new();
    for text_node in node.inclusive_descendants().text_nodes() {
        if ignore_whitespace && text_node.borrow().trim().is_empty() {
            continue;
        }

        result.push_str(&text_node.borrow());

        if ignore_whitespace {
            result.push('\n');
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config();

    let mut input: Box<dyn io::Read> = match config.input_path.as_ref() {
        None => Box::new(std::io::stdin()),
        Some(f) => Box::new(File::open(f).unwrap()),
    };

    let stdout = std::io::stdout();
    let mut output: Box<dyn io::Write> = match config.output_path.as_ref() {
        None => Box::new(stdout.lock()),
        Some(f) => Box::new(File::create(f).unwrap()),
    };

    let document = kuchiki::parse_html().from_utf8().read_from(&mut input)?;

    let base = if config.detect_base {
        link::detect_base(&document)
    } else if let Some(b) = config.base {
        let u = Url::parse(&b);

        if let Err(e) = u {
            eprintln!("Failed to parse the provided base URL: {}", e);
            process::exit(1)
        };

        Some(u.unwrap())
    } else {
        None
    };

    if let Some(base) = base {
        link::rewrite_relative_urls(&document, &base);
    }

    if config.remove_nodes.is_some() {
        for selector in config.remove_nodes.unwrap() {
            let node_refs: Vec<NodeDataRef<ElementData>> = document
                .select(&selector)
                .expect("Failed to parse CSS selector")
                .collect();

            // For some reason I can't just iterate over this directly
            // I need to collect each node and then detach them separately
            for node_ref in node_refs {
                node_ref.as_node().detach();
            }
        }
    }

    for css_match in document
        .select(&config.selector)
        .expect("Failed to parse CSS selector")
    {
        let node = css_match.as_node();

        match config.output_format {
            config::OutputFormat::Attributes(ref attributes) =>
                select_attributes(node, attributes, &mut output),
            config::OutputFormat::TextOnly => {
                let content = serialize_text(node, config.ignore_whitespace);
                output.write_all(format!("{}\n", content).as_ref())?;
            }
            config::OutputFormat::PrettyPrint => {
                let content = pretty_print::pretty_print(node);
                output.write_all(content.as_ref())?;
            }
            config::OutputFormat::Pass => {
                let mut content: Vec<u8> = Vec::new();
                node.serialize(&mut content)?;
                output.write_all(format!("{}\n", str::from_utf8(&content)?).as_ref())?;
            }
        }
    }

    Ok(())
}
