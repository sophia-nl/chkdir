#![deny(
    clippy::all,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]
#![warn(
    clippy::absolute_paths,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::as_underscore,
    clippy::assertions_on_result_states,
    clippy::big_endian_bytes,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::default_union_representation,
    clippy::deref_by_slicing,
    clippy::disallowed_script_idents,
    clippy::else_if_without_else,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::error_impl_error,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::host_endian_bytes,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::indexing_slicing,
    clippy::inline_asm_x86_att_syntax,
    clippy::inline_asm_x86_intel_syntax,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::little_endian_bytes,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::min_ident_chars,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::missing_inline_in_public_items,
    clippy::missing_trait_methods,
    clippy::mixed_read_write_in_expression,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::needless_raw_strings,
    clippy::non_ascii_literal,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::pattern_type_mismatch,
    clippy::pub_use,
    clippy::pub_with_shorthand,
    clippy::pub_without_shorthand,
    clippy::question_mark_used,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::redundant_type_annotations,
    clippy::ref_patterns,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::semicolon_inside_block,
    clippy::semicolon_outside_block,
    clippy::separated_literal_suffix,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::single_char_lifetime_names,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::todo,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm
)]
#![allow(clippy::module_name_repetitions)]

mod args;
mod diff;
mod fs;
mod item;
mod last;
mod new;
mod walk;

use args::ArgumentsResult::{Help, InvalidArguments, UserSelectDir, Version};

fn main() {
    match args::parse() {
        Help(help) => println!("{help}"),
        InvalidArguments(error) => error.suggestion(),
        UserSelectDir(working_dir) => {
            if let Ok(walk_result) = walk::walk(working_dir.as_path()) {
                let new_check_result = new::create(walk_result.contents, working_dir.as_path());
                last::find(&walk_result.result_files).map_or_else(
                    || {
                        new_check_result.write(&working_dir);
                        println!("\x1B[1mThe first check is done.\x1B[0m");
                    },
                    |last_check_result| {
                        let diff_result = diff::diff(&last_check_result, &new_check_result);
                        if diff_result.added.is_empty() && diff_result.deleted.is_empty() {
                            println!("\x1B[1mNo change.\x1B[0m");
                        } else {
                            new_check_result.write(&working_dir);
                            diff_result.print();
                        }
                    },
                );
            } else {
                eprintln!(
                    "\x1B[91;1merror\x1B[0;1m:\x1B[0m \"\x1B[0;1m{}\x1B[0m\" can not be read.",
                    working_dir.display()
                );
            }
        }
        Version(version) => println!("{version}"),
    }
}
