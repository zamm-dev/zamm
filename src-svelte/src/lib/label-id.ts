import { customAlphabet } from "nanoid/non-secure";

const nanoid = customAlphabet("1234567890", 6);

export default function getComponentId(componentType?: string) {
  const prefix = componentType || "component";
  return `${prefix}-${nanoid()}`;
}
