<script lang="ts">
  let command = ''
  let loading = false
  let error: string | null = null

  async function submit() {
    loading = true

    try {
      await fetch('https://api.fastspend.poom.dev/command', {
        method: 'POST',
        body: JSON.stringify({command}),
      })

      error = null
    } catch (err) {
      error = err.message
    } finally {
      command = ''
      loading = false
    }
  }
</script>

<div
  class="flex flex-col items-center justify-center min-h-screen bg-stone-100 space-y-4"
>
  <input
    type="text"
    class="bg-neutral-900 text-white px-4 py-4 text-3xl font-extralight rounded-md shadow-2xl text-center"
    class:error-input={error}
    class:loading-input={loading}
    bind:value={command}
    on:keypress={(e) => e.key === 'Enter' && submit()}
  />

  {#if error}
    <div class="text-red-500">Error: {error}</div>
  {/if}
</div>
