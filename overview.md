# overview

1. compiler.new - initialise compiler state
1. compiler.run
    - compiler.file.get - read input file
    - compiler.run_main_tasks
        - compiler.set_lines_of_chars
        - compiler.set_lines_of_tokens
        - compiler.run_main_loop
            - compiler.main_loop_over_lines_of_tokens
                - loop over compiler.lines_of_tokens
                    - parse::current_line
                        - loop over tokens in line
                            - current_token
                                - if get_inbuilt_function_index_by_name
                                    - if ElementInfo::InbuiltFunctionDef -> inbuilt_function_call
                                    - if ElementInfo::FunctionDef -> function_call
                                    - if ElementInfo::Arg
                                        - if function_call
                                        - else token_by_first_chars
                                    - else token_by_first_chars
                                - else
                                    - if get_inbuilt_type_index_by_name
                                    - else token_by_first_chars
                - OK = compiler.ast.output.set_output
                    - loop 10x replace_any_unknown_types
                    - set_output_append - manually add first line "fn main() {"
                    - indent
                    - loop over compiler.ast.elements
                    - **incomplete...**
                - ERR = loop println over compiler.error_stack
    - compiler.file.writefile_or_error

main.rs

-   compiler = Compiler::new
-   compiler.run

lib.rs

-   Compiler
    -   file
    -   debug
    -   filepath
    -   outputdir
    -   lines_of_chars
    -   lines_of_tokens
    -   output
    -   current_line
    -   current_line_token
    -   error_stack
    -   ast
    -   (methods)
        -   new
        -   run
        -   run_main_tasks
        -   run_main_loop
        -   main_loop_over_lines_of_tokens
        -   set_lines_of_chars
        -   set_lines_of_tokens

---

ast.rs

-   Ast
    -   (fmt::Debug)
    -   (methods)
        -   new
    -   (fn)
        -   debug_flat_usize_array
        -   get_initial_types
        -   get_booleans
        -   get_boolean_fns
        -   get_initial_arithmetic_operators
        -   get_list_functions

errors.rs

-   Errors
    -   (tests)
    -   (fn)
        -   append_error
        -   error_if_parent_is_invalid
        -   error_if_parent_is_invalid_for_list
        -   error_if_parent_is_invalid_for_commentsingleline
        -   error_if_parent_is_invalid_for_int
        -   error_if_parent_is_invalid_for_float
        -   error_if_parent_is_invalid_for_string
        -   error_if_parent_is_invalid_for_bool
        -   error_if_parent_is_invalid_for_arg
        -   error_if_parent_is_invalid_for_constantref
        -   error_if_parent_is_invalid_for_constant
        -   error_if_parent_is_invalid_for_assignment
        -   error_if_parent_is_invalid_for_inbuiltfncall
        -   error_if_parent_is_invalid_for_fncall
        -   error_if_parent_is_invalid_for_parenthesis
        -   error_if_parent_is_invalid_for_loopfor
        -   error_if_parent_is_invalid_for_fndefwip
        -   error_if_parent_is_invalid_for_println
        -   error_if_parent_is_invalid_for_if_expression

file.rs

-   File
    -   (methods)
        -   new
        -   get
        -   writefile_or_error

formatting.rs

-   (tests)
-   (fn)
    -   get_formatted_argname_argtype_pairs

parse.rs

-   (tests) - big
-   (fn)
    -   types
    -   current_line
    -   current_token
    -   token_by_first_chars
    -   comment_single_line
    -   println
    -   if_expression
    -   string
    -   int
    -   float
    -   constant
    -   assignment
    -   function_call
    -   list_empty
    -   list_start
    -   list_end
    -   function_definition_start
    -   loop_for_range_start
    -   loop_end
    -   function_definition_end
    -   functiontypesig_or_functionreference_start
    -   functiontypesig_or_functionreference_end
    -   is_integer
    -   is_float
    -   is_string
    -   get_args_from_dyn_fn
    -   concatenate_vec_strings
    -   strip_leading_whitespace
    -   strip_trailing_whitespace

---

ast/elements/append.rs

-   (fn)
    -   append
    -   \_append_as_ref
    -   types
    -   indent_if_first_in_line
    -   comment_single_line
    -   println
    -   if_expression
    -   string
    -   outdent_if_last_expected_child
    -   seol_if_last_in_line
    -   is_return_expression
    -   int
    -   float
    -   assignment
    -   inbuilt_function_call
    -   function_call1
    -   list_start
    -   function_definition_start
    -   loop_for_range_start
    -   functiontypesig_or_functionreference_start
    -   functiontypesig_or_functionreference_end
    -   constant_ref
    -   new_constant_or_arg
    -   function_ref_or_call
    -   function_call

ast/parents/indent.rs

-   (fn)
    -   indent
    -   indent_this

ast/parent/outdent.rs

-   (fn)
    -   outdent
    -   within_fndef_from_return_expression
    -   within_fndef_for_inbuiltfncall_from_inbuiltfndef
    -   inbuiltfncall_from_inbuiltfndef
    -   within_fndef_for_fncall_from_fndef
    -   fncall
    -   println
    -   constant
    -   assignment
    -   if_expression
    -   functioncall_of_arg
    -   functioncall_of_functiondef

ast/elements.rs

-   ElementInfo - main enum of types
-   (fmt::Debug)
-   (tests)
-   (fn)
    -   get_element_by_name
    -   get_arg_index_by_name
    -   get_inbuilt_type_index_by_name
    -   get_constant_index_by_name
    -   get_constant_by_name
    -   get_function_index_by_name
    -   get_inbuilt_function_index_by_name
    -   \_get_inbuilt_function_index_by_name_and_returntype
    -   get_inbuilt_function_by_name
    -   \_get_inbuilt_function_by_name_and_returntype
    -   get_last_element
    -   get_updated_elementinfo_with_infered_type
    -   get_infered_type_of_any_element
    -   get_infered_type_of_arg_element
    -   get_infered_type_of_constant_element
    -   get_infered_type_of_constantref_element
    -   get_infered_type_of_inbuiltfunctioncall_element
    -   get_infered_type_of_functioncall_element
    -   get_infered_type_of_if_element
    -   get_elementinfo_type
    -   is_existing_constant
    -   replace_element_child
    -   replace_funcdefwip_with_funcdef
    -   get_argtypes_from_argtokens
    -   get_returntype_from_argtokens
    -   get_argnames_from_argtokens
    -   get_formatted_dyn_fn_type_sig

ast/output.rs

-   (fn)
    -   replace_any_unknown_types
    -   get_depths_vec
    -   get_depths_flattened
    -   get_output_for_element_index - big
    -   set_output
    -   set_output_for_element_open
    -   set_output_for_element_close
    -   set_output_append
    -   set_output_append_no_indent

ast/parents.rs

-   (fn)
    -   get_current_parent_element_from_parents
    -   get_current_parent_ref_from_parents
    -   get_current_parent_element_from_element_children_search
    -   get_current_parent_ref_from_element_children_search
    -   get_indent
    -   vec_remove_head
    -   vec_remove_tail
