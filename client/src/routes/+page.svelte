<script lang="ts">
  import logo from "$lib/assets/NL2-Control-Panel-Logo.png";
  import arrow from "$lib/assets/arrow_right.svg";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { PORT, HOSTNAME_STORAGE_KEY } from "$lib/globals";

  let hostname = "";
  onMount(() => {
    hostname = document.location.hostname;
  });

  async function connect() {
    let data: PostData = {
      action: "",
      connectionTest: true,
    };

    try {
      await fetch(`http://${hostname}:${PORT}`, {
        method: "POST",
        body: JSON.stringify(data),
        headers: { "Content-Type": "application/json" },
        mode: "cors",
      });

      sessionStorage.setItem(HOSTNAME_STORAGE_KEY, hostname);
      await goto("/panel");
    } catch {
      alert("Couldn't connect to server");
    }
  }
</script>

<div class="menuRoute">
  <div class="box">
    <img class="logo" src={logo} alt="NL2 Control Panel" />
    <div class="menu">
      <button on:click={connect}
        ><img src={arrow} alt="arrow" class="arrow" /></button
      >
    </div>
  </div>
</div>

<style>
  .menuRoute {
    margin: 0;
    padding: 0;
    height: 100vh;
    display: grid;
    place-items: center;
    background: #231f20;
  }

  .box {
    display: grid;
    place-items: center;
    translate: 0 -10%;
  }

  .logo {
    width: 90vw;
    height: auto;
  }

  .menu {
    margin-top: 5vh;
  }

  button {
    appearance: none;
    aspect-ratio: 1/1;
    width: 8vh;
    margin: 0;
    padding: 1rem;
    border: none;
    border-radius: 50%;
    outline: none;
    font-size: 1rem;
    position: relative;
    cursor: pointer;
    background-color: white;
  }

  .arrow {
    position: absolute;
    top: 50%;
    left: 50%;
    translate: -50% -50%;
    margin: 0;
    padding: 0;
    aspect-ratio: 1;
    width: 60%;
    height: auto;
  }

  @media only screen and (min-width: 700px) {
    .logo {
      height: 40vh;
      width: auto;
    }
    button {
      width: 8vmin;
    }
  }
</style>
