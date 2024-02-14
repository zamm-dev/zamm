import { type ChildProcess, spawn, spawnSync } from "child_process";
import { join } from "path";

// keep track of the `tauri-driver` child process
let tauriDriver: ChildProcess;

exports.config = {
  specs: ["./test/specs/**/*.js"],
  maxInstances: 1,
  hostname: "localhost",
  port: 4444,
  capabilities: [
    {
      browserName: "wry",
      maxInstances: 1,
      "tauri:options": {
        application: "../src-tauri/target/release/zamm",
      },
    },
  ],
  reporters: ["spec"],
  framework: "mocha",
  mochaOpts: {
    ui: "bdd",
    timeout: 10000,
  },

  // ensure the rust project is built since we expect this binary to exist for the
  // webdriver sessions
  onPrepare: () => spawnSync("cargo", ["build", "--release"]),

  // ensure we are running `tauri-driver` before the session starts so that we can
  // proxy the webdriver requests
  beforeSession: () =>
    (tauriDriver = spawn("tauri-driver", [], {
      stdio: [null, process.stdout, process.stderr],
    })),

  // clean up the `tauri-driver` process we spawned at the start of the session
  afterSession: () => tauriDriver.kill(),

  services: [
    [
      "image-comparison",
      {
        baselineFolder: join(process.cwd(), "./screenshots/baseline/"),
        formatImageName: "{tag}-{width}x{height}",
        screenshotPath: join(process.cwd(), "./screenshots/testing/"),
        savePerInstance: true,
        autoSaveBaseline: true,
        blockOutStatusBar: true,
        blockOutToolBar: true,
        isHybridApp: true,
      },
    ],
  ],
};
