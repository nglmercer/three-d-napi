import { expect, test, describe, beforeEach } from "bun:test";
import * as three_d from "../index";
import { Context } from "../index";

describe("Context Management", () => {
  let ctx: Context;

  beforeEach(() => {
    ctx = new Context();
  });

  test("Context constructor creates valid instance", () => {
    expect(ctx).toBeDefined();
    expect(ctx).toBeInstanceOf(three_d.Context);
  });

  test("Context has valid information", () => {
    expect(ctx.isValid()).toBe(true);
    expect(ctx.getInfo()).toBeString();
    expect(ctx.getInfo()).toContain("Context");
  });

  test("Context returns GLSL version", () => {
    const glsl = ctx.getGlslVersion();
    expect(glsl).toBeString();
    expect(glsl).toMatch(/GLSL/);
  });

  test("Context version supports OpenGL 3.2", () => {
    const version = ctx.getVersion();
    expect(version).toBeDefined();
    if (version) {
      expect(version.supports(3, 2)).toBe(true);
      expect(version.toString()).toContain("OpenGL");
      expect(version.glslTarget()).toBeDefined();
    }
  });
});

describe("Version", () => {
  test("Constructor", () => {
    const version = new three_d.Version(3, 2);
    expect(version).toBeInstanceOf(three_d.Version);
  });

  test("toString() method", () => {
    const version = new three_d.Version(3, 2);
    const str = version.toString();
    expect(str).toContain("OpenGL");
    expect(str).toContain("3.2");
  });

  test("supports() method", () => {
    const version = new three_d.Version(3, 2);
    expect(version.supports(3, 2)).toBe(true);
    expect(version.supports(4, 6)).toBe(false);
  });

  test("glslTarget() method", () => {
    const version = new three_d.Version(3, 2);
    const glsl = version.glslTarget();
    expect(glsl).toBeString();
    expect(glsl).toContain("GLSL");
  });
});
