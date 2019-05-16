use scratch::{
    client::Client,
    scratch3::NetworkProject,
    target::PyGameTarget,
    Project,
};
use std::path::PathBuf;

#[test]
pub fn common() {
    let path = PathBuf::from("projects/scratch_309320008");
    let project: Project = Project::load(path.clone()).unwrap();
    let mut target = PyGameTarget::new();
    project.build(&mut target).unwrap();
    project.run(&mut target).unwrap();
}

#[test]
pub fn save_scratch_3() {
    let path = PathBuf::from("projects");
    let code = "309320008";
    let mut client = Client::new();
    let net_project = NetworkProject::new(String::from(code));
    let project_data = client.get_data_3(&net_project).unwrap();
    //dbg!(project);
    let mut project: Project = project_data.into();
    project.name = Some(format!("scratch_{}", code));
    project
        .save(path.clone(), scratch::SaveOptions::new())
        .unwrap();
}
