export class AssertionError extends Error {}

export function assert(
  condition: boolean,
  description?: string,
): asserts condition {
  if (condition) return;
  if (description === undefined) {
    description = "assertion failed";
    console.error("assertion failed");
  } else {
    console.error("assertion failed:", description);
  }
  throw new AssertionError(description);
}

export function assertUnreachable(): never {
  assert(false, "unreachable code reached");
}
