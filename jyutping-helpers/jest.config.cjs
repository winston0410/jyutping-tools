module.exports = {
  preset: "ts-jest",
  transform: {
    "^.+\\.(ts|tsx)?$": "ts-jest",
  },
  coverageReporters: ["json-summary", "text", "lcov"],
  collectCoverageFrom: ["**/*.{ts,tsx}"],
};
