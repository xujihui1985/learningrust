use std::{
    char::ToUppercase,
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
    rc::Rc, vec,
};

static mut NAME: &str = "Rust";

fn return_slice() -> &'static [u32] {
    let v = vec![1, 2, 3, 4, 5];
    &v
}

fn main() {
    unsafe {
        NAME = "World";
        println!("hello {NAME}!");
    }

    let path = &PathBuf::from("src/main.rs");
    let name = get_file_name(path);
    println!("file name: {}", name);

    let headers = HashMap::new();
    let v = headers.get_optional_header("name");

    let stragety = AdhocStrategy {
        task: "task".to_string(),
    };

    let executor = Executor::new(Box::new(stragety));
    executor.execute();
}

fn get_file_name(path: &Path) -> String {
    let file_name: FileName = path.into();
    file_name.into()
}

trait Headers {
    fn get_optional_header(&self, name: &str) -> Option<String>;
}

impl Headers for HashMap<String, String> {
    fn get_optional_header(&self, name: &str) -> Option<String> {
        self.get(name).map(ToOwned::to_owned)
    }
}

// use our own type to lay out the rules for the file name
struct FileName(String);

impl From<&Path> for FileName {
    fn from(path: &Path) -> Self {
        let file_name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown")
            .to_string();
        Self(file_name)
    }
}

impl From<FileName> for String {
    fn from(file_name: FileName) -> Self {
        file_name.0
    }
}

#[derive(Default)]
struct HandlerResult {
    status: u16,
    body: String,
}

struct HandlerEvent {
    body: Option<String>,
}

struct AdhocStrategy {
    task: String,
}

trait EventStrategy {
    fn file_name(&self) -> FileName;
    fn code(&self) -> Result<String, Box<dyn Error>>;
    fn health_check(&self) -> Result<HandlerResult, Box<dyn Error>>;
    fn payload() -> Option<String>
    where
        Self: Sized;
    fn parse_result()
    where
        Self: Sized;
}

impl TryFrom<Rc<HandlerEvent>> for AdhocStrategy {
    type Error = Box<dyn Error>;

    fn try_from(event: Rc<HandlerEvent>) -> Result<Self, Self::Error> {
        let task = match event.body {
            Some(ref body) => body,
            None => return Err("No body found".into()),
        };
        Ok(Self {
            task: task.to_owned(),
        })
    }
}

impl EventStrategy for AdhocStrategy {
    fn file_name(&self) -> FileName {
        FileName(self.task.clone())
    }

    fn code(&self) -> Result<String, Box<dyn Error>> {
        todo!()
    }

    fn payload() -> Option<String> {
        todo!()
    }

    fn health_check(&self) -> Result<HandlerResult, Box<dyn Error>> {
        println!("health check");
        Ok(HandlerResult::default())
    }

    fn parse_result() {
        todo!()
    }
}

struct Executor {
    strategy: Box<dyn EventStrategy>,
}

impl Executor {
    fn new(strategy: Box<dyn EventStrategy>) -> Self {
        Self { strategy }
    }

    fn execute(&self) -> Result<HandlerResult, Box<dyn Error>> {
        self.strategy.health_check()
    }
}
