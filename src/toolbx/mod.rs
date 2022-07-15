use std::{fmt::Display, iter::zip, process::Command, str::FromStr, string::ParseError, sync::Arc};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq)]
pub enum ToolbxError {
    ParseStatusError(String),
    JSONSerializationError(String),
    CommandExecutionError(String),
    CommandUnsuccessfulError(String),
}

impl std::error::Error for ToolbxError {}

impl Display for ToolbxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolbxError::ParseStatusError(parse_error) => write!(f, "{}", parse_error),
            ToolbxError::CommandExecutionError(command_exec_error) => {
                write!(f, "{}", command_exec_error)
            }
            ToolbxError::JSONSerializationError(msg) => {
                write!(f, "{}", msg)
            }
            ToolbxError::CommandUnsuccessfulError(command_unsuc_error) => {
                write!(f, "{}", command_unsuc_error)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ToolbxStatus {
    Running,
    Configured,
    Created,
    Exited,
}

impl Default for ToolbxStatus {
    fn default() -> Self {
        ToolbxStatus::Configured
    }
}

impl FromStr for ToolbxStatus {
    type Err = ToolbxError;

    fn from_str(s: &str) -> Result<ToolbxStatus, ToolbxError> {
        match s {
            "running" => Ok(ToolbxStatus::Running),
            "configured" => Ok(ToolbxStatus::Configured),
            "created" => Ok(ToolbxStatus::Created),
            "exited" => Ok(ToolbxStatus::Exited),
            s => Err(ToolbxError::ParseStatusError(format!(
                "'{}' is not a valid toolbx status.",
                s
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct ToolbxContainer {
    pub id: String,
    pub name: String,
    pub created: String,
    pub status: ToolbxStatus,
    pub image: String,
}

pub type PodmanInspectArray = Vec<PodmanInspectInfo>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodmanInspectInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "State")]
    pub state: PodManInspectState,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ImageName")]
    pub image_name: String,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodManInspectState {
    #[serde(rename = "Status")]
    pub status: String,
}

impl ToolbxContainer {
    pub fn get_toolboxes() -> Vec<ToolbxContainer> {
        let output = run_cmd_toolbx_list_containers();
        println!("{}", output);
        parse_cmd_list_containers(output.as_str())
    }

    fn parse_status(output : &str) -> Result<PodmanInspectInfo, ToolbxError> {
        let result : Result<PodmanInspectArray, _> = serde_json::from_str(output);
        match result {
            Ok(inspect_vec) => {
                match inspect_vec.first() {
                    Some(info) => {
                        Ok(info.clone())
                    }
                    None => {
                        Err(ToolbxError::JSONSerializationError("Inspect command returned empty vecotr.".to_string()))
                    }
                }
            }
            Err(e) => {
                Err(ToolbxError::JSONSerializationError(e.to_string()))
            }
        }
        
    }

    pub fn update_status(&mut self) -> Result<(), ToolbxError>{
        let output = Command::new("podman")
        .arg("container")
        .arg("inspect")
        .arg(self.name.clone())
        .output()
        .expect("Failed to execute command");

        let output = String::from_utf8_lossy(&output.stdout).to_string();
        let inspect_result = ToolbxContainer::parse_status(output.as_str())?;
        self.status = ToolbxStatus::from_str(inspect_result.state.status.as_str())?;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), ToolbxError> {
        let output = Command::new("podman")
            .arg("stop")
            .arg(self.name.clone())
            .output();

        if output.is_err() {
            return Err(ToolbxError::CommandExecutionError(
                output.unwrap_err().to_string(),
            ));
        }
        let output = output.unwrap();

        // Success: Output { status: ExitStatus(unix_wait_status(0)), stdout: "tbx_name\n", stderr: "" }
        //Fail:
        // Output {
        //     status: ExitStatus(unix_wait_status(32000)),
        //     stdout: "",
        //     stderr: "Error: no container with name or ID \"tbx_name\" found: no such container\n"
        // }

        if output.status.code() == Some(0) {
            self.status = ToolbxStatus::Exited;
            Ok(())
        } else {
            Err(ToolbxError::CommandUnsuccessfulError(
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ))
        }
    }

    pub fn start(&mut self) -> Result<(), ToolbxError> {
        let output = Command::new("podman")
            .arg("start")
            .arg(self.name.clone())
            .output();

        if output.is_err() {
            return Err(ToolbxError::CommandExecutionError(
                output.unwrap_err().to_string(),
            ));
        }
        let output = output.unwrap();

        // Success: status: Output { ExitStatus(unix_wait_status(0)), stdout: "tbx_name\n", stderr: "" }
        // Fail: status:
        // Output {
        //     status: ExitStatus(unix_wait_status(32000)),
        //     stdout: "",
        //     stderr: "Error: no container with name or ID \"tbx_name\" found: no such container\n"
        // }

        if output.status.code() == Some(0) {
            self.status = ToolbxStatus::Running;
            Ok(())
        } else {
            Err(ToolbxError::CommandUnsuccessfulError(
                String::from_utf8_lossy(&output.stderr).into_owned(),
            ))
        }
    }
}

#[test]
fn test_start_1non_existing_containter() {
    // TODO: create container that exists based on simple image
    //       run command
    //       delete container
    //let tbx = ToolbxContainer{created: "".to_string(), id: "".to_string(), name: "latex".to_string(), image: "".to_string(), status: ToolbxStatus::Exited};

    //tbx.stop();
}

#[test]
fn test_inspect_parsing() {
    let podman_inspect = concat!(
        "[{",
        "\"Id\": \"ae05203091ab4cdf047a9aeba6af8a7bed8105f7f59d09a35d2b64c837ecac0d\",",
        "\"Created\": \"2021-12-10T20:51:43.140418098+01:00\",",
        "\"State\": {",
               "\"Status\": \"running\"",
        "},",
        "\"Image\": \"ab8bc106d4a710a7a27c538762864610467b3559f80b413d30e0a1bfcfe272a5\",",
        "\"ImageName\": \"registry.fedoraproject.org/fedora-toolbox:35\",",
        "\"Name\": \"rust\"",
     "}]"
    );
    let inspect_info = ToolbxContainer::parse_status(podman_inspect).unwrap();
    assert_eq!("running", inspect_info.state.status);
}

#[test]
fn test_start_non_existing_containter() {
    let mut tbx = ToolbxContainer {
        created: "".to_string(),
        id: "".to_string(),
        name: "zy2lM6BdZoTnKHaVPkUJ".to_string(),
        image: "".to_string(),
        status: ToolbxStatus::Exited,
    };

    assert_eq!(Ok(()), tbx.start());
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
#[ignore]
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
    let toolbox_cmd_container_header = "ae05203091ab  rust               4 months ago  \
        running     registry.fedoraproject.org/fedora-toolbox:35";

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

fn parse_line_list_containers(line: &str) -> Result<ToolbxContainer, ToolbxError> {
    let tokens = tokenize_line_list_containers(line);
    if tokens.len() != 5 {
        panic! {"Expected 5 tokens found {} in {:?}",  tokens.len(), tokens};
    }
    Ok(ToolbxContainer {
        id: tokens[0].clone(),
        name: tokens[1].clone(),
        created: tokens[2].clone(),
        status: ToolbxStatus::from_str(&tokens[3])?,
        image: tokens[4].clone(),
    })
}

#[test]
fn test_parse_line_list_containers() {
    let toolbox_cmd_container_header = "ae05203091ab  rust               4 months ago  \
        running     registry.fedoraproject.org/fedora-toolbox:35";

    let target = ToolbxContainer {
        id: "ae05203091ab".to_string(),
        name: "rust".to_string(),
        created: "4 months ago".to_string(),
        status: ToolbxStatus::Running,
        image: "registry.fedoraproject.org/fedora-toolbox:35".to_string(),
    };
    let result = parse_line_list_containers(toolbox_cmd_container_header);
    assert_eq!(target, result.unwrap());
}

fn parse_cmd_list_containers(output: &str) -> Vec<ToolbxContainer> {
    let lines = output.trim().split("\n").skip(1);
    println!("{:?}", lines);
    lines.map(parse_line_list_containers).flatten().collect()
}

#[test]
fn test_parse_cmd_list_containers() {
    // This requires toolbx to be installed
    let toolbox_cmd_container_header = concat!(
        "CONTAINER ID  CONTAINER NAME     CREATED       STATUS      IMAGE NAME\n",
        "cee1002b5f0b  fedora-toolbox-35  2 months ago  exited      registry.fedoraproject.org/fedora-toolbox:35\n",
        "9b611313bf65  latex              4 months ago  configured  registry.fedoraproject.org/fedora-toolbox:35\n",
        "1235203091ab  website            4 months ago  created     registry.fedoraproject.org/fedora-toolbox:35\n",
        "ae05203091ab  rust               4 months ago  running     registry.fedoraproject.org/fedora-toolbox:35\n"
    );

    let desired_result = vec![
        ToolbxContainer {
            id: "cee1002b5f0b".to_string(),
            name: "fedora-toolbox-35".to_string(),
            created: "2 months ago".to_string(),
            status: ToolbxStatus::Exited,
            image: "registry.fedoraproject.org/fedora-toolbox:35".to_string(),
        },
        ToolbxContainer {
            id: "9b611313bf65".to_string(),
            name: "latex".to_string(),
            created: "4 months ago".to_string(),
            status: ToolbxStatus::Configured,
            image: "registry.fedoraproject.org/fedora-toolbox:35".to_string(),
        },
        ToolbxContainer {
            id: "1235203091ab".to_string(),
            name: "website".to_string(),
            created: "4 months ago".to_string(),
            status: ToolbxStatus::Created,
            image: "registry.fedoraproject.org/fedora-toolbox:35".to_string(),
        },
        ToolbxContainer {
            id: "ae05203091ab".to_string(),
            name: "rust".to_string(),
            created: "4 months ago".to_string(),
            status: ToolbxStatus::Running,
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
