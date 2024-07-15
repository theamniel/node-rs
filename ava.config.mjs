export default {
  extensions: ['ts'],
  workerThreads: false,
  cache: false,
  require: ['jiti/register.js'],
  files: ['./packages/**/tests/*.spec.ts', './tests/**/*.spec.ts'],
  timeout: '3m',
  environmentVariables: {
    TS_NODE_PROJECT: './tsconfig.json',
  },
};
