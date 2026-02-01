const config = {
  "*.{js,jsx,ts,tsx}": ["eslint --fix", "prettier --write"],
  "*.{json,css,md,html,yml,yaml,ejs}": ["prettier --write"],
  "*.rs": [() => "cargo fmt --all", () => "cargo clippy -- -D warnings"],
};

export default config;
