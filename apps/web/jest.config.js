const config = {
  testMatch: ["**/?(*.)+(spec|test).ts"],
  preset: "ts-jest",
  transform: {
    "^.+\\.(ts|tsx)?$": "ts-jest",
  },
  coverageReporters: ["json-summary", "text", "lcov"],
  collectCoverageFrom: ["**/*.{ts,tsx}"],
};

export default config;
