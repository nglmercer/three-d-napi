import { expect, test, describe } from "bun:test";
import * as three_d from "../index";

describe("Buffer System", () => {
  describe("VertexBuffer", () => {
    test("Constructor", () => {
      const vbo = new three_d.VertexBuffer("positions", 1000, "static_draw");
      expect(vbo).toBeInstanceOf(three_d.VertexBuffer);
      expect(vbo.getBufferType()).toBe("Vertex");
      expect(vbo.estimateSizeBytes()).toBeGreaterThan(0);
      expect(vbo.isValid()).toBe(true);
    });
  });

  describe("ElementBuffer", () => {
    test("Constructor", () => {
      const ebo = new three_d.ElementBuffer("indices", 3000, "dynamic_draw");
      expect(ebo).toBeInstanceOf(three_d.ElementBuffer);
      expect(ebo.getBufferType()).toBe("Element");
      expect(ebo.getUsage()).toBe("dynamic_draw");
    });
  });

  describe("InstanceBuffer", () => {
    test("Constructor with data", () => {
      const data = Array(16 * 50)
        .fill(0)
        .map((_, i) => Math.random());
      const ibo = new three_d.InstanceBuffer("instances", 50, data);
      expect(ibo).toBeInstanceOf(three_d.InstanceBuffer);
      expect(ibo.getBufferType()).toBe("Instance");
    });
  });

  describe("UniformBuffer", () => {
    test("Constructor", () => {
      const ubo = new three_d.UniformBuffer("matrices", 0, 1024);
      expect(ubo).toBeInstanceOf(three_d.UniformBuffer);
      expect(ubo.getBufferType()).toBe("Uniform");
      expect(ubo.getSizeBytes()).toBe(1024);
    });
  });
});
