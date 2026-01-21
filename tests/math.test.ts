import { expect, test, describe } from "bun:test";
import * as three_d from "../index";

describe("Point Types", () => {
  test("Point2 Constructor", () => {
    const p2 = new three_d.Point2(1.5, 2.5);
    expect(p2).toBeInstanceOf(three_d.Point2);
    expect(p2.x).toBe(1.5);
    expect(p2.y).toBe(2.5);
  });

  test("Point3 Constructor", () => {
    const p3 = new three_d.Point3(1.0, 2.0, 3.0);
    expect(p3).toBeInstanceOf(three_d.Point3);
    expect(p3.x).toBe(1.0);
    expect(p3.y).toBe(2.0);
    expect(p3.z).toBe(3.0);
  });
});

describe("Vector2", () => {
  test("Constructor and properties", () => {
    const v2 = new three_d.Vector2(3.0, 4.0);
    expect(v2).toBeInstanceOf(three_d.Vector2);
    expect(v2.x).toBe(3.0);
    expect(v2.y).toBe(4.0);
  });

  test("Operations", () => {
    const v2 = new three_d.Vector2(3.0, 4.0);
    expect(v2.length()).toBeCloseTo(5.0, 5);

    const normalized = v2.normalize();
    expect(normalized.length()).toBeCloseTo(1.0, 5);

    const v1 = new three_d.Vector2(1.0, 2.0);
    expect(v1.dot(v2)).toBeCloseTo(11.0, 5);
    expect(v1.add(v2).x).toBe(4.0);
    expect(v1.sub(v2).x).toBe(-2.0);
  });
});

describe("Vector3", () => {
  test("Constructor and properties", () => {
    const v3 = new three_d.Vector3(1.0, 2.0, 3.0);
    expect(v3).toBeInstanceOf(three_d.Vector3);
  });

  test("Operations", () => {
    const v1 = new three_d.Vector3(1.0, 0.0, 0.0);
    const v2 = new three_d.Vector3(0.0, 1.0, 0.0);

    expect(v1.dot(v2)).toBeCloseTo(0.0, 5);

    const cross = v1.cross(v2);
    expect(cross.z).toBeCloseTo(1.0, 5);

    const lerped = v1.lerp(new three_d.Vector3(10, 0, 0), 0.5);
    expect(lerped.x).toBeCloseTo(5.5, 5);
  });
});

describe("Vector4", () => {
  test("Constructor and properties", () => {
    const v4 = new three_d.Vector4(1.0, 2.0, 3.0, 4.0);
    expect(v4).toBeInstanceOf(three_d.Vector4);
  });

  test("Operations", () => {
    const v4 = new three_d.Vector4(2.0, 0.0, 0.0, 0.0);
    expect(v4.length()).toBe(2.0);
    expect(v4.normalize().length()).toBeCloseTo(1.0, 5);
  });
});

describe("Matrices", () => {
  test("Matrix2", () => {
    const m = new three_d.Matrix2();
    expect(m.data).toBeArray();
    expect(three_d.Matrix2.identity().data).toEqual([1, 0, 0, 1]);
  });

  test("Matrix3", () => {
    const m = three_d.Matrix3.identity();
    expect(m.data.length).toBe(9);
  });

  test("Matrix4", () => {
    const m = three_d.Matrix4.identity();
    expect(m.data.length).toBe(16);
  });
});

describe("Math Types (Quaternions/Units)", () => {
  test("NDeg/NRad", () => {
    const deg = new three_d.NDeg(45.0);
    expect(deg.value).toBe(45.0);
    const rad = new three_d.NRad(3.14159);
    expect(rad.value).toBeCloseTo(3.14159, 5);
  });

  test("NQuaternion", () => {
    const q = new three_d.NQuaternion(1.0, 2.0, 3.0, 0.5);
    expect(q.w).toBe(0.5);
  });

  test("NF16", () => {
    const f = new three_d.Nf16(1.5);
    expect(f.value).toBe(1.5);
  });

  test("NSrgba", () => {
    const c = new three_d.NSrgba(1.0, 0.5, 0.0, 1.0);
    expect(c.r).toBe(1.0);
  });
});

describe("AxisAlignedBoundingBox", () => {
  test("Operations", () => {
    const aabb = new three_d.AxisAlignedBoundingBox(-10, -10, -10, 10, 10, 10);
    expect(aabb).toBeInstanceOf(three_d.AxisAlignedBoundingBox);
    expect(aabb.contains(5, 5, 5)).toBe(true);
    expect(aabb.contains(11, 11, 11)).toBe(false);
    expect(aabb.center()).toBeArray();
    expect(aabb.size()).toBeArray();
  });

  test("merge() method", () => {
    const aabb1 = new three_d.AxisAlignedBoundingBox(0, 0, 0, 10, 10, 10);
    const aabb2 = new three_d.AxisAlignedBoundingBox(5, 5, 5, 15, 15, 15);
    const merged = aabb1.merge(aabb2);
    expect(merged).toBeInstanceOf(three_d.AxisAlignedBoundingBox);
  });
});
