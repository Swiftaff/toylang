use crate::ast::elements;
use crate::ast::elements::ElementInfo;
use crate::ast::parents;
use crate::formatting;
use crate::Ast;
use crate::Compiler;

pub fn replace_any_unknown_types(ast: &mut Ast) {
    let depths = get_depths_vec(ast.clone());
    let depths_flattened = get_depths_flattened(&depths);
    //dbg!(&depths_flattened);
    for el_index in depths_flattened {
        ast.elements[el_index].0 =
            elements::get_updated_elementinfo_with_infered_type(ast, el_index);
    }
    //dbg!(&ast.elements);
}

pub fn get_depths_vec(ast: Ast) -> Vec<Vec<usize>> {
    // collect a vec of all children
    // from deepest block in the 'tree' to highest
    // (ordered top to bottom for block at same level)
    // and reverse order within each block
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

pub fn get_depths_flattened(depths: &Vec<Vec<usize>>) -> Vec<usize> {
    // flattens depths from bottom (deepest) to top
    // this is so that it can be used to traverse elements in the correct order
    // to allow correcting the types from the deepest elements first
    // since higher levels may rely on type of deeper elements.
    // e.g. a higher level "+" fn with type "i64|f64" will need to be disambiguated
    // to either i64 or f64 based on the type of it's 2 child args
    // so the two child args are fixed first (if unknown)
    // then "+" fn can be determined safely
    let mut output = vec![];
    for i in (0..depths.len()).rev() {
        let level = &depths[i];
        output = vec![].iter().chain(&output).chain(level).cloned().collect();
    }
    output
}

pub fn get_output_for_element_index(
    ast: &mut Ast,
    element_index: usize,
    skip_in_case_handled_by_parent: bool,
) -> String {
    let element = ast.elements[element_index].clone();
    //dbg!(&element.0);
    //dbg!(&element, ast.parents); //            ast.get_current_parent_ref_from_parents(),            ast.get_current_parent_element()   );
    let skip = "".to_string();

    //skip children for certain parents who already parsed them
    if skip_in_case_handled_by_parent {
        match parents::get_current_parent_element_from_element_children_search(&ast, element_index)
        {
            Some((ElementInfo::Assignment, _)) => return skip,
            Some((ElementInfo::FunctionCall(_, _), _)) => return skip,
            Some((ElementInfo::Println, _)) => return skip,
            Some((ElementInfo::List(_), _)) => return skip,
            Some((ElementInfo::If(_), _)) => return skip,
            // explicitly listing other types rather than using _ to not overlook new types in future.
            Some((ElementInfo::Root, _)) => (),
            Some((ElementInfo::CommentSingleLine(_), _)) => (),
            Some((ElementInfo::Int(_), _)) => (),
            Some((ElementInfo::Float(_), _)) => (),
            Some((ElementInfo::String(_), _)) => (),
            Some((ElementInfo::Arg(_, _, _), _)) => (),
            Some((ElementInfo::Constant(_, _), _)) => (),
            Some((ElementInfo::ConstantRef(_, _, _), _)) => (),
            Some((ElementInfo::InbuiltFunctionDef(_, _, _, _, _), _)) => (),
            Some((ElementInfo::InbuiltFunctionCall(_, _, _), _)) => (),
            Some((ElementInfo::FunctionDefWIP, _)) => (),
            Some((ElementInfo::FunctionDef(_, _, _, _), _)) => (),
            Some((ElementInfo::LoopForRangeWIP, _)) => (),
            Some((ElementInfo::LoopForRange(_, _, _), _)) => (),
            Some((ElementInfo::Parens, _)) => (),
            Some((ElementInfo::Type(_), _)) => (),
            Some((ElementInfo::Eol, _)) => (),
            Some((ElementInfo::Seol, _)) => (),
            Some((ElementInfo::Indent, _)) => (),
            Some((ElementInfo::Unused, _)) => (),
            None => (),
        }
    }

    match element.0 {
        ElementInfo::Root => "".to_string(),
        ElementInfo::CommentSingleLine(comment_string) => format!("{}", comment_string),
        ElementInfo::Int(val) => format!("{}", val),
        ElementInfo::Float(val) => format!("{}", val),
        ElementInfo::String(val) => format!("{}.to_string()", val),
        ElementInfo::Arg(name, _scope, _returntype) => format!("{}", name).to_string(),
        ElementInfo::Constant(name, _returntype) => format!("{}", name).to_string(),
        ElementInfo::ConstantRef(name, _typename, _reference) => {
            format!("{}", name)
        }
        ElementInfo::Assignment => {
            let mut returntype = "Undefined".to_string();
            let children = element.1;
            if children.len() < 1 {
                format!("// let ?: ? = ? OUTPUT ERROR: Can't get constant for this assignment from : {:?}", children)
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
                format!("let {}: {} = ", constant_output, returntype)
            }
        }
        ElementInfo::InbuiltFunctionDef(name, _argnames, _argtypes, _returntype, _format) => {
            format!("fn {}() ->{{ /* stuff */ }}", name)
        }
        ElementInfo::List(returntype) => {
            //dbg!("List");
            let children = element.1;
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
                return format!("{} ]", &output);
            } else {
                let mut vec_type = returntype.clone();
                if returntype.len() > 5 && returntype[..3] == "Vec".to_string() {
                    vec_type = returntype[4..returntype.len() - 1].to_string();
                }
                return format!("Vec::<{}>::new()", vec_type);
            }
        }
        ElementInfo::InbuiltFunctionCall(name, _fndef_index, _returntype) => {
            //dbg!("InbuiltFunctionCall");
            if let Some(def) = elements::get_inbuilt_function_by_name(ast, &name) {
                match def {
                    ElementInfo::InbuiltFunctionDef(_, argnames, _, _, format) => {
                        let children = element.1;
                        //dbg!(&argnames, &children);
                        let mut output = format;
                        //dbg!(&output);
                        for i in 0..argnames.len() {
                            let arg_var_num = format!("arg~{}", i + 1);
                            let arg_value_el_ref = children[i];
                            let arg_output =
                                get_output_for_element_index(ast, arg_value_el_ref, true);
                            output = output.replace(&arg_var_num, &arg_output);
                            //dbg!("---",&arg_var_num,arg_value_el_ref,&arg_output,&output);
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
        ElementInfo::FunctionDefWIP => "".to_string(),
        ElementInfo::FunctionDef(name, argnames, argtypes, returntype) => {
            let args = formatting::get_formatted_argname_argtype_pairs(&argnames, &argtypes);
            format!("fn {}({}) -> {} {{\r\n", name, args, returntype)
        }
        ElementInfo::FunctionCall(name, _) => {
            let arguments = element.1;
            let mut args = "".to_string();
            for i in 0..arguments.len() {
                let arg_el_ref = arguments[i];
                //let arg_el = ast.elements[arg_el_ref];
                let arg = get_output_for_element_index(ast, arg_el_ref, false);
                let mut borrow = "".to_string();
                //dbg!("here", &name, &returntype, &arg_el);
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
                    "".to_string()
                } else {
                    ", ".to_string()
                };
                args = format!("{}{}{}{}", args, borrow, arg, comma);
            }
            format!("{}({})", name, args)
        }
        ElementInfo::Parens => {
            let children = &element.1;
            let mut output = "".to_string();
            for i in 0..children.len() {
                let child_ref = children[i];
                let child = get_output_for_element_index(ast, child_ref, false);
                output = format!("{}{}", output, child);
            }
            format!("({})", output)
        }
        ElementInfo::LoopForRangeWIP => "".to_string(),
        ElementInfo::LoopForRange(name, from, to) => {
            format!("For {} in {}..{} {{\r\n", name, from, to)
        }
        ElementInfo::Eol => format!("\r\n"),
        ElementInfo::Seol => format!(";\r\n"),
        ElementInfo::Indent => parents::get_indent(ast),
        ElementInfo::Type(name) => format!("{}", name),
        ElementInfo::Unused => "".to_string(),
        ElementInfo::Println => {
            let children = &element.1;
            let mut output = "".to_string();
            for i in 0..children.len() {
                let child_ref = children[i];
                let child = get_output_for_element_index(ast, child_ref, false);
                output = format!("{}{}", output, child);
            }
            format!("println!(\"{{}}\", {})", output)
        }
        ElementInfo::If(_returntype) => {
            dbg!("If");
            let children = element.1;
            let child1_output = get_output_for_element_index(ast, children[0], false);
            let mut output = format!("if {} {{", child1_output);
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
            //parents::outdent::outdent(&mut ast);
            return output;
        }
    }
}

pub fn set_output(compiler: &mut Compiler) {
    //dbg!(&ast);
    //let ast = &mut compiler.ast;
    for _i in 0..10 {
        replace_any_unknown_types(&mut compiler.ast);
    }
    set_output_append(&mut compiler.ast, "fn main() {\r\n");
    compiler.ast.parents = vec![0];
    // the values of indent and outdent don't matter when outputting - only using parents.len()
    // values do matter when building the ast
    parents::indent::indent(&mut compiler.ast);

    let mut stack: Vec<usize> = compiler.ast.elements[0].1.clone();
    while stack.len() > 0 {
        let current_item = stack[0];
        // remove current item from stack
        stack = parents::vec_remove_head(&stack);
        // if it is an outdent marker, outdent level!
        if current_item == 0 {
            parents::outdent::outdent(compiler);
            // push current end tag to output
            let end_tag = stack[0];

            set_output_for_element_close(&mut compiler.ast, end_tag);
            // removed the outdent marker earlier, now remove the end tag indicator
            stack = parents::vec_remove_head(&stack);
            // if the end_tag was the end of a func_def we don't want to display the trailing semicolon
            // since it needs to be treated as the return statement, so remove it if there is one
        } else {
            // push current to output
            set_output_for_element_open(&mut compiler.ast, current_item);
            // if current item has children...
            let mut current_item_children = compiler.ast.elements[current_item].1.clone();

            // don't render children of certain elements - they are rendered separately
            let el = &compiler.ast.elements[current_item];
            match el.0 {
                ElementInfo::InbuiltFunctionCall(_, _, _) => current_item_children = vec![],
                _ => (),
            }

            if current_item < compiler.ast.elements.len() && current_item_children.len() > 0 {
                // prepend with current item end tag indicator - so we know to close it at after the outdent
                stack.splice(0..0, vec![current_item]);
                // prepend with 0 (marker for outdent)
                stack.splice(0..0, vec![0]);
                // prepend with children
                stack.splice(0..0, compiler.ast.elements[current_item].1.clone());
                // and increase indent
                parents::indent::indent(&mut compiler.ast);
            }
        }
    }
    parents::outdent::outdent(compiler);
    set_output_append(&mut compiler.ast, "}\r\n");
    //println!("AST_OUTPUT\r\n{:?}\r\n{:?}", ast.elements, ast.output);
}

pub fn set_output_for_element_open(ast: &mut Ast, el_index: usize) {
    if el_index < ast.elements.len() {
        let element = ast.elements[el_index].clone();
        let element_string = get_output_for_element_index(ast, el_index, true);
        match element.0 {
            ElementInfo::Eol => set_output_append_no_indent(ast, &element_string),
            ElementInfo::Seol => set_output_append_no_indent(ast, &element_string),
            _ => set_output_append(ast, &element_string),
        }
    }
}

pub fn set_output_for_element_close(ast: &mut Ast, el_index: usize) {
    if el_index < ast.elements.len() {
        let element = &ast.elements[el_index];
        let element_string = match element.0 {
            ElementInfo::FunctionDef(_, _, _, _) => {
                format!("\r\n{}}}\r\n", parents::get_indent(ast))
            }
            ElementInfo::LoopForRange(_, _, _) => {
                format!(";\r\n{}}}", parents::get_indent(ast))
            }
            _ => "".to_string(),
        };
        set_output_append(ast, &element_string);
    }
}

pub fn set_output_append(ast: &mut Ast, append_string: &str) {
    ast.output = format!("{}{}", ast.output, append_string);
}

pub fn set_output_append_no_indent(ast: &mut Ast, append_string: &str) {
    ast.output = format!("{}{}", ast.output, append_string);
}
