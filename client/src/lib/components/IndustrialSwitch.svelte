<script lang="ts">
  import switch_off from "$lib/assets/switch_right.png";
  import switch_on from "$lib/assets/switch_left.png";
  import { createEventDispatcher, onMount } from "svelte";
  import { getBase64FromUrl } from "$lib/globals";

  // preload images to stop flickering when toggling
  const images = new Array(2);
  onMount(async () => {
    images[0] = await getBase64FromUrl(switch_off);
    images[1] = await getBase64FromUrl(switch_on);
  });

  export let type: string;
  export let left_state: Action;
  export let right_state: Action;
  export let panel_state: boolean;
  export let open: string = "OPEN";
  export let close: string = "CLOSE";

  const dispatch = createEventDispatcher();
  let switch_state = false;

  function click() {
    switch_state = !switch_state;
    if (!panel_state) return;
    dispatch("request", switch_state ? left_state : right_state);
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="switch"
  id="{type.toLowerCase()}Switch"
  style="background-image: url({switch_state
    ? images[1]
      ? images[1]
      : ''
    : images[0]
      ? images[0]
      : ''});"
  on:click={click}
>
  <!-- crazy image preloading magic above :o-->

  <h1 class="switchTitle" id="{type.toLowerCase()}Title">
    {type.toUpperCase()}
  </h1>

  <div class="subSigns">
    <p class="openSign" id="openSign{type}">
      {open}
    </p>
    <p class="closeSign" id="closeSign{type}">
      {close}
    </p>
  </div>
</div>

<style>
  .switch {
    background: url("$lib/assets/switch_right.png");
    background-position: center;
    background-repeat: no-repeat;
    background-size: contain;
    width: 12%;
    height: 24%;
    position: absolute;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
  }

  .switchTitle {
    font-size: 1.3vw;
    text-align: center;
    transform: translate(0, -200%);
  }

  .subSigns {
    display: flex;
    justify-content: center;
    transform: translate(0, -110%);
  }

  .openSign {
    font-size: 1.3vw;
    margin-right: 40%;
  }

  .closeSign {
    font-size: 1.3vw;
    margin-left: 40%;
  }

  #gatesSwitch {
    top: 20%;
    left: 52.5%;
  }

  #harnessSwitch {
    top: 20%;
    left: 27.5%;
  }

  #platformSwitch {
    top: 55%;
    left: 27.5%;
  }

  #carSwitch {
    top: 55%;
    left: 52.5%;
  }
</style>
