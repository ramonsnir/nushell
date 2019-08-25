use crate::commands::WholeStreamCommand;
use crate::errors::ShellError;
use crate::parser::CommandRegistry;
use crate::prelude::*;

pub struct Last;

impl WholeStreamCommand for Last {
    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        last(args, registry)
    }

    fn name(&self) -> &str {
        "last"
    }

    fn signature(&self) -> Signature {
        Signature::build("last").required("amount", SyntaxType::Literal)
    }
}

fn last(args: CommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let args = args.evaluate_once(registry)?;
    let (input, args) = args.parts();

    let amount = args.expect_nth(0)?.as_i64();
    let amount = match amount {
        Ok(o) => o as usize,
        Err(_) => {
            return Err(ShellError::labeled_error(
                "Value is not a number",
                "expected integer",
                args.expect_nth(0)?.span(),
            ))
        }
    };

    let output = input.values.collect::<Vec<_>>();

    let output = output.map(move |mut vec| {
        vec.reverse();
        let mut last_values = vec.into_iter().take(amount).collect::<Vec<_>>();
        last_values.reverse();
        last_values.into_iter().collect::<VecDeque<_>>()
    });

    Ok(output.flatten_stream().from_input_stream())
}
