/*! Handles formatting the AST into the final output code
 */

use crate::ast::elements;
use crate::ast::elements::ElementInfo;
use crate::ast::parents;
use crate::formatting;
use crate::Ast;
use crate::Compiler;

/// The main function to set the output string from the compiler
///
/// Dynamically pushes and pops from a stack of root element children
/// Outputting
pub fn set_output(compiler: &mut Compiler) {
    compiler.ast.log(format!("output::set_output {:?}", ""));

    replace_any_unknown_types(&mut compiler.ast);

    set_output_append(&mut compiler.ast, "fn main() {\r\n");

    // we re-use the ast.parents from the parser, using it's length only, as a simple way to define the ongoing indent level.
    // The actual contents don't matter, so we reset it with two zeros for good measure to define the first indent level under the main function for the first line of code.
    // Thereafter we pop or push the last AST element since thats what parents::indent::indent does - but the element doesn't matter, just the ast.parents length
    // So it will usually contain duplicates like this, [ 0, 0, 29, 29, 29] (5 elements minus 2 indicating 3 levels of indent?)
    compiler.ast.parents = vec![0, 0];

    // We create a separate stack as a list of the children of root, i.e. the top level items to output
    // we go down them and dynamically push/pop their children to the stack
    // if we have added, and encounter, a 0, that indicates an "outdent marker" and we outdent ast.parents
    // otherwise some elements will manually indent ast.parents as we go

    compiler.ast.output_stack = compiler.ast.elements[0].1.clone();

    while compiler.ast.output_stack.len() > 0 {
        let current_el_index = compiler.ast.output_stack[0];
        let current_el = compiler.ast.elements[current_el_index].clone();
        let current_el_is_an_outdent_marker = current_el_index == 0;
        let children = current_el.1;

        // remove first/current item from stack
        compiler.ast.output_stack = parents::vec_remove_head(&compiler.ast.output_stack);

        if current_el_is_an_outdent_marker {
            compiler.ast.output_stack = output_end_of_element_and_outdent(compiler);
        } else {
            // push current element to output

            set_premain_output_for_element(&mut compiler.ast, current_el_index);
            set_output_for_element_open(&mut compiler.ast, current_el_index);

            // Render children if any, except of certain elements where the children are rendered by the parent
            /*
            let should_render_children =
                !is_skippable_due_to_parent(&mut compiler.ast, current_el_index);
            */
            // Render children if any, except of certain elements where the children are rendered separately
            let should_render_children = match current_el.0 {
                ElementInfo::InbuiltFunctionCall(_, _, _) => false,
                ElementInfo::Struct(_, _, _) => false,
                _ => children.len() > 0,
            };
            if should_render_children {
                compiler.ast.output_stack =
                    indent_and_add_children(compiler, current_el_index, children);
            }
        }
    }
    parents::outdent::outdent(compiler);
    set_output_append(&mut compiler.ast, "}\r\n");
    compiler.ast.output = format!("{}{}", compiler.ast.premain_output, compiler.ast.output);
}

/// indent from current parent and list children to output next
fn indent_and_add_children(
    compiler: &mut Compiler,
    current_el_index: usize,
    children: Vec<usize>,
) -> Vec<usize> {
    compiler
        .ast
        .log(format!("output::indent_and_add_children {:?}", ""));
    // add the following to stack in reverse order so they are then handled immediately,
    // and in correct order when popped off the stack in the next while loops
    let mut new_stack = compiler.ast.output_stack.clone();
    // prepend with current item end tag indicator - so we know to close it after the outdent
    new_stack.splice(0..0, vec![current_el_index]);
    // prepend with 0 (marker for outdent)
    new_stack.splice(0..0, vec![0]);
    // prepend with children
    new_stack.splice(0..0, children);
    // and increase indent
    parents::indent::indent(&mut compiler.ast);
    new_stack.clone()
}

/// outdent from list of children, back to parents next sibling
fn output_end_of_element_and_outdent(compiler: &mut Compiler) -> Vec<usize> {
    compiler.ast.log(format!(
        "output::output_end_of_element_and_outdent {:?}",
        ""
    ));
    parents::outdent::outdent(compiler);
    // push current end tag to output
    let end_tag = compiler.ast.output_stack[0];

    set_output_for_element_close(&mut compiler.ast, end_tag);
    // removed the outdent marker earlier, now remove the end tag indicator
    parents::vec_remove_head(&compiler.ast.output_stack)
    // if the end_tag was the end of a func_def we don't want to display the trailing semicolon
    // since it needs to be treated as the return statement, so remove it if there is one
}

/// Goes through the Ast several times to fill in all the Undefined types
pub fn replace_any_unknown_types(ast: &mut Ast) {
    ast.log(format!("output::replace_any_unknown_types {:?}", ""));
    for _i in 0..10 {
        let depths = get_depths_vec(&mut ast.clone());
        let depths_flattened = get_depths_flattened(&depths);
        for el_index in depths_flattened {
            ast.elements[el_index].0 =
                elements::get_updated_elementinfo_with_infered_type(ast, el_index);
        }
    }
}

/// Collects a vec of all Ast children in order so that the deepest types are inferred first, so these can inform the inference of their parents types
///
/// Ordered from deepest block in the 'tree' to highest
/// (ordered top to bottom for blocks at same level)
/// and reverse order within each block
pub fn get_depths_vec(ast: &mut Ast) -> Vec<Vec<usize>> {
    ast.log(format!("output::get_depths_vec {:?}", ""));
    let mut tracked_parents: Vec<usize> = vec![0];
    let mut children: Vec<usize> = ast.elements[0].1.clone();
    let mut depths: Vec<Vec<usize>> = vec![children];
    loop {
        //println!("{:?}", &tracked_parents);
        let mut next_level = vec![];
        let current_level = depths.last().unwrap().clone();
        for el_ref in current_level {
            let el = &ast.elements[el_ref];
            children = el.1.iter().cloned().rev().collect();
            next_level = vec![]
                .iter()
                .chain(&next_level)
                .chain(&children)
                .cloned()
                .collect();
            tracked_parents.push(el_ref);
        }
        if next_level.len() > 0 {
            depths.push(next_level);
        } else {
            break;
        }
        //println!("{:?}", &tracked_parents);
    }
    depths
}

/// Flattens the depths Vec, from bottom (deepest) to top
///
/// This is so that it can be used to traverse elements in the correct order
/// to allow correcting the types from the deepest elements first
/// since higher levels may rely on type of deeper elements.
/// e.g. a higher level "+" fn with type "i64|f64" will need to be disambiguated
/// to either i64 or f64 based on the type of it's 2 child args
/// so the two child args are fixed first (if unknown)
/// then "+" fn can be determined safely
pub fn get_depths_flattened(depths: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut output = vec![];
    for i in (0..depths.len()).rev() {
        let level = &depths[i];
        output = vec![].iter().chain(&output).chain(level).cloned().collect();
    }
    output
}

/// Gets the output string for an element based on its index - but for the code before the main function
fn get_premain_output_for_element_index(
    ast: &mut Ast,
    element_index: usize,
    request_skip_if_element_is_handled_by_parent: bool,
) -> String {
    ast.log(format!(
        "output::get_output_for_element_index {:?}",
        element_index
    ));
    let element = ast.elements[element_index].clone();
    let children = element.1;
    let empty_string = "".to_string();

    //skip children for certain parents who already parsed them
    if request_skip_if_element_is_handled_by_parent
        && is_skippable_due_to_parent(ast, element_index)
    {
        return "".to_string();
    }
    if let ElementInfo::Struct(name, _, _) = element.0 {
        get_premain_output_for_struct(ast, name, children)
    } else {
        "".to_string()
    }
}

/// Gets the output string for an element based on its index
fn get_output_for_element_index(
    ast: &mut Ast,
    element_index: usize,
    request_skip_if_element_is_handled_by_parent: bool,
) -> String {
    ast.log(format!(
        "output::get_output_for_element_index {:?}",
        element_index
    ));
    let element = ast.elements[element_index].clone();
    let children = element.1;
    let empty_string = "".to_string();

    //skip children for certain parents who already parsed them
    if request_skip_if_element_is_handled_by_parent
        && is_skippable_due_to_parent(ast, element_index)
    {
        return "".to_string();
    }

    match element.0 {
        ElementInfo::Root => empty_string,
        ElementInfo::CommentSingleLine(comment_string) => comment_string,
        ElementInfo::Int(val) => val,
        ElementInfo::Float(val) => val,
        ElementInfo::String(val) => format!("{}.to_string()", val),
        ElementInfo::Bool(val) => format!("{}.to_string()", val),
        ElementInfo::Arg(name, _scope, _argmodifier, _returntype) => name,
        ElementInfo::Struct(name, _, _) => {
            get_output_for_struct(ast, name, children, element_index)
        }
        ElementInfo::Constant(name, _) => name,
        ElementInfo::ConstantRef(name, _, _reference) => name,
        ElementInfo::Assignment => get_output_for_assignment(ast, children),
        ElementInfo::InbuiltFunctionDef(name, _, _, _, _, _) => {
            format!("fn {}() ->{{ /* stuff */ }}", name)
        }
        ElementInfo::List(returntype) => get_output_for_list(ast, children, returntype),
        ElementInfo::InbuiltFunctionCall(name, _, _) => {
            get_output_for_inbuiltfncall(ast, name, children)
        }
        ElementInfo::FunctionDefWIP => empty_string,
        ElementInfo::FunctionDef(name, argnames, argtypes, returntype) => {
            let empty_arg_modifiers = argnames.iter().map(|_s| String::new()).collect();
            let args = formatting::get_formatted_argname_argtype_pairs(
                &argnames,
                &argtypes,
                &empty_arg_modifiers,
            );
            format!("fn {}({}) -> {} {{\r\n", name, args, returntype)
        }
        ElementInfo::FunctionCall(name, _) => get_output_for_functioncall(ast, name, children),
        ElementInfo::Parens => get_output_for_parens(ast, children),
        ElementInfo::LoopForRangeWIP => empty_string,
        ElementInfo::LoopForRange(name, from, to) => {
            format!("For {} in {}..{} {{\r\n", name, from, to)
        }
        ElementInfo::Eol => format!("\r\n"),
        ElementInfo::Seol => format!(";\r\n"),
        ElementInfo::Indent => parents::get_indent(ast),
        ElementInfo::Type(name) => format!("{}", name),
        ElementInfo::Unused => empty_string,
        ElementInfo::Println => get_output_for_println(ast, children),
        ElementInfo::If(returntype) => get_output_for_if(ast, children, returntype),
    }
}

/// Defines which Elements' children should be skipped from being output directly,
/// as the children are handled by the parent Element's output
fn is_skippable_due_to_parent(ast: &mut Ast, element_index: usize) -> bool {
    let parent =
        parents::get_current_parent_element_from_element_children_search(&ast, element_index);
    ast.log(format!(
        "output::is_skippable_due_to_parent {} {:?}",
        element_index, parent
    ));
    match parent {
        Some((ElementInfo::Assignment, _)) => true,
        Some((ElementInfo::FunctionCall(_, _), _)) => true,
        Some((ElementInfo::Println, _)) => true,
        Some((ElementInfo::List(_), _)) => true,
        Some((ElementInfo::If(_), _)) => true,
        Some((ElementInfo::Struct(_, _, _), _)) => true,
        // explicitly listing other types rather than using _ to not overlook new types in future.
        Some((ElementInfo::Root, _)) => false,
        Some((ElementInfo::CommentSingleLine(_), _)) => false,
        Some((ElementInfo::Int(_), _)) => false,
        Some((ElementInfo::Float(_), _)) => false,
        Some((ElementInfo::String(_), _)) => false,
        Some((ElementInfo::Bool(_), _)) => false,
        Some((ElementInfo::Arg(_, _, _, _), _)) => false,
        Some((ElementInfo::Constant(_, _), _)) => false,
        Some((ElementInfo::ConstantRef(_, _, _), _)) => false,
        Some((ElementInfo::InbuiltFunctionDef(_, _, _, _, _, _), _)) => false,
        Some((ElementInfo::InbuiltFunctionCall(_, _, _), _)) => false,
        Some((ElementInfo::FunctionDefWIP, _)) => false,
        Some((ElementInfo::FunctionDef(_, _, _, _), _)) => false,
        Some((ElementInfo::LoopForRangeWIP, _)) => false,
        Some((ElementInfo::LoopForRange(_, _, _), _)) => false,
        Some((ElementInfo::Parens, _)) => false,
        Some((ElementInfo::Type(_), _)) => false,
        Some((ElementInfo::Eol, _)) => false,
        Some((ElementInfo::Seol, _)) => false,
        Some((ElementInfo::Indent, _)) => false,
        Some((ElementInfo::Unused, _)) => false,
        None => false,
    }
}

/// Pre-main Output for Struct
fn get_premain_output_for_struct(ast: &mut Ast, name: String, children: Vec<usize>) -> String {
    ast.log(format!("output::get_premain_output_for_struct {:?}", name));
    /*
    Should output something like this ...

    #[derive(Clone, Debug)]
    pub struct Newstruct {
        pub firstname: String,
        pub surname: String,
        pub age: i64,
    }

    impl Newstruct {
        pub fn new(
            firstname: String,
            surname: String,
            age: i64,
            ) -> Newstruct {
            Newstruct {
                firstname,
                surname,
                age,
            }
        }
    }

    ... two linebreaks before fn main() ...
    */

    // a Structs children should all be Assignments
    // each Assignment should have one child Constant
    let mut struct_pub_keys_types = "".to_string();
    let mut struct_keys_types = "".to_string();
    let mut struct_keys = "".to_string();
    for i in 0..children.len() as usize {
        let assignment_ref = ast.elements[children[i]].1[0];
        if let ElementInfo::Constant(key, a_type) = &ast.elements[assignment_ref].0 {
            struct_pub_keys_types = format!(
                "{}    pub {}: {},\r\n",
                &struct_pub_keys_types, &key, &a_type
            );
            struct_keys_types = format!("{}        {}: {},\r\n", &struct_keys_types, &key, &a_type);
            struct_keys = format!("{}            {},\r\n", &struct_keys, &key);
        }
    }

    let derive = "#[derive(Clone, Debug)]\r\n".to_string();
    let a_struct = format!(
        "pub struct {} {{\r\n{}}}\r\n\r\n",
        name, struct_pub_keys_types
    );
    let new_fn = format!(
        "    pub fn new(\r\n{}) -> {} {{\r\n        {} {{\r\n{}        }}\r\n    }}",
        struct_keys_types, name, name, struct_keys
    );
    format!(
        "{}{}impl Newstruct {{\r\n{}\r\n}}\r\n\r\n",
        derive, a_struct, new_fn
    )
}

/// Output for Struct
fn get_output_for_struct(
    ast: &mut Ast,
    name: String,
    children: Vec<usize>,
    element_index: usize,
) -> String {
    ast.log(format!("output::get_output_for_struct {:?}", name));
    // a Structs children should all be Assignments
    // each Assignment should have one child Constant
    let mut constants = vec![];
    for i in 0..children.len() as usize {
        let constant = ast.elements[children[i]].1[0];
        constants.push(constant);
    }

    let mut args_output = "".to_string();

    // each Constant should have one child Value or Expression that will be used as an argument to the fn new(arg1, arg2)
    for i in 0..constants.len() as usize {
        let arg_ref = ast.elements[constants[i]].1[0];
        let arg = get_output_for_element_index(ast, arg_ref, false);
        let no_first_comma = if i == 0 {
            "".to_string()
        } else {
            ", ".to_string()
        };
        args_output = format!("{}{}{}", &args_output, &no_first_comma, &arg);
    }

    // remove children so they aren't output later
    //ast.elements[element_index].1 = vec![];

    format!("{}::new({})", name, args_output)
}

/// Output for Assignment
fn get_output_for_assignment(ast: &mut Ast, children: Vec<usize>) -> String {
    ast.log(format!("output::get_output_for_assignment {:?}", ""));
    let mut returntype = "Undefined".to_string();
    if children.len() < 1 {
        format!(
            "// let ?: ? = ? OUTPUT ERROR: Can't get constant for this assignment from : {:?}",
            children
        )
    } else {
        let constant_index = children[0];
        let constant_output = get_output_for_element_index(ast, constant_index, false);
        let constant = &ast.elements[constant_index];
        match &constant.0 {
            ElementInfo::Constant(_, r) => {
                returntype = r.clone();
            }
            _ => (),
        }
        let mut mut_if_assigning_to_struct = "".to_string();
        let first_child_ref = children[0];
        if let ElementInfo::Constant(_, _) = ast.elements[first_child_ref].0 {
            let constant_first_child_ref = ast.elements[first_child_ref].1[0];
            if let ElementInfo::Struct(_, _, _) = ast.elements[constant_first_child_ref].0 {
                mut_if_assigning_to_struct = "mut ".to_string();
            }
        }
        format!(
            "let {}{}: {} = ",
            mut_if_assigning_to_struct, constant_output, returntype
        )
    }
}

/// Output for List
fn get_output_for_list(ast: &mut Ast, children: Vec<usize>, returntype: String) -> String {
    ast.log(format!("output::get_output_for_list {:?}", ""));
    if children.len() > 0 {
        let mut output = "vec![ ".to_string();
        for i in 0..children.len() {
            let child_ref = children[i];
            let child_output = get_output_for_element_index(ast, child_ref, false);
            let no_first_comma = if i == 0 {
                "".to_string()
            } else {
                ", ".to_string()
            };
            output = format!("{}{}{}", &output, &no_first_comma, &child_output);
        }
        format!("{} ]", &output)
    } else {
        let mut vec_type = returntype.clone();
        if returntype.len() > 5 && returntype[..3] == "Vec".to_string() {
            vec_type = returntype[4..returntype.len() - 1].to_string();
        }
        format!("Vec::<{}>::new()", vec_type)
    }
}

/// Output for InbuiltFnCall
fn get_output_for_inbuiltfncall(ast: &mut Ast, name: String, children: Vec<usize>) -> String {
    ast.log(format!("output::get_output_for_inbuiltfncall {:?}", ""));
    if let Some(def) = elements::get_inbuilt_function_by_name(ast, &name) {
        match def.clone() {
            ElementInfo::InbuiltFunctionDef(_, argnames, _, _, _, format) => {
                let mut output = format;
                for i in 0..argnames.len() {
                    let arg_var_num = format!("arg~{}", i + 1);
                    if i >= children.len() {
                        dbg!("output error", &name, &children, i);
                    } else {
                        let arg_value_el_ref = children[i];
                        let arg_output = get_output_for_element_index(ast, arg_value_el_ref, true);
                        output = output.replace(&arg_var_num, &arg_output);
                    }
                }
                if children.len() > 0 && children.len() == (argnames.len() + 1) {
                    let last_child = elements::get_last_element(ast);
                    match &last_child.0 {
                        ElementInfo::Seol => {
                            output = format!("{};\r\n", output);
                            ()
                        }
                        _ => (),
                    }
                }
                return output;
            }
            _ => return "".to_string(),
        }
    }
    "".to_string()
}

/// Output for FunctionCall
fn get_output_for_functioncall(ast: &mut Ast, name: String, arguments: Vec<usize>) -> String {
    ast.log(format!("output::get_output_for_functioncall {:?}", ""));
    let empty_string = "".to_string();
    let mut args = empty_string.clone();
    for i in 0..arguments.len() {
        let arg_el_ref = arguments[i];
        //let arg_el = ast.elements[arg_el_ref];
        let arg = get_output_for_element_index(ast, arg_el_ref, false);
        let mut borrow = empty_string.clone();
        if let Some(fndef_ref) = elements::get_function_index_by_name(ast, &name) {
            let fndef = &ast.elements[fndef_ref];
            match &fndef.0 {
                ElementInfo::FunctionDef(_, _, argtypes, _) => {
                    if argtypes.len() == arguments.len() {
                        if argtypes[i].contains("&dyn Fn") {
                            borrow = "&".to_string();
                        }
                    }
                }
                _ => (),
            }
        }

        let comma = if i == arguments.len() - 1 {
            empty_string.clone()
        } else {
            ", ".to_string()
        };
        args = format!("{}{}{}{}", args, borrow, arg, comma);
    }
    format!("{}({})", name, args)
}

/// Output for Parens
fn get_output_for_parens(ast: &mut Ast, children: Vec<usize>) -> String {
    ast.log(format!("output::get_output_for_parens {:?}", ""));
    let mut output = "".to_string();
    for i in 0..children.len() {
        let child_ref = children[i];
        let child = get_output_for_element_index(ast, child_ref, false);
        output = format!("{}{}", output, child);
    }
    format!("({})", output)
}

/// Output for Println
fn get_output_for_println(ast: &mut Ast, children: Vec<usize>) -> String {
    ast.log(format!("output::get_output_for_println {:?}", ""));
    let mut output = "".to_string();
    for i in 0..children.len() {
        let child_ref = children[i];
        let child = get_output_for_element_index(ast, child_ref, false);
        output = format!("{}{}", output, child);
    }
    format!("println!(\"{{}}\", {})", output)
}

/// Output for If statement
fn get_output_for_if(ast: &mut Ast, children: Vec<usize>, returntype: String) -> String {
    ast.log(format!("output::get_output_for_if {:?}", ""));
    let mut output = "".to_string();
    if children.len() < 3 {
        dbg!("output error", &returntype, &children);
    } else {
        let child1_output = get_output_for_element_index(ast, children[0], false);
        output = format!("{}if {} {{", output, child1_output);
        let child2_output = get_output_for_element_index(ast, children[1], false);
        output = format!(
            "{}\r\n{}{}\r\n{}}} else {{",
            output,
            " ".repeat(4 * (ast.parents.len())),
            child2_output,
            " ".repeat(4 * (ast.parents.len() - 1)),
        );
        let child3_output = get_output_for_element_index(ast, children[2], false);
        output = format!(
            "{}\r\n{}{}\r\n{}}}",
            output,
            " ".repeat(4 * (ast.parents.len())),
            child3_output,
            " ".repeat(4 * (ast.parents.len() - 1)),
        );
    }
    //parents::outdent::outdent(&mut ast);

    return output;
}

/// Append the current element's formatted string to the output string - but for "premain", i.e. the output prepended before the main function
fn set_premain_output_for_element(ast: &mut Ast, el_index: usize) {
    ast.log(format!(
        "output::set_output_for_element_premain {:?}",
        el_index
    ));
    let element_string = get_premain_output_for_element_index(ast, el_index, true);
    set_premain_output_append(ast, &element_string);
}

/// Append the current element's formatted string to the output string
fn set_output_for_element_open(ast: &mut Ast, el_index: usize) {
    ast.log(format!(
        "output::set_output_for_element_open {:?}",
        el_index
    ));
    let element_string = get_output_for_element_index(ast, el_index, true);
    set_output_append(ast, &element_string);
}

/// Append an indented closing bracket for FunctionDef to the output string, or nothing
fn set_output_for_element_close(ast: &mut Ast, el_index: usize) {
    ast.log(format!("output::set_output_for_element_close {:?}", ""));
    if el_index < ast.elements.len() {
        let element = &ast.elements[el_index];
        let element_string = match element.0 {
            ElementInfo::FunctionDef(_, _, _, _) => {
                format!("\r\n{}}}\r\n", parents::get_indent(ast))
            }
            ElementInfo::LoopForRange(_, _, _) => {
                format!(";\r\n{}}}", parents::get_indent(ast))
            }
            ElementInfo::Struct(_, _, _) => {
                // cleanup children
                ast.elements[el_index].1 = vec![];
                /*
                let struct_children = ast.elements[el_index].1;
                for i in 0..struct_children.len() {
                    let child_assignment = struct_children[i];


                    let assignment_children = ast.elements[child_assignment].1;
                    for a in 0..assignment_children.len() {
                        let child_constant = assignment_children[a];
                        let constant_children = ast.elements[child_constant].1;
                        for v in 0..constant_children.len() {
                            let child_value =
                        }
                    }
                }
                */
                dbg!("struct");
                "".to_string()
            }
            _ => "".to_string(),
        };
        set_output_append(ast, &element_string);
    }
}

/// Append a string to the output string
fn set_output_append(ast: &mut Ast, append_string: &str) {
    ast.log(format!("output::set_output_append {:?}", append_string));
    ast.output = format!("{}{}", ast.output, append_string);
}

/// Append a string to the output string for the premain section
fn set_premain_output_append(ast: &mut Ast, append_string: &str) {
    ast.log(format!(
        "output::set_premain_output_append {:?}",
        append_string
    ));
    ast.premain_output = format!("{}{}", ast.premain_output, append_string);
}
