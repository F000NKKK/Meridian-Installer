import { describe, expect, it } from "vitest";
import { formatBytes } from "./format";

describe("formatBytes", () => {
  it("writes bytes and megabytes without decimals", () => {
    expect(formatBytes(512)).toBe("512 B");
    expect(formatBytes(59 * 1024 * 1024)).toBe("59 MB");
  });

  it("writes gigabytes with two decimals", () => {
    expect(formatBytes(2.1 * 1024 ** 3)).toBe("2.10 GB");
  });
});
