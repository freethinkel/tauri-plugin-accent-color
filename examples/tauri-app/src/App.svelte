<script>
  import { accentColor } from "tauri-plugin-accent-color";
  import Button from "./lib/Button.svelte";

  accentColor.subscribe((color) => {
    document.documentElement.style.setProperty("--color-accent", color);
  });
</script>

<main class="container">
  <h1>Accent color</h1>
  <svg class="crcl">
    <circle x="0" y="0" r="50" fill="var(--color-accent)" />
  </svg>

  <Button>Button</Button>
</main>

<style>
  .crcl {
    position: absolute;
  }
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
  }
  h1 {
    text-align: center;
    margin: 0 0 24px;
  }
</style>
