use std::{iter::zip, process::Command, sync::Arc, fmt::Result, string::ParseError};

#[derive(Debug, PartialEq)]
pub struct ToolbxContainer {
    id: String,
    name: String,
    created: String,
    status: String,
    image: String,
}

impl ToolbxContainer {
    pub fn get_toolboxes() -> Vec<ToolbxContainer> {
        let output = run_cmd_toolbx_list_containers();
        println!("{}", output);
        parse_cmd_list_containers(output.as_str())
    }

    pub fn stop(self) {
        todo!{}
    }
    pub fn start(self) {
        todo!{}
    }
}

pub fn run_cmd_toolbx_list_containers() -> String {
    let output = Command::new("toolbox")
        .arg("list")
        .arg("--containers")
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_cmd_list_containers() {
    // This requires toolbx to be installed
    let toolbox_cmd_container_header =
        "CONTAINER ID  CONTAINER NAME     CREATED       STATUS      IMAGE NAME";
    assert!(run_cmd_toolbx_list_containers().starts_with(toolbox_cmd_container_header));
}

fn tokenize_line_list_containers(line: &str) -> Vec<String> {
    let mut tokens = Vec::<String>::new();
    let mut current_token = Vec::<char>::new();

    let mut whitespace_section = false;

    let mut iter = line.chars().peekable();
    while let Some(&c) = iter.peek() {
        match (whitespace_section, c) {
            (false, ' ') => {
                iter.next();
                if Some(' ') == iter.peek().map(|x| x.clone()) {
                    whitespace_section = true;
                } else {
                    current_token.push(c);
                }
            }
            (true, ' ') => {
                iter.next();
            }
            (true, c) => {
                whitespace_section = false;
                tokens.push(current_token.into_iter().collect());
                current_token = Vec::new();
                current_token.push(c);
                iter.next();
            }
            (false, c) => {
                current_token.push(c);
                iter.next();
            }
        }
    }
    tokens.push(current_token.into_iter().collect());

    tokens
}

#[test]
fn test_tokenize_line_list_containers() {
    let toolbox_cmd_container_header = 
        "ae05203091ab  rust               4 months ago  running     registry.fedoraproject.org/fedora-toolbox:35";

    let target = vec![
        "ae05203091ab",
        "rust",
        "4 months ago",
        "running",
        "registry.fedoraproject.org/fedora-toolbox:35",
    ];
    let result = tokenize_line_list_containers(toolbox_cmd_container_header);
    assert_eq!(target, result);
}

fn parse_line_list_containers(line: &str) -> ToolbxContainer {
    let tokens = tokenize_line_list_containers(line);
    if tokens.len() != 5 {
        panic! {"Expected 5 tokens found {} in {:?}",  tokens.len(), tokens};
    }
    ToolbxContainer {
        id: tokens[0].clone(),
        name: tokens[1].clone(),
        created: tokens[2].clone(),
        status: tokens[3].clone(),
        image: tokens[4].clone(),
    }
}

#[test]
fn test_parse_line_list_containers() {
    let toolbox_cmd_container_header = 
        "ae05203091ab  rust               4 months ago  running     registry.fedoraproject.org/fedora-toolbox:35";

    let target = ToolbxContainer {
        id: "ae05203091ab".to_string(),
        name: "rust".to_string(),
        created: "4 months ago".to_string(),
        status: "running".to_string(),
        image: "registry.fedoraproject.org/fedora-toolbox:35".to_string(),
    };
    let result = parse_line_list_containers(toolbox_cmd_container_header);
    assert_eq!(target, result);
}

fn parse_cmd_list_containers(output: &str) -> Vec<ToolbxContainer> {
    let lines = output.trim().split("\n").skip(1);
    println!("{:?}", lines);
    lines.map(parse_line_list_containers).collect()
}

#[test]
fn test_parse_cmd_list_containers() {
    // This requires toolbx to be installed
    let toolbox_cmd_container_header = concat!(
        "CONTAINER ID  CONTAINER NAME     CREATED       STATUS      IMAGE NAME\n",
        "cee1002b5f0b  fedora-toolbox-35  2 months ago  exited      registry.fedoraproject.org/fedora-toolbox:35\n",
        "9b611313bf65  latex              4 months ago  configured  registry.fedoraproject.org/fedora-toolbox:35\n",
        "ae05203091ab  rust               4 months ago  running     registry.fedoraproject.org/fedora-toolbox:35\n"
    );

    let desired_result = vec![
        ToolbxContainer {
            id: "cee1002b5f0b".to_string(),
            name: "fedora-toolbox-35".to_string(),
            created: "2 months ago".to_string(),
            status: "exited".to_string(),
            image: "registry.fedoraproject.org/fedora-toolbox:35".to_string(),
        },
        ToolbxContainer {
            id: "9b611313bf65".to_string(),
            name: "latex".to_string(),
            created: "4 months ago".to_string(),
            status: "configured".to_string(),
            image: "registry.fedoraproject.org/fedora-toolbox:35".to_string(),
        },
        ToolbxContainer {
            id: "ae05203091ab".to_string(),
            name: "rust".to_string(),
            created: "4 months ago".to_string(),
            status: "running".to_string(),
            image: "registry.fedoraproject.org/fedora-toolbox:35".to_string(),
        },
    ];

    for (result, desired) in zip(
        parse_cmd_list_containers(toolbox_cmd_container_header).iter(),
        desired_result.iter(),
    ) {
        assert_eq!(result, desired)
    }
}
