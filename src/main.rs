use std::io;
use aup3conv::project::Project;
use std::env;


fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();

    let project = Project::new(&argv[1]);
    project.list_labels();
    Ok(())
}
