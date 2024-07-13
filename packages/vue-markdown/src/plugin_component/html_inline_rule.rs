use crate::plugin_component::html_re::*;
use markdown_it::parser::inline::InlineRule;
use markdown_it::parser::inline::InlineState;
use markdown_it::MarkdownIt;

pub fn is_letter(char: number) -> bool {
  let lc = char | 0x20;
  return lc >= 0x61 && lc <= 0x7a;
}

pub fn html_inline_rule(state: InlineState, silent: bool) -> bool {
  let pos = state.pos;

  // if (!state.md.options.html) {
  //   return false;
  // }
  let max = state.pos_max;

  if (state.src.charCodeAt(pos) != 0x3c /* < */ || pos + 2 >= max) {
    return false;
  }

  // Quick fail on second char
  let char = state.src.charCodeAt(pos + 1);
  if (char != 0x21 /* ! */ &&
      char != 0x3f /* ? */ &&
      char != 0x2f /* / */ &&
      !isLetter(ch))
  {
    return false;
  }

  let slices = &state.src[pos..];
  let matched = HTML_TAG_RE.exec(slices);
  if !matched {
    return false;
  }

  if !silent {
    let token = state.push("html_inline", "", 0);
    token.content = state.src.slice(pos, pos + matched[0].length);
  }
  state.pos += matched[0].length;
  true
}
