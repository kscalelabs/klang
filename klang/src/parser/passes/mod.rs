use super::ast::{Command as AstCommand, Program as AstProgram};
use super::errors::ParseError;
use super::ir::{
    line::LineKind, text_part::PartKind, Command, Function, Line, Program, TextWithArgs,
};
use std::collections::HashMap;

fn get_function_signature(name: &TextWithArgs) -> (String, Vec<String>) {
    let mut signature = String::new();
    let mut params = Vec::new();
    let mut first = true;
    for part in &name.parts {
        if first {
            first = false;
        } else {
            signature.push(' ');
        }
        match &part.part_kind {
            Some(PartKind::Text(text)) => {
                signature.push_str(text);
            }
            Some(PartKind::FunctionArg(arg)) => {
                signature.push('[');
                signature.push_str(&arg.text);
                signature.push(']');
                params.push(arg.text.clone());
            }
            None => {}
        }
    }
    (signature, params)
}

fn match_function_call(
    call_name: &TextWithArgs,
    func_signature: &TextWithArgs,
    current_arg_map: &HashMap<String, String>,
) -> Option<HashMap<String, String>> {
    let mut args = HashMap::new();
    let mut call_iter = call_name.parts.iter();
    let mut sig_iter = func_signature.parts.iter();

    loop {
        match (call_iter.next(), sig_iter.next()) {
            (Some(call_part), Some(sig_part)) => {
                match (&call_part.part_kind, &sig_part.part_kind) {
                    (Some(PartKind::Text(call_text)), Some(PartKind::Text(sig_text))) => {
                        if call_text != sig_text {
                            return None;
                        }
                    }
                    (Some(PartKind::Text(call_text)), Some(PartKind::FunctionArg(arg))) => {
                        args.insert(arg.text.clone(), call_text.clone());
                    }
                    (
                        Some(PartKind::FunctionArg(call_arg)),
                        Some(PartKind::FunctionArg(sig_arg)),
                    ) => {
                        if let Some(value) = current_arg_map.get(&call_arg.text) {
                            args.insert(sig_arg.text.clone(), value.clone());
                        } else {
                            args.insert(sig_arg.text.clone(), call_arg.text.clone());
                        }
                    }
                    _ => {
                        return None;
                    }
                }
            }
            (None, None) => break,
            _ => return None,
        }
    }
    Some(args)
}

fn process_line_with_args(
    line: &Line,
    functions: &HashMap<String, (Function, Vec<String>)>,
    call_stack: &mut Vec<String>,
    arg_map: &HashMap<String, String>,
) -> Result<Vec<AstCommand>, ParseError> {
    if let Some(kind) = &line.line_kind {
        match kind {
            LineKind::Function { .. } => Ok(Vec::new()),
            LineKind::FunctionCall(func_call) => {
                if let Some(name) = &func_call.name {
                    let (call_signature, _) = get_function_signature(name);
                    for (func_sig, (func_def, _)) in functions {
                        if let Some(name_def) = &func_def.name {
                            if let Some(mut new_arg_map) =
                                match_function_call(name, name_def, arg_map)
                            {
                                // Merge parent scope arguments with new arguments
                                // New arguments take precedence over parent scope
                                for (key, value) in arg_map.iter() {
                                    new_arg_map
                                        .entry(key.clone())
                                        .or_insert_with(|| value.clone());
                                }

                                if call_stack.contains(func_sig) {
                                    return Err(ParseError::new(format!(
                                        "Recursive function call: {}",
                                        func_sig
                                    )));
                                }
                                call_stack.push(func_sig.clone());

                                // Create parent command with function name
                                let function_text = substitute_text_with_args(name, arg_map)?;

                                // Process child commands
                                let mut children = Vec::new();
                                for inner_line in &func_def.lines {
                                    let mut cmds = process_line_with_args(
                                        inner_line,
                                        functions,
                                        call_stack,
                                        &new_arg_map,
                                    )?;
                                    children.append(&mut cmds);
                                }

                                call_stack.pop();

                                // Return single command with children
                                return Ok(vec![AstCommand {
                                    text: function_text,
                                    children,
                                }]);
                            }
                        }
                    }
                    Err(ParseError::new(format!(
                        "Function not found: {{ {} }} Available functions: {{ {} }}",
                        call_signature,
                        functions
                            .keys()
                            .map(|k| k.as_str())
                            .collect::<Vec<&str>>()
                            .join(", ")
                    )))
                } else {
                    Err(ParseError::new("Function call without name".to_string()))
                }
            }
            LineKind::Command(cmd) => {
                let ast_command = process_command_with_args(cmd, arg_map)?;
                Ok(vec![ast_command])
            }
        }
    } else {
        Ok(Vec::new())
    }
}

fn process_command_with_args(
    cmd: &Command,
    arg_map: &HashMap<String, String>,
) -> Result<AstCommand, ParseError> {
    if let Some(text) = &cmd.text {
        let text = substitute_text_with_args(text, arg_map)?;
        Ok(AstCommand {
            text,
            children: Vec::new(),
        })
    } else {
        Err(ParseError::new("Command without text".to_string()))
    }
}

fn substitute_text_with_args(
    text_with_args: &TextWithArgs,
    arg_map: &HashMap<String, String>,
) -> Result<String, ParseError> {
    let mut result = String::new();
    let mut first = true;
    for part in &text_with_args.parts {
        if first {
            first = false;
        } else {
            result.push(' ');
        }
        match &part.part_kind {
            Some(PartKind::Text(text)) => {
                result.push_str(text);
            }
            Some(PartKind::FunctionArg(arg)) => {
                if let Some(value) = arg_map.get(&arg.text) {
                    result.push_str(value);
                } else {
                    result.push_str(&arg.text);
                }
            }
            None => {}
        }
    }
    Ok(result)
}

fn collect_functions(
    line: &Line,
    functions: &mut HashMap<String, (Function, Vec<String>)>,
) -> Result<(), ParseError> {
    if let Some(kind) = &line.line_kind {
        match kind {
            LineKind::Function(func) => {
                if let Some(name) = &func.name {
                    let (signature, params) = get_function_signature(name);
                    functions.insert(signature.clone(), (func.clone(), params));
                    for inner_line in &func.lines {
                        collect_functions(inner_line, functions)?;
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    } else {
        Ok(())
    }
}

pub(crate) fn ir_to_ast(ir_program: &Program) -> Result<AstProgram, ParseError> {
    let mut functions = HashMap::new();
    for line in &ir_program.lines {
        collect_functions(line, &mut functions)?;
    }

    let mut ast_program = AstProgram {
        commands: Vec::new(),
    };
    let mut call_stack = Vec::new();
    for line in &ir_program.lines {
        let mut commands =
            process_line_with_args(line, &functions, &mut call_stack, &HashMap::new())?;
        ast_program.commands.append(&mut commands);
    }
    Ok(ast_program)
}
