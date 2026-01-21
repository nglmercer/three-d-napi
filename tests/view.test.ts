import { expect, test, describe } from "bun:test";
import * as three_d from "../index";

describe("Camera", () => {
  test("Camera constructor with flat coordinates", () => {
    const cam = new three_d.Camera(
      1,
      2,
      3, // position
      0,
      0,
      0, // target
      0,
      1,
      0, // up
      60, // fov
      0.1, // near
      1000, // far
    );

    expect(cam).toBeInstanceOf(three_d.Camera);
    const pos = cam.getPosition();
    expect(pos).toBeArray();
    expect(pos[0]).toBe(1);
  });

  test("Camera with different parameters", () => {
    const cam = new three_d.Camera(10, 5, -2, 0, 0, 0, 0, 0, 1, 45, 0.5, 100);
    expect(cam).toBeDefined();
  });
});

describe("Viewport", () => {
  test("Viewport constructor", () => {
    const vp = new three_d.Viewport(0, 0, 800, 600);
    expect(vp).toBeInstanceOf(three_d.Viewport);
    expect(vp.getInfo()).toContain("Viewport");
  });

  test("Viewport atOrigin factory", () => {
    const vp = three_d.Viewport.atOrigin(1920, 1080);
    expect(vp).toBeInstanceOf(three_d.Viewport);
  });

  test("Viewport calculations", () => {
    const vp = new three_d.Viewport(0, 0, 400, 300);
    expect(vp.aspectRatio()).toBeCloseTo(1.3333, 4);
    expect(vp.contains(200, 200)).toBe(true);
    expect(vp.contains(500, 500)).toBe(false);
  });
});
