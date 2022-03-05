module.exports = {
  "**/*.{js,ts}": ["jest --bail --findRelatedTests", "eslint --fix"],
  "**/*.{js,ts,svelte,scss,sass,css}": ["prettier --write"],
};
