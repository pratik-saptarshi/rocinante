# ui/src/test/

## Responsibility
Shared test setup and browser-side validation utilities for the UI package.

## Design
Keeps test environment initialization separate from feature modules. Contains
setup code that wires DOM matchers and runtime shims for Vitest-compatible
component tests.

## Flow
1. Test runner loads the shared setup file.
2. DOM and assertion helpers are registered once per suite.
3. Component and contract tests import the setup implicitly through config.

## Integration
- Consumed by Vitest and any browser-oriented tests that rely on the shared
  setup configuration.
