import fs from "fs";
import yaml from "js-yaml";
import { Convert } from "./sample-call";
import { camelCase, isArray } from "lodash";

function customAssert(condition: boolean, message?: string): void {
  if (!condition) {
    throw new Error(message);
  }
}

export interface ParsedCall {
  request: (string | Record<string, string>)[];
  response: Record<string, string>;
  succeeded: boolean;
}

function loadYamlFromNetwork(url: string): string {
  const request = new XMLHttpRequest();
  request.open("GET", url, false);
  request.send(null);
  return request.responseText;
}

function parseJsonRequest(request: string): Record<string, string> {
  const jsonRequest = JSON.parse(request);
  for (const key in jsonRequest) {
    const camelKey = camelCase(key);
    if (camelKey !== key) {
      jsonRequest[camelKey] = jsonRequest[key];
      delete jsonRequest[key];
    }
  }
  return jsonRequest;
}

export function parseSampleCall(sampleFile: string): ParsedCall {
  const sample_call_yaml =
    typeof process === "object"
      ? fs.readFileSync(sampleFile, "utf-8")
      : loadYamlFromNetwork(sampleFile);
  const sample_call_json = JSON.stringify(yaml.load(sample_call_yaml));
  const rawSample = Convert.toSampleCall(sample_call_json);

  customAssert(rawSample.request.length <= 2);
  const parsedRequest =
    rawSample.request.length === 2
      ? [rawSample.request[0], parseJsonRequest(rawSample.request[1])]
      : rawSample.request;
  const parsedSample: ParsedCall = {
    request: parsedRequest,
    response: JSON.parse(rawSample.response.message),
    // if response.success does not exist, then it defaults to true
    succeeded: rawSample.response.success !== false,
  };
  return parsedSample;
}

function stringify(obj: any): string {
  if (isArray(obj)) {
    const items = obj.map((item) => stringify(item));
    return `[${items.join(",")}]`;
  }
  if (obj.constructor === Object) {
    return JSON.stringify(obj, Object.keys(obj).sort());
  }
  return JSON.stringify(obj);
}

export class TauriInvokePlayback {
  unmatchedCalls: ParsedCall[];
  callPauseMs?: number;

  constructor() {
    this.unmatchedCalls = [];
  }

  mockCall(
    ...args: (string | Record<string, string>)[]
  ): Promise<Record<string, string>> {
    const jsonArgs = stringify(args);
    const stringifiedUnmatchedCalls = this.unmatchedCalls.map((call) =>
      stringify(call.request),
    );
    const matchingCallIndex = stringifiedUnmatchedCalls.findIndex(
      (call) => call === jsonArgs,
    );
    if (matchingCallIndex === -1) {
      const candidates = stringifiedUnmatchedCalls.join("\n");
      const errorMessage =
        `No matching call found for ${jsonArgs}.\n` +
        `Candidates are ${candidates}`;
      if (typeof process === "object") {
        console.error(errorMessage);
        throw new Error(errorMessage);
      } else {
        return Promise.reject(errorMessage);
      }
    }
    const matchingCall = this.unmatchedCalls[matchingCallIndex];
    this.unmatchedCalls.splice(matchingCallIndex, 1);

    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (matchingCall.succeeded) {
          resolve(matchingCall.response);
        } else {
          reject(matchingCall.response);
        }
      }, this.callPauseMs || 0);
    });
  }

  addCalls(...calls: ParsedCall[]): void {
    this.unmatchedCalls.push(...calls);
  }

  addSamples(...sampleFiles: string[]): void {
    const calls = sampleFiles.map((filename) => parseSampleCall(filename));
    this.addCalls(...calls);
  }
}
