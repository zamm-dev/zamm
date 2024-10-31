import { type Result } from "./bindings";

export async function unwrap<T, U>(promise: Promise<Result<T, U>>) {
  const result = await promise;
  if (result.status === "ok") {
    return result.data;
  } else {
    throw result.error;
  }
}
