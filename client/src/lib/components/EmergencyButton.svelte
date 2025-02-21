<script lang="ts">
  import emergency_off from "$lib/assets/emergencyBtn_released.png";
  import emergency_on from "$lib/assets/emergencyBtn_pressed.png";
  import { createEventDispatcher, onMount } from "svelte";
  import { getBase64FromUrl } from "$lib/globals";

  // preload images to stop flickering when toggling
  const images = new Array(2);
  onMount(async () => {
    images[0] = await getBase64FromUrl(emergency_off);
    images[1] = await getBase64FromUrl(emergency_on);
  });

  export let panel_state: boolean;

  const dispatch = createEventDispatcher();
  let emergency_state = false;

  function click() {
    emergency_state = !emergency_state;
    if (!panel_state) return;
    dispatch("request", emergency_state ? "emergencyOn" : "emergencyOff");
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="emergencyBtn"
  style="background-image: url({emergency_state
    ? images[1]
      ? images[1]
      : ''
    : images[0]
      ? images[0]
      : ''});"
  on:click={click}
></div>

<!-- crazy image preloading magic above :o-->

<style>
  .emergencyBtn {
    background: url("$lib/assets/emergencyBtn_released.png");
    background-position: center;
    background-repeat: no-repeat;
    background-size: contain;
    width: 25%;
    height: 50%;
    position: absolute;
    top: 5%;
    right: 2.5%;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
  }
</style>
