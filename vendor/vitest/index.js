import { isDeepStrictEqual } from "node:util";

function state() {
  if (!globalThis.__vitestShim) {
    globalThis.__vitestShim = { suites: [], current: [] };
  }
  return globalThis.__vitestShim;
}

export function describe(name, fn) {
  const testState = state();
  testState.current.push(name);
  try {
    fn();
  } finally {
    testState.current.pop();
  }
}

export function it(name, fn) {
  const testState = state();
  testState.suites.push({ name: [...testState.current, name].join(" > "), fn });
}

export function expect(received) {
  return {
    toBe(expected) {
      if (!Object.is(received, expected)) throw new Error(`Expected ${format(received)} to be ${format(expected)}`);
    },
    toEqual(expected) {
      if (!matchesDeep(received, expected)) throw new Error(`Expected ${format(received)} to equal ${format(expected)}`);
    },
    toContain(expected) {
      if (!received?.includes?.(expected)) throw new Error(`Expected ${format(received)} to contain ${format(expected)}`);
    },
    toHaveLength(expected) {
      if (received?.length !== expected) throw new Error(`Expected length ${received?.length} to be ${expected}`);
    },
    toMatchObject(expected) {
      if (!matchesObject(received, expected)) throw new Error(`Expected ${format(received)} to match ${format(expected)}`);
    },
    toThrow(expected) {
      let thrown;
      try {
        received();
      } catch (error) {
        thrown = error;
      }
      if (!thrown) throw new Error("Expected function to throw");
      const message = thrown instanceof Error ? thrown.message : String(thrown);
      if (expected instanceof RegExp && !expected.test(message)) {
        throw new Error(`Expected thrown message ${format(message)} to match ${expected}`);
      }
    },
  };
}

expect.objectContaining = function objectContaining(expected) {
  return { __matcher: "objectContaining", expected };
};

function matchesObject(actual, expected) {
  if (expected?.__matcher === "objectContaining") return matchesObject(actual, expected.expected);
  if (expected === null || typeof expected !== "object") return Object.is(actual, expected);
  if (actual === null || typeof actual !== "object") return false;
  if (Array.isArray(expected)) {
    return Array.isArray(actual) && expected.every((value, index) => matchesObject(actual[index], value));
  }
  return Object.entries(expected).every(([key, value]) => matchesObject(actual[key], value));
}

function matchesDeep(actual, expected) {
  if (expected?.__matcher === "objectContaining") return matchesObject(actual, expected.expected);
  if (Array.isArray(expected)) {
    return Array.isArray(actual) && actual.length === expected.length && expected.every((value, index) => matchesDeep(actual[index], value));
  }
  if (expected && typeof expected === "object") {
    if (!actual || typeof actual !== "object") return false;
    const expectedEntries = Object.entries(expected);
    const actualEntries = Object.entries(actual);
    return expectedEntries.length === actualEntries.length && expectedEntries.every(([key, value]) => matchesDeep(actual[key], value));
  }
  return isDeepStrictEqual(actual, expected);
}

function format(value) {
  return typeof value === "string" ? JSON.stringify(value) : JSON.stringify(value, null, 2);
}
