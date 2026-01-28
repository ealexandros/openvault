## Coding Rules

- Always use **arrow functions** (`const fn = () => {}`); do not use `function` declarations.
- Prefer **TypeScript `type` aliases**; do not use `interface`.
- Use `cn()` for class name composition; do not use template literals (``).
- Avoid any; use unknown and narrow explicitly.
- No default exports. Always export named exports. Except for next.js pages.
- One React component per file.
- Don't use return on components when possible (use JSX instead).

- If a React component contains complex logic, extract it into a custom hook. The hook should have the name of the component with a **use** prefix in a separate file.
- If there is a reusable component, extract it into a separate file somewhere in the `components` directory.
