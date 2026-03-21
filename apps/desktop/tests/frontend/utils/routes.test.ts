import { matchRoute, matchRoutes, routeJoin } from "@/utils/routes";
import { describe, expect, it } from "bun:test";

describe("matchRoute", () => {
  const patterns = ["/images", "/images/:id"];

  it("should return true for an exact static match", () => {
    expect(matchRoute("/home", "/home")).toBe(true);
  });

  it("should return true for a dynamic parameter match", () => {
    expect(matchRoute("/users/123", "/users/:id")).toBe(true);
  });

  it("should return true for multiple dynamic parameters", () => {
    expect(matchRoute("/products/electronics/tv", "/products/:category/:item")).toBe(true);
  });

  it("should return true when the path with the parameter is matched", () => {
    expect(matchRoutes("/images/10", patterns)).toBe(true);
  });

  it("should return true when the base path without the parameter is matched", () => {
    expect(matchRoutes("/images", patterns)).toBe(true);
  });

  it("should return false for a completely different path", () => {
    expect(matchRoutes("/photos", patterns)).toBe(false);
  });

  it("should return false for a path that is too long", () => {
    expect(matchRoutes("/images/10/edit", patterns)).toBe(false);
  });

  it("should handle URL-encoded characters correctly", () => {
    const pathname = "/users/John%20Doe";
    const pattern = "/users/:name";
    expect(matchRoute(pathname, pattern)).toBe(true);
  });

  it("should return false for a non-matching static path", () => {
    expect(matchRoute("/home", "/about")).toBe(false);
  });

  it("should return false for a partially matching path", () => {
    expect(matchRoute("/users/123/profile", "/users/:id")).toBe(false);
  });

  it("should return false when a required dynamic parameter is missing", () => {
    expect(matchRoute("/users", "/users/:id")).toBe(false);
  });

  it("should return false for an empty pathname against a valid pattern", () => {
    expect(matchRoute("", "/home")).toBe(false);
  });
});

describe("matchRoutes", () => {
  const patterns = ["/dashboard", "/users/:id", "/settings"];

  it("should return true if the pathname matches the first pattern in the array", () => {
    expect(matchRoutes("/dashboard", patterns)).toBe(true);
  });

  it("should return true if the pathname matches a dynamic pattern in the middle of the array", () => {
    expect(matchRoutes("/users/456", patterns)).toBe(true);
  });

  it("should return true if the pathname matches the last pattern in the array", () => {
    expect(matchRoutes("/settings", patterns)).toBe(true);
  });

  it("should return false if the pathname matches none of the patterns", () => {
    expect(matchRoutes("/about", patterns)).toBe(false);
  });

  it("should return false for a partially matching path that is not in the list", () => {
    expect(matchRoutes("/users/456/edit", patterns)).toBe(false);
  });

  it("should return false if the array of patterns is empty", () => {
    expect(matchRoutes("/any/path", [])).toBe(false);
  });
});

describe("routeJoin", () => {
  it("should join base and a single path correctly", () => {
    expect(routeJoin("/dictionary", "apple")).toBe("/dictionary/apple");
  });

  it("should handle base with trailing slash", () => {
    expect(routeJoin("/dictionary/", "apple")).toBe("/dictionary/apple");
  });

  it("should handle base with trailing slashes on both", () => {
    expect(routeJoin("/dictionary/", "/apple/")).toBe("/dictionary/apple");
  });

  it("should handle path with leading slash", () => {
    expect(routeJoin("/dictionary", "/apple")).toBe("/dictionary/apple");
  });

  it("should handle multiple paths", () => {
    expect(routeJoin("/dictionary", "apple", "123")).toBe("/dictionary/apple/123");
  });

  it("should skip undefined paths", () => {
    expect(routeJoin("/dictionary", undefined, "123")).toBe("/dictionary/123");
  });

  it("should skip empty string paths", () => {
    expect(routeJoin("/dictionary", "", "123")).toBe("/dictionary/123");
  });

  it("should return base only if no paths provided", () => {
    expect(routeJoin("/dictionary")).toBe("/dictionary");
  });

  it("should remove extra leading slashes from paths", () => {
    expect(routeJoin("/dictionary", "///apple", "//123")).toBe("/dictionary/apple/123");
  });

  it("should remove trailing slash from base even if paths have leading slashes", () => {
    expect(routeJoin("/dictionary/", "/apple", "/123")).toBe("/dictionary/apple/123");
  });
});
