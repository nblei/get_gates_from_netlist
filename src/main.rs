use std::collections::HashMap;
use std::path::PathBuf;
use sv_parser::{parse_sv, unwrap_node, Locate, RefNode};

use clap::{App, Arg};

fn main() {
    let matches = App::new("GateLister")
        .version("0.1")
        .author("Nathaniel Bleier <nbleier3@illinois.edu>")
        .about("Lists all of the gates in a netlist")
        .arg(Arg::new("NETLIST")
             .about("input netlist")
             .required(true)
             .index(1))
        .arg(Arg::new("count")
             .about("print the number of gates in netlist")
             .required(false)
             .short('c')
             .long("count")
        ).get_matches();

    let path = PathBuf::from(matches.value_of("NETLIST").unwrap());
    let gate_count = matches.is_present("count");
    gate_parser(path, gate_count);
}


fn gate_parser(path: PathBuf, count: bool) {
    let defines = HashMap::new();
    let includes: Vec<PathBuf> = vec![];
    let mut gate_count = 0;

    let result = parse_sv(&path, &defines, &includes, false, false);
    
    if let Ok((st, _)) = result {
        for node in &st {
            if let RefNode::ModuleInstantiation(x) = node {
                gate_count += 1;
                if !count {
                    let modid = unwrap_node!(x, ModuleIdentifier).unwrap();
                    let instid = unwrap_node!(x, InstanceIdentifier).unwrap();
                    let modid = get_identifier(modid).unwrap();
                    let instid = get_identifier(instid).unwrap();
                    let modid = st.get_str(&modid).unwrap();
                    let instid = st.get_str(&instid).unwrap();
                    println!("{}\t{}", modid, instid);
                }
            } else {
                // println!("Not a module!");
            }
        }
    }
    else {
        panic!("unable to parse netlist");
    }
    if count {
        println!("{}", gate_count);
    }
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    // unwrap_node! can take multiple types
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        Some(RefNode::EscapedIdentifier(x)) => {
            return Some(x.nodes.0);
        }
        _ => None,
    }
}
