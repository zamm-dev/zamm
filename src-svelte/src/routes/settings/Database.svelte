<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import { importDb, exportDb } from "$lib/bindings";
  import { snackbarError } from "$lib/snackbar/Snackbar.svelte";
  import Button from "$lib/controls/Button.svelte";
  import ButtonGroup from "$lib/controls/ButtonGroup.svelte";
  import Warning from "$lib/Warning.svelte";
  import { save, open, type DialogFilter } from "@tauri-apps/api/dialog";

  const ZAMM_DB_FILTER: DialogFilter = {
    name: "ZAMM Database",
    extensions: ["zamm.yaml"],
  };

  async function importData() {
    const filePath = await open({
      title: "Import ZAMM data",
      directory: false,
      multiple: false,
      filters: [ZAMM_DB_FILTER, { name: "All Files", extensions: ["*"] }],
    });

    try {
      if (filePath === null) {
        return;
      }

      if (filePath instanceof Array) {
        throw new Error("More than one file selected");
      }

      await importDb(filePath);
    } catch (error) {
      snackbarError(error as string | Error);
    }
  }

  async function exportData() {
    const filePath = await save({
      title: "Export ZAMM data",
      filters: [ZAMM_DB_FILTER],
    });

    try {
      if (filePath === null) {
        return;
      }

      exportDb(filePath);
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
