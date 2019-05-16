extern crate scratch_crate;

extern crate toml;

extern crate curl;
extern crate indicatif;
extern crate serde_json;

extern crate fs_extra;

mod scratch;

use crate::{
    scratch::target::PyGameTarget,
    scratch_crate::{
        Project,
        SaveOptions,
    },
};
use clap::{
    App,
    Arg,
    SubCommand,
};
use std::path::PathBuf;

fn main() {
    let matches = App::new("scratch-native")
        .subcommand(
            SubCommand::with_name("new")
                .arg(Arg::with_name("path").required(true)) //Remove for current dir?
                .arg(
                    Arg::with_name("code")
                        .takes_value(true)
                        .short("c")
                        .long("code")
                        .required(true),
                ),
        ) //temp
        .subcommand(
            SubCommand::with_name("info")
                .arg(Arg::with_name("code").required(true)) //Allow paths? Optional args?
                .arg(
                    Arg::with_name("type")
                        .takes_value(true)
                        .short("t")
                        .long("type"),
                ),
        )
        .subcommand(SubCommand::with_name("build").arg(Arg::with_name("path").required(true))) //Remove for current dir?
        .subcommand(SubCommand::with_name("old_main"))
        .get_matches();

    let mut api = scratch::api::Api::new();
    let mut client = scratch_crate::client::Client::new();

    match matches.subcommand() {
        ("new", Some(matches)) => {
            let path = PathBuf::from(matches.value_of("path").expect("No path specified"));
            let code = matches.value_of("code").expect("No Project Code Specified");

            println!("Downloading Project... ");
            let net_project = scratch_crate::NetworkProject::new(code.to_string());
            let data = client.get_data(&net_project).unwrap();
            let mut project: Project = data.into();
            project.name = Some(format!("scratch_{}", code));
            project.save(path.clone(), SaveOptions::new()).unwrap();
        }
        ("info", Some(matches)) => {
            let code = matches.value_of("code").expect("Code not specified");
            if let Some(data_type) = matches.value_of("type") {
                match data_type {
                    "all" => {
                        let mut project =
                            api.get_project(code).expect("Error getting project json");
                        println!("{:#?}", project);
                    }
                    "json" => {
                        let mut project = api
                            .get_project_json(code)
                            .expect("Error getting project json");
                        println!("{:#?}", project);
                    }
                    "stats" => {
                        let stats = api.get_stats(code).expect("Error getting project json");
                        println!("{:#?}", stats);
                    }
                    _ => {
                        println!("type {} is not valid", &data_type);
                    }
                }
            } else {
                let net_project = scratch_crate::NetworkProject::new(code.to_string());
                let stats = client.get_data(&net_project).unwrap();
                dbg!(stats);

                let mut project = api.get_stats(code).expect("Error getting project json");
                println!("{:#?}", project);
            }
        }
        ("build", Some(matches)) => {
            let path = PathBuf::from(matches.value_of("path").expect("No path specified"));
            let mut project: Project = Project::load(path.clone()).unwrap();
            let mut target = PyGameTarget::new();
            project.build(&mut target).unwrap();
            project.run(&mut target).unwrap();
        }
        ("old_main", Some(matches)) => {
            old_main();
        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}

//Hack to keep old code
fn old_main() {
    std::fs::remove_dir_all("projects/scratch_239742347/target/js");
    println!("Loading Project... ");
    let mut path = PathBuf::from("projects");
    path.push("scratch_239742347");
    let mut project = scratch::api::types::Project::from_path(path).expect("Error Loading Project");
    let mut target = scratch::api::targets::JsTarget::new();
    println!("Setting up target... ");
    project.init_target(&mut target).unwrap();
    println!("Building target... ");
    project.build_target(&mut target).unwrap();
    println!("Testing target... ");
    project.test_target(&mut target).unwrap();

    return;
    print!("Building Project... ");
    /*
    let work = scratch::build_project(&std::path::PathBuf::from("projects\\239742347"))
        .unwrap()
        .map(|_|{
            println!("Done");
        })
        .map_err(|err|{
            println!("Error: {:#?}", err);
        });

    rt::run(work);
    */
}
