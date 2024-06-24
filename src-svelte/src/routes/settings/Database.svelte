<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import { importDb, exportDb, type DatabaseCounts } from "$lib/bindings";
  import { snackbarInfo, snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Button from "$lib/controls/Button.svelte";
  import ButtonGroup from "$lib/controls/ButtonGroup.svelte";
  import Warning from "$lib/Warning.svelte";
  import { save, open, type DialogFilter } from "@tauri-apps/api/dialog";

  const ZAMM_DB_FILTER: DialogFilter = {
    name: "ZAMM Database",
    extensions: ["zamm.yaml"],
  };

  function nounify(counts: DatabaseCounts): string {
    let noun: string;
    if (counts.num_api_keys > 0 && counts.num_llm_calls === 0) {
      noun = "API key";
    } else if (counts.num_api_keys === 0 && counts.num_llm_calls > 0) {
      noun = "LLM call";
    } else {
      noun = "item";
    }

    const total = counts.num_api_keys + counts.num_llm_calls;
    const nounified = `${total} ${noun}`;
    return total === 0 || total > 1 ? nounified + "s" : nounified;
  }

  async function importData() {
    const filePath = await open({
      title: "Import ZAMM data",
      directory: false,
      multiple: false,
      filters: [ZAMM_DB_FILTER, { name: "All Files", extensions: ["*"] }],
    });
    if (filePath === null) {
      return;
    }

    try {
      if (filePath instanceof Array) {
        throw new Error("More than one file selected");
      }

      const importCounts = await importDb(filePath);
      const importMessage = `Imported ${nounify(importCounts.imported)}`;
      if (
        importCounts.ignored.num_api_keys > 0 ||
        importCounts.ignored.num_llm_calls > 0
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
      const exportCounts = await exportDb(filePath);
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
