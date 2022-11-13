import { PolywrapClient } from "@polywrap/client-js";
import * as App from "../types";
import path from "path";

jest.setTimeout(60000);

describe("Monowrap", () => {

  const client: PolywrapClient = new PolywrapClient();
  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  })

  it("calls sampleMethod", async () => {
    const expected: string = "polywrap";

    const result = await App.Monowrap_Module.getManifest({
      path: "",
    }, client, wrapperUri)

    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value).toEqual(expected);
  });
});
