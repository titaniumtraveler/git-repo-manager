use git_repo_manager::template::Template;

fn main() {
    println!("{}", Template::new(b"${dir.data}/repo/${repo.name}.git"));
}
