// Minimal CodeMirror 6 bundle: syntax highlighting + theme + basic editing.
// Avoids basicSetup's heavy extras (autocomplete, bracket matching, fold
// gutter, search panel, etc.) to keep the bundle small.

import { EditorView, keymap, drawSelection, highlightActiveLine } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { syntaxHighlighting, defaultHighlightStyle, indentUnit } from "@codemirror/language";
import { rust } from "@codemirror/lang-rust";
import { oneDark } from "@codemirror/theme-one-dark";

const minimalSetup = [
    drawSelection(),
    highlightActiveLine(),
    history(),
    syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
    keymap.of([...defaultKeymap, ...historyKeymap, indentWithTab]),
    indentUnit.of("  "),
    EditorState.tabSize.of(2),
];

export { EditorView, EditorState, minimalSetup, rust, oneDark };
