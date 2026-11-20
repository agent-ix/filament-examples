export default {
  name: "fsl-agent-evals",
  rootDir: import.meta.dirname,
  reportsDir: "reports",
  provider: {
    command: "agent-evals",
    args: ["provider"],
    timeoutMs: 300000,
  },
};
