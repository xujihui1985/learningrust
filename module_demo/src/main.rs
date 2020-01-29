mod filesystem;

mod another_mod {
    use crate::filesystem;

    pub fn do_sth() {
        filesystem::dentry::inode();
    }
}

fn main() {
    filesystem::dentry::inode();
    another_mod::do_sth();
}
