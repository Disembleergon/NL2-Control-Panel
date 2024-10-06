<script lang="ts">
  import key_left from "$lib/assets/key_left.png";
  import key_right from "$lib/assets/key_right.png";
  import { getBase64FromUrl } from "$lib/globals";
  import { onMount } from "svelte";

  // preload images to stop flickering when toggling
  const images = new Array(2);
  onMount(async () => {
    images[0] = await getBase64FromUrl(key_left);
    images[1] = await getBase64FromUrl(key_right);
  });

  export let panel_state: boolean;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  class="key"
  style="background-image: url({panel_state
    ? images[1]
      ? images[1]
      : ''
    : images[0]
      ? images[0]
      : ''});"
  on:click={() => (panel_state = !panel_state)}
>
  <!-- crazy image preloading magic above :o-->

  <h1>POWER</h1>
</div>

<style>
  .key {
    background: url("$lib/assets/key_left.png");
    background-position: center;
    background-repeat: no-repeat;
    background-size: contain;
    width: 12%;
    height: 24%;
    position: absolute;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
    left: 5%;
    top: 20%;
  }

  .key h1 {
    font-size: 1.3vw;
    text-align: center;
    transform: translate(0, -180%);
  }
</style>
