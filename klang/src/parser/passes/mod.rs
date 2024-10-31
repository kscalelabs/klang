use super::ast::{Command as AstCommand, Program as AstProgram};
use super::errors::ParseError;
use super::ir::line::LineKind;
use super::ir::{Command, Function, Line, Program};
use std::collections::HashMap;

pub(crate) fn ir_to_ast(ir_program: &Program) -> Result<AstProgram, ParseError> {
    let mut functions = HashMap::new();
    for command in &ir_program.commands {
        let line = Line {
            line_kind: Some(LineKind::Command(command.clone())),
        };
        collect_functions(&line, &mut functions)?;
    }

    let mut ast_program = AstProgram {
        commands: Vec::new(),
    };
    let mut call_stack = Vec::new();
    for command in &ir_program.commands {
        let line = Line {
            line_kind: Some(LineKind::Command(command.clone())),
        };
        let mut commands = process_line(&line, &functions, &mut call_stack)?;
        ast_program.commands.append(&mut commands);
    }
    Ok(ast_program)
}

fn collect_functions(
    line: &Line,
    functions: &mut HashMap<String, Function>,
) -> Result<(), ParseError> {
    if let Some(kind) = &line.line_kind {
        match kind {
            LineKind::Function(func) => {
                functions.insert(func.name.clone(), func.clone());
                for inner_line in &func.lines {
                    collect_functions(inner_line, functions)?; // Note: pass inner_line directly
                }
                Ok(())
            }
            _ => Ok(()),
        }
    } else {
        Ok(())
    }
}

fn process_line(
    line: &Line,
    functions: &HashMap<String, Function>,
    call_stack: &mut Vec<String>,
) -> Result<Vec<AstCommand>, ParseError> {
    if let Some(kind) = &line.line_kind {
        match kind {
            LineKind::Function { .. } => Ok(Vec::new()),
            LineKind::FunctionCall(func_call) => {
                let name = &func_call.name;
                if call_stack.contains(name) {
                    panic!("Recursive function call detected: {}", name);
                }
                if let Some(func) = functions.get(name) {
                    call_stack.push(name.clone());
                    let mut commands = Vec::new();
                    for inner_line in &func.lines {
                        let mut cmds = process_line(inner_line, functions, call_stack)?;
                        commands.append(&mut cmds);
                    }
                    call_stack.pop();
                    Ok(commands)
                } else {
                    Ok(Vec::new())
                }
            }
            LineKind::Command(cmd) => {
                let ast_command = process_command(cmd)?;
                Ok(vec![ast_command])
            }
        }
    } else {
        Ok(Vec::new())
    }
}

fn process_command(cmd: &Command) -> Result<AstCommand, ParseError> {
    let mut children = Vec::new();
    for child in &cmd.children {
        let child_cmd = process_command(child)?;
        children.push(child_cmd);
    }
    Ok(AstCommand {
        text: cmd.text.clone(),
        children,
    })
}
