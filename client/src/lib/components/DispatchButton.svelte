<script lang="ts">
  import dispatch_off from "$lib/assets/greenBtn_off.png";
  import dispatch_on from "$lib/assets/greenBtn_on.png";
  import { getBase64FromUrl } from "$lib/globals";
  import { createEventDispatcher, onMount } from "svelte";

  // preload images to stop flickering when toggling
  const images = new Array(2);
  onMount(async () => {
    images[0] = await getBase64FromUrl(dispatch_off);
    images[1] = await getBase64FromUrl(dispatch_on);
  });

  export let button_number: string;
  export let button_state: boolean; // bind on parent
  export let dispatch_running: boolean;
  export let panel_state: boolean;

  const dispatch = createEventDispatcher();
  function click() {
    if (dispatch_running || !panel_state) return;
    button_state = !button_state;
    dispatch("checkDispatch");
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="dispatchBtn"
  id="dispatchBtn{button_number}"
  style="background-image: url({button_state
    ? images[1]
      ? images[1]
      : ''
    : images[0]
      ? images[0]
      : ''});"
  on:click={click}
>
  <!-- crazy image preloading magic above :o-->

  <h1 class="dispatchTitle" id="dispatchTitle{button_number}">DISPATCH</h1>
</div>

<style>
  .dispatchBtn {
    background: url("$lib/assets/greenBtn_off.png");
    background-position: center;
    background-repeat: no-repeat;
    background-size: contain;
    width: 12%;
    height: 24%;
    position: absolute;
    -webkit-tap-highlight-color: transparent;
    cursor: pointer;
  }

  .dispatchTitle {
    font-size: 1.3vw;
    text-align: center;
    transform: translate(0, -180%);
  }

  #dispatchBtn1 {
    top: 70%;
    left: 5%;
  }

  #dispatchBtn2 {
    top: 70%;
    right: 5%;
  }
</style>
