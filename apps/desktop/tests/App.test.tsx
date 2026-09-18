import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App } from "../src/app/App";

describe("app scaffold", () => {
  it("renders an empty application surface", () => {
    render(<App />);

    expect(
      screen.getByRole("main", { name: "SilverSetup" }),
    ).toBeEmptyDOMElement();
  });
});
