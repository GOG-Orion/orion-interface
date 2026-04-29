import { describe, expect, it } from "vitest";
import { getMenuText } from "./languages";

describe("getMenuText", () => {
  it("returns the requested language entry", () => {
    const portuguese = getMenuText("PT-BR");

    expect(portuguese.menu_item01).toBe("Integrações");
    expect(portuguese.int_search).toBe("Buscar");
  });

  it("falls back to English when the language is unknown", () => {
    const fallback = getMenuText("ZZZ");

    expect(fallback.lang).toBe("ENG");
    expect(fallback.menu_item04).toBe("Configuration");
  });
});
