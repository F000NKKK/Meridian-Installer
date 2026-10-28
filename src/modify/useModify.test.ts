import { describe, expect, it } from "vitest";
import { clearable } from "./useModify";

describe("clearable", () => {
  it("lets a chosen or recommended component be cleared", () => {
    expect(clearable(undefined)).toBe(true);
    expect(clearable({ kind: "chosen" })).toBe(true);
    expect(clearable({ kind: "recommended", workload: "w" })).toBe(true);
  });

  it("keeps what a workload or another component requires", () => {
    expect(clearable({ kind: "required", workload: "w" })).toBe(false);
    expect(clearable({ kind: "requiredBy", component: "c" })).toBe(false);
  });
});
