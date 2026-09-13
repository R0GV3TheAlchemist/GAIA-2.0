# Accessibility notes (issue #28)

This is a basic WCAG considerations list for the in-process permission console.
It is not a certified audit.

## Keyboard
- Skip link to `#main`.
- Buttons are real `<button>` elements.
- Focus outline is 3px on `a:focus` and `button:focus`.

## Contrast
- Page uses light text (`#eee`) on a dark background (`#111`).
- Controls use a light border on a dark fill.
- This is a target, not a measured 4.5:1 proof for every state.

## Screen reader
- `lang="en"` on the root.
- Headings and `aria-labelledby` section labels.
- Intent stream uses `aria-live="polite"`.
- Revoke controls have `aria-label`.

## Not claimed
- No automated axe run.
- No live WebSocket region updates.
- No full WCAG 2.2 AA pass.
