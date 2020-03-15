use std::collections::HashMap;
use basin2_protocol::network::CommandNode;

pub trait McCommand {
    fn register(&self, root: &mut CommandNode);

    fn execute(&self, arguments: HashMap<String, String>);
}

pub fn register_commands(root: &mut CommandNode) {
    
}