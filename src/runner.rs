use settimeout::set_timeout;
use std::path::Path;

pub enum CommandType {
    Sh,
    Node,
    None,
}

#[macro_export]
macro_rules! command_from_string {
    ($ss: expr) => {
        match $ss {
            "sh" => $crate::runner::CommandType::Sh,
            "node" => $crate::runner::CommandType::Node,
            _ => $crate::runner::CommandType::None,
        }
    };
}

pub struct Runner<'a> {
    runner_command: CommandType,
    file_path: &'a Path,
}

impl<'a> Runner<'a> {
    pub fn new(runner_command: CommandType, file_path: &'a Path) -> Self {
        return Runner {
            runner_command,
            file_path,
        };
    }

    pub fn execute(self) -> () {
        match self.runner_command {
            CommandType::Sh => {
                let mut sh = std::process::Command::new("sh");
                sh.arg(self.file_path)
                    .spawn()
                    .expect("sh command failed to start");
            }
            CommandType::Node => {
                let mut node = std::process::Command::new("node");
                node.arg(self.file_path)
                    .spawn()
                    .expect("node command failed to start");
            }
            _ => {}
        }
    }

    pub async fn execute_after_timeout(self, timeout: chrono::Duration) {
        println!("It will be executed on the time you specified 😉");
        set_timeout(std::time::Duration::from_secs(
            timeout.num_seconds().to_string().parse::<u64>().unwrap(),
        ))
        .await;
        self.execute();
        println!("done 😎");
    }
}
