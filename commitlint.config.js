const pc = require("picocolors");

const scoreRule = (parsed) => {
  const validScores = ["js-plugins", "rust-plugins", "all", "none"];
  const isValid = validScores.includes(parsed.scope);

  return [
    isValid,
    `current scope is ${pc.yellow(parsed.scope)}, ${pc.green(
      "because we need the right scope to do CI dispatch"
    )} .Mark ${pc.yellowBright(
      "js-plugins"
    )} if you need to change the ${pc.yellowBright("js package")}.
    Mark ${pc.yellowBright(
      "rust-plugins"
    )} if you need to change the ${pc.yellowBright(
      "rust package"
    )} mark ${pc.yellowBright("all")} if you have ${pc.yellowBright(
      "both"
    )} and ${pc.yellowBright("none if you don't need to send packages")}`,
  ];
};

module.exports = {
  rules: {
    // "scope-enum": [2, "always", ["js-plugins", "rust-plugins", "all", "none"]],
    "score-rule": [2, "always"],
    "type-enum": [
      2,
      "always",
      ["feat", "fix", "docs", "style", "refactor", "perf", "test", "chore"],
    ],
  },
  plugins: [
    {
      rules: {
        "score-rule": scoreRule,
      },
    },
  ],
};
