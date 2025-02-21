<script lang="ts">
  import { goto } from "$app/navigation";
  import DispatchButton from "$lib/components/DispatchButton.svelte";
  import IndustrialSwitch from "$lib/components/IndustrialSwitch.svelte";
  import KeySwitch from "$lib/components/KeySwitch.svelte";
  import EmergencyButton from "$lib/components/EmergencyButton.svelte";

  async function sendRequest(event: any) {
    let data: PostData = {
      action: event.detail,
      connectionTest: false,
    };

    try {
      await fetch("/", {
        method: "POST",
        body: JSON.stringify(data),
        headers: { "Content-Type": "application/json" }
      });
    } catch {
      alert("Couldn't connect to the server");
      await goto("/");
    }
  }

  //////////////////////////////////////////////////////////////////

  let dispatch1_state = false;
  let dispatch2_state = false;
  let dispatch_running = false;
  let panel_state = false;

  async function checkDispatch() {
    // cancel if both dispatch buttons aren't activated
    if (!dispatch1_state || !dispatch2_state) return;

    // prevent multiple requests with one dispatch
    if (!dispatch_running) await sendRequest({ detail: "dispatch" });
    dispatch_running = true;

    // turn off light after timeout
    setTimeout(() => {
      dispatch1_state = false;
      dispatch2_state = false;
      dispatch_running = false;
    }, 3500);
  }
</script>

<div class="controlPanelRoute">
  <DispatchButton
    button_number="1"
    {dispatch_running}
    bind:button_state={dispatch1_state}
    {panel_state}
    on:checkDispatch={checkDispatch}
  />

  <DispatchButton
    button_number="2"
    {dispatch_running}
    bind:button_state={dispatch2_state}
    {panel_state}
    on:checkDispatch={checkDispatch}
  />

  <IndustrialSwitch
    type="Harness"
    left_state="harnessOpen"
    right_state="harnessClose"
    {panel_state}
    on:request={sendRequest}
  />

  <IndustrialSwitch
    type="Gates"
    left_state="gatesOpen"
    right_state="gatesClose"
    {panel_state}
    on:request={sendRequest}
  />

  <IndustrialSwitch
    type="Platform"
    open="RAISE"
    close="LOWER"
    left_state="platformRaise"
    right_state="platformLower"
    {panel_state}
    on:request={sendRequest}
  />

  <IndustrialSwitch
    type="Car"
    open="UNLOCK"
    close="LOCK"
    left_state="carUnlock"
    right_state="carLock"
    {panel_state}
    on:request={sendRequest}
  />

  <KeySwitch bind:panel_state />
  <EmergencyButton {panel_state} on:request={sendRequest} />
</div>

<svelte:head>
  <style>
    .controlPanelRoute * {
      user-select: none;
      -webkit-user-select: none;
    }
  </style>
</svelte:head>

<style>
  .controlPanelRoute {
    margin: 0;
    padding: 0;
    height: 100vh;
    width: 100vw;
    background-color: #f1f1f1;
    position: relative;
  }
</style>
