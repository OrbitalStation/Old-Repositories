pub(super) fn remove_comments(mut code: String) -> String {
	let mut offset = 0;

	loop {
		loop {
			let first_non_tab_or_space = match code[offset..].find(|c: char| c != '\t' && c != ' ') {
				Some(x) => x,
				_ => break
			} + offset;

			let coff = &code[first_non_tab_or_space..];

			if coff.starts_with("*") && [' ', '\t'].contains(&coff.chars().skip(1).next().unwrap_or(' ')) {
				// Do not strip off the '\n' as it is needed to preserve span
				let end = coff.find('\n').unwrap_or_else(|| code.len() - first_non_tab_or_space);
				// Replace characters with ' ' instead of simply deleting to, again, preserve span
				code.replace_range(first_non_tab_or_space..first_non_tab_or_space + end, &' '.to_string().repeat(end));
			}

			break
		}

		offset += match code[offset..].find('\n') {
			Some(x) => x,
			None => break
		} + '\n'.len_utf8();
	}

	code
}
