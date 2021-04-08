extern crate gitlab_api as gitlab;

fn main() {
    let gl = gitlab::GitLab::new(&"gitlab.com", &"GITLAB_TOKEN_XXXXXXX").unwrap();

    // Get GitLab's version.
    let gitlab_version = gl.version().unwrap();
    println!("gitlab_version: {:?}", gitlab_version);


    // Low level methods

    // Get projects, owned by authenticated user and which are archived.
    let projects = gl.projects().owned().archived(true).list().unwrap();
    println!("projects: {:?}", projects);

    // Get groups owned by authenticated user.
    let owned_groups = gl.groups().owned().list().unwrap();
    println!("owned_groups: {:?}", owned_groups);

    // Get closed issues.
    let closed_issues = gl.issues().state(gitlab::issues::State::Closed).list().unwrap();
    println!("closed_issues: {:?}", closed_issues);


    // Higher level methods

    // Get a specific project
    let project = gl.get_project("nbigaouette1", "gitlab-api-rs").chain_err(|| "cannot get project")?;

    // Get a specific issue
    let issue = gl.get_issue("nbigaouette1", "gitlab-api-rs", 1).chain_err(|| "cannot get issue")?;

    // Get a specific merge request
    let merge_request = gl.get_merge_request("nbigaouette1", "gitlab-api-rs", 1).chain_err(|| "cannot get merge_request")?;
}