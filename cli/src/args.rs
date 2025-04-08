use clap::{Args, Parser, Subcommand};

/// Program to call agents
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct APCli {
    /// URL of the Agent
    #[arg(short, long, default_value_t = String::from("http://localhost:8000"))]
    pub url: String,

    /// Command to run
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "task")]
    Task(TaskArgs),
}

#[derive(Args, Debug)]
pub struct TaskArgs {
    /// Task ID
    #[arg(name = "id")]
    pub id: Option<String>,

    #[command(subcommand)]
    pub commands: TaskCommands,
}

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    /// List Tasks
    #[command(name = "list")]
    List(TaskListArgs),

    /// Create a New Task
    #[command(name = "create")]
    Create(TaskCreateArgs),

    /// Execute or List Steps for a Task
    /// Example: apc task <task-id> step list
    #[command(name = "step")]
    Step(TaskStepArgs),

    /// Upload or Download Artifacts
    #[command(name = "artifact")]
    Artifact(TaskArtifactArgs),
}

#[derive(Args, Debug)]
pub struct TaskListArgs {
    /// Page Number
    /// Default: 1
    #[arg(short, long, default_value_t = 1)]
    pub page: i32,

    /// Page Size
    /// Default: 10
    #[arg(short = 's', long, default_value_t = 10)]
    pub page_size: i32,
}

#[derive(Args, Debug)]
pub struct TaskCreateArgs {
    /// Task Input
    /// Example: --input 'Write Washington to a file'
    #[arg(name = "input", short = 'i', long)]
    pub input: String,

    /// Additional Input
    /// Default: None
    /// Example: --additional-input '{"foo": "bar"}'
    #[arg(name = "additional_input", short = 'a', long)]
    pub additional_input: Option<String>,
}

#[derive(Args, Debug)]
pub struct TaskStepArgs {
    /// Step ID
    #[arg(name = "id")]
    pub id: Option<String>,

    #[command(subcommand)]
    pub commands: TaskStepCommands,
}

#[derive(Subcommand, Debug)]
pub enum TaskStepCommands {
    /// List Steps for this Task
    #[command(name = "list")]
    List(TaskStepListArgs),

    /// Execute a New Step for thiagent-protocols Task
    #[command(name = "execute")]
    Execute(TaskStepExecuteArgs),
}

#[derive(Args, Debug)]
pub struct TaskStepListArgs {
    /// Page Number
    /// Default: 1
    #[arg(short, long, default_value_t = 1)]
    pub page: u8,

    /// Page Size
    /// Default: 10
    #[arg(short = 's', long, default_value_t = 10)]
    pub page_size: u8,
}

#[derive(Args, Debug)]
pub struct TaskStepExecuteArgs {
    /// Step Input
    /// Example: --input 'Write Washington to a file'
    #[arg(name = "input", short = 'i', long)]
    pub input: String,

    /// Additional Input
    /// Default: None
    /// Example: --additional-input '{"foo": "bar"}'
    #[arg(name = "additional_input", short = 'a', long)]
    pub additional_input: Option<String>,
}

#[derive(Args, Debug)]
pub struct TaskArtifactArgs {
    /// Artifact ID
    #[arg(name = "id")]
    pub id: Option<String>,

    #[command(subcommand)]
    pub commands: TaskArtifactCommands,
}

#[derive(Subcommand, Debug)]
pub enum TaskArtifactCommands {
    /// List Artifacts for this Task
    #[command(name = "list")]
    List(TaskArtifactListArgs),

    /// Upload a New Artifact for this Task
    #[command(name = "upload")]
    Upload(TaskArtifactUploadArgs),

    /// Download an Artifact for this Task
    #[command(name = "download")]
    Download(TaskArtifactDownloadArgs),
}

#[derive(Args, Debug)]
pub struct TaskArtifactListArgs {
    /// Page Number
    /// Default: 1
    #[arg(short, long, default_value_t = 1)]
    pub page: u8,

    /// Page Size
    /// Default: 10
    #[arg(short = 's', long, default_value_t = 10)]
    pub page_size: u8,
}

#[derive(Args, Debug)]
pub struct TaskArtifactUploadArgs {
    /// Artifact Input
    /// Example: --input 'Write Washington to a file'
    #[arg(name = "input", short = 'i', long)]
    pub input: String,

    /// Additional Input
    /// Default: None
    /// Example: --additional-input '{"foo": "bar"}'
    #[arg(name = "additional_input", short = 'a', long)]
    pub additional_input: Option<String>,
}

#[derive(Args, Debug)]
pub struct TaskArtifactDownloadArgs {
    /// Output File
    /// Default: None
    /// Example: --output-file 'washington.txt'
    #[arg(name = "output_file", short = 'o', long)]
    pub output_file: Option<String>,
}
