<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import { commands, type DatabaseCounts } from "$lib/bindings";
  import { unwrap } from "$lib/tauri";
  import { snackbarInfo, snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import { systemInfo } from "$lib/system-info";
  import Button from "$lib/controls/Button.svelte";
  import ButtonGroup from "$lib/controls/ButtonGroup.svelte";
  import Warning from "$lib/Warning.svelte";
  import { save, open, type DialogFilter } from "@tauri-apps/plugin-dialog";

  const ZAMM_DB_FILTER: DialogFilter = {
    name: "ZAMM Database",
    extensions: ["zamm.yaml"],
  };

  const MAC_ZAMM_DB_FILTER: DialogFilter = {
    name: "ZAMM Database",
    extensions: ["yaml"],
  };

  interface DefinedDatabaseCounts {
    num_api_keys: number;
    num_llm_calls: number;
    num_terminal_sessions: number;
  }

  function substituteUndefinedCounts(
    counts: DatabaseCounts | undefined,
  ): DefinedDatabaseCounts {
    if (counts === undefined) {
      return {
        num_api_keys: 0,
        num_llm_calls: 0,
        num_terminal_sessions: 0,
      };
    }

    return {
      num_api_keys: counts.num_api_keys ?? 0,
      num_llm_calls: counts.num_llm_calls ?? 0,
      num_terminal_sessions: counts.num_terminal_sessions ?? 0,
    };
  }

  function nounify(counts: DatabaseCounts | undefined): string {
    if (counts === undefined) {
      return "0 items";
    }

    let noun: string;
    const itemTypes = Object.keys(counts).length;
    const definedCounts = substituteUndefinedCounts(counts);
    if (itemTypes === 0 || itemTypes > 1) {
      noun = "item";
    } else if (definedCounts.num_api_keys > 0) {
      noun = "API key";
    } else if (definedCounts.num_llm_calls > 0) {
      noun = "LLM call";
    } else {
      if (definedCounts.num_terminal_sessions === 0) {
        console.error("Unexpected terminal session count");
      }
      noun = "terminal session";
    }

    const total =
      definedCounts.num_api_keys +
      definedCounts.num_llm_calls +
      definedCounts.num_terminal_sessions;
    const nounified = `${total} ${noun}`;
    return total === 0 || total > 1 ? nounified + "s" : nounified;
  }

  async function importData() {
    const defaultImportFilter =
      $systemInfo?.os === "Mac" ? MAC_ZAMM_DB_FILTER : ZAMM_DB_FILTER;
    const filePath =
      window.WEBDRIVER_FILE_PATH ??
      (await open({
        title: "Import ZAMM data",
        directory: false,
        multiple: false,
        filters: [
          defaultImportFilter,
          { name: "All Files", extensions: ["*"] },
        ],
      }));
    if (filePath === null) {
      return;
    }

    try {
      if (filePath instanceof Array) {
        throw new Error("More than one file selected");
      }

      const importCounts = await unwrap(commands.importDb(filePath));
      const importMessage = `Imported ${nounify(importCounts.imported)}`;

      const ignoredImports = substituteUndefinedCounts(importCounts.ignored);
      if (
        ignoredImports.num_api_keys > 0 ||
        ignoredImports.num_llm_calls > 0 ||
        ignoredImports.num_terminal_sessions > 0
      ) {
        snackbarInfo(
          `${importMessage}, ignored ${nounify(importCounts.ignored)}`,
        );
      } else {
        snackbarInfo(importMessage);
      }
    } catch (error) {
      snackbarError(error as string | Error);
    }
  }

  async function exportData() {
    const filePath = await save({
      title: "Export ZAMM data",
      filters: [ZAMM_DB_FILTER],
    });
    if (filePath === null) {
      return;
    }

    try {
      const exportCounts = await unwrap(commands.exportDb(filePath));
      snackbarInfo(`Exported ${nounify(exportCounts)}`);
    } catch (error) {
      snackbarError(error as string | Error);
    }
  }
</script>

<InfoBox title="Data" childNumber={1}>
  <Warning
    >Exported files will contain <strong>sensitive information</strong> such as API
    keys and all correspondence with LLMs.</Warning
  >
  <ButtonGroup>
    <Button unwrapped leftEnd on:click={importData}>Import data</Button>
    <Button unwrapped rightEnd on:click={exportData}>Export data</Button>
  </ButtonGroup>
</InfoBox>
