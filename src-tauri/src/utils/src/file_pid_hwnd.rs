use winsys::process::{hwnd::Hwnd, pid::Pid};

#[derive(Debug, Clone)]
pub struct FilesPid(pub Vec<FilePid>);

impl FilesPid {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, file_pid: FilePid) {
        if let Some(index) = self.0.iter().position(|x| x.file == file_pid.file) {
            self.0[index] = file_pid;
        } else {
            self.0.push(file_pid);
        }
    }

    pub fn extend(&mut self, files_pid: FilesPid) {
        files_pid.0.iter().for_each(|x| {
            self.insert(x.clone());
        });
    }
}

#[derive(Debug, Clone)]
pub struct FilePid {
    pub file: String,
    pub pid: Pid,
    pub hwnd: Hwnd,
}

impl FilePid {
    pub fn new(file: String, pid: Pid, hwnd: Hwnd) -> Self {
        Self { file, pid, hwnd }
    }
}
