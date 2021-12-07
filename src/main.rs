use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, 
    UntaggedValue, Value, SyntaxShape
};

/// The plugin itself
struct Bg;

impl Plugin for Bg {
    /// Tell nushell once when the plugin is loaded
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("bg")
            .desc("Starts a process in the background")
            .required(
                "command",
                SyntaxShape::String,
                "The command to start as a background job"
            )
            .rest("arguments",
                SyntaxShape::String,
                "The arguments of the command"
            ))
    }

    /// Called at the begining of the stream of values (even if there are none) here
    /// are processed the arguments of the plugin
    fn sink(&mut self, call_info: CallInfo, _: Vec<Value>) {
        // Extract the command and arguments from the `call_info`
        let (cmd, args) = if let Some(values) = call_info.args.positional {
            // TODO: Maintain the Vec<String> and with some unsafe code extract the 
            // (String, Vec<String>) without reallocating
            let values: Vec<String> = values.into_iter()
                .map(|value| value.convert_to_string())
                .collect();
            (values[0].clone(), values[1..].to_owned())
        } else {
            return panic!("Expected command");
        };

        // Start the task as a background child process
        std::process::Command::new(cmd)
            .args(args)
            .spawn()
            .unwrap();
    }
}

fn main() {
    serve_plugin(&mut Bg);
}
