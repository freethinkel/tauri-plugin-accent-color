<button class="btn">
  <slot/>
</button>

<style>
  .btn {
    display: flex;
    align-items: center;
    appearance: none;
    background: var(--color-accent);
    border-radius: var(--border-radius);
    font-size: 1rem;
    height: 36px;
    border: none;
    font-weight: 600;
    padding: 0 12px;
  }
</style>
