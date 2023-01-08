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

    const result = await App.Monowrap_Module.getManifest({
      path: "src/schemas/manifest.example.json",
    }, client, wrapperUri)

    // console.log(result)

    expect(result.ok).toBeTruthy();
    if (!result.ok) return;

    const res = await App.Monowrap_Module.buildContextGraphs({
      manifest: result.value,
    }, client, wrapperUri);

    if (!res.ok) throw Error();
    console.log(Object.fromEntries(res.value.commandGraph.vertices.entries()))
    console.log(Object.fromEntries(res.value.commandGraph.adjList.entries()))
  });
});


let k = { test: ["lint", "build"], lint: ["codegen"], build: ["codegen"]}