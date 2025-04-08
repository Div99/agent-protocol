use apc::apis;
use clap::Parser;
use tokio;
// use apc::models;

mod args;
use args::{APCli, Commands, TaskArtifactCommands, TaskCommands, TaskStepCommands};

#[tokio::main]
async fn main() {
    let args = APCli::parse();

    let configuration: apis::configuration::Configuration = apis::configuration::Configuration {
        base_path: args.url.clone(),
        user_agent: Some("Agent-CLI/v1".to_owned()),
        client: reqwest::Client::new(),
    };

    println!("{:?}", args);

    match args.commands {
        Commands::Task(task_args) => {
            match task_args.commands {
                TaskCommands::List(task_list_args) => {
                    if let Some(_) = task_args.id {
                        eprintln!("Error: Cannot list tasks and use an ID simultaneously. Please choose one operation.");
                        std::process::exit(1);
                    }
                    let tasks = apis::agent_api::list_agent_tasks(
                        &configuration,
                        Some(task_list_args.page),
                        Some(task_list_args.page_size),
                    )
                    .await
                    .unwrap();

                    println!("{:?}", tasks);

                    println!("Will be listing tasks with arguments: {:?}", task_list_args)
                }
                TaskCommands::Create(task_create_args) => {
                    if let Some(_) = task_args.id {
                        eprintln!("Error: Cannot create a task and use an ID simultaneously. Please choose one operation.");
                        std::process::exit(1);
                    }
                    // let task = apis::agent_api::create_agent_task(
                    //     &args.url,
                    //     task_create_args.input,
                    //     task_create_args.additional_input,
                    // )
                    // .unwrap();

                    // println!("{:?}", task);
                    println!("Will be creating task with input: {:?}", task_create_args);
                }
                TaskCommands::Step(task_step_args) => {
                    // ! TODO: This is a hacky way to do this. There has to be a better way.
                    if let Some(id) = task_args.id {
                        match task_step_args.commands {
                            TaskStepCommands::List(task_step_list_args) => {
                                // let steps = apis::agent_api::list_agent_task_steps(
                                //     &args.url,
                                //     &id,
                                //     task_step_list_args.page,
                                //     task_step_list_args.page_size,
                                // )
                                // .unwrap();

                                // println!("{:?}", steps);
                                println!(
                                    "Will be listing steps for task {} with arguments: {:?}",
                                    id, task_step_list_args
                                );
                            }
                            TaskStepCommands::Execute(task_step_execute_args) => {
                                // let step = apis::agent_api::execute_agent_task_step(
                                //     &args.url,
                                //     &id,
                                //     task_step_execute_args.input,
                                //     task_step_execute_args.additional_input,
                                // )
                                // .unwrap();

                                // println!("{:?}", step);
                                println!(
                                    "Will be executing step for task {} with input: {:?}",
                                    id, task_step_execute_args
                                );
                            }
                        }
                    } else {
                        eprintln!("Error: Cannot execute a step without a task ID. Please specify a task ID.");
                        std::process::exit(1);
                    }
                }
                TaskCommands::Artifact(task_artifact_args) => {
                    // ! TODO: Same as the other, this is a hacky way to do this.
                    if let Some(id) = task_args.id {
                        match task_artifact_args.commands {
                            TaskArtifactCommands::List(task_artifact_list_args) => {
                                // let artifacts = apis::agent_api::list_agent_task_artifacts(
                                //     &args.url,
                                //     &id,
                                //     task_artifact_list_args.page,
                                //     task_artifact_list_args.page_size,
                                // )
                                // .unwrap();

                                // println!("{:?}", artifacts);
                                println!(
                                    "Will be listing artifacts for task {} with arguments: {:?}",
                                    id, task_artifact_list_args
                                );
                            }
                            TaskArtifactCommands::Upload(task_artifact_upload_args) => {
                                // let artifact = apis::agent_api::upload_agent_task_artifact(
                                //     &args.url,
                                //     &id,
                                //     task_artifact_upload_args.input,
                                //     task_artifact_upload_args.additional_input,
                                // )
                                // .unwrap();

                                // println!("{:?}", artifact);
                                println!(
                                    "Will be uploading artifact for task {} with input: {:?}",
                                    id, task_artifact_upload_args
                                );
                            }
                            TaskArtifactCommands::Download(task_artifact_download_args) => {
                                // let artifact = apis::agent_api::download_agent_task_artifact(
                                //     &args.url,
                                //     &id,
                                //     task_artifact_download_args.output_file,
                                // )
                                // .unwrap();

                                // println!("{:?}", artifact);
                                println!(
                                    "Will be downloading artifact for task {} with arguments: {:?}",
                                    id, task_artifact_download_args
                                );
                            }
                        }
                    } else {
                        eprintln!("Error: Cannot upload or download an artifact without a task ID. Please specify a task ID.");
                        std::process::exit(1)
                    }
                }
            }
        }
    }
}
