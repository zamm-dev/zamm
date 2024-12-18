<script lang="ts">
  import InfoBox from "$lib/InfoBox.svelte";
  import Loading from "$lib/Loading.svelte";
  import { commands, type OS } from "$lib/bindings";
  import { systemInfo } from "$lib/system-info";
  interface Props {
    [key: string]: any;
  }

  let { ...rest }: Props = $props();

  let systemInfoCall = commands.getSystemInfo();
  systemInfoCall
    .then((result) => {
      systemInfo.set(result);
    })
    .catch((error) => {
      console.error(`Could not retrieve system info: ${error}`);
    });

  function formatOsString(os: OS | null | undefined) {
    if (os === "Mac") {
      return "Mac OS";
    }
    return os ?? "Unknown";
  }

  let os = $derived(formatOsString($systemInfo?.os));
</script>

<div class="container">
  <InfoBox title="System Info" {...rest}>
    {#await systemInfoCall}
      <Loading />
    {:then systemInfo}
      <table>
        <tbody>
          <tr>
            <th colspan="2">ZAMM</th>
          </tr>
          <tr>
            <td>Version</td>
            <td class="version-value">{systemInfo.zamm_version}</td>
          </tr>
          <tr>
            <td>Stability</td>
            <td class="stability-value">Unstable (Alpha)</td>
          </tr>
          <tr>
            <td>Fork</td>
            <td>Original</td>
          </tr>
        </tbody>
      </table>

      <table class="less-space">
        <tbody>
          <tr>
            <th colspan="2">Computer</th>
          </tr>
          <tr>
            <td>OS</td>
            <td>{os}</td>
          </tr>
          <tr>
            <td>Shell</td>
            <td>{systemInfo.shell ?? "Unknown"}</td>
          </tr>
        </tbody>
      </table>
    {:catch error}
      <span role="status">error: {error}</span>
    {/await}
  </InfoBox>
</div>

<style>
  .container {
    display: inline-block;
    width: fit-content;
  }

  table {
    margin-top: 0.5rem;
  }

  th,
  td {
    text-align: left;
    padding-left: 0;
  }

  td {
    vertical-align: text-top;
  }

  td:first-child {
    color: var(--color-faded);
    padding-right: 1rem;
  }

  .stability-value {
    color: var(--color-caution);
  }

  .less-space {
    margin-bottom: -0.33rem;
  }
</style>
