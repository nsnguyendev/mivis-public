/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  roots: ['<rootDir>/tests'],
  // Only match files ending with .integration.test.ts/js
  testMatch: ['<rootDir>/tests/**/*.integration.test.[jt]s?(x)'],
  moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx', 'json', 'node'],
  transform: {
    '^.+\\.tsx?$': ['ts-jest', {
      tsconfig: 'tsconfig.json',
    }],
  },
  clearMocks: true,
  // Increase timeout for integration tests as they involve network calls
  testTimeout: 30000, // 30 seconds
};
