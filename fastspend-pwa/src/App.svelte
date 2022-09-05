<script lang="ts">
  let command = ''
  let loading = false
  let error: string | null = null

  async function submit() {
    loading = true

    try {
      const res = await fetch('https://api.fastspend.poom.dev/command', {
        method: 'POST',
        body: JSON.stringify({command}),
      })

      if (res.status !== 200) throw new Error(await res.text())

      error = null
    } catch (err) {
      error = err.message
    } finally {
      command = ''
      loading = false
    }
  }
</script>

<div class="bg-stone-100">
  <div
    class="flex flex-col items-center justify-center min-h-screen space-y-4 mx-4"
  >
    <input
      type="text"
      class="bg-neutral-900 text-white px-4 py-4 text-3xl font-extralight rounded-md shadow-2xl text-center focus-visible:outline-blue-500 focus-visible:outline-offset-4 w-full"
      class:error-input={error}
      class:loading-input={loading}
      bind:value={command}
      on:keypress={(e) => e.key === 'Enter' && submit()}
    />

    {#if error}
      <div class="text-red-500">{error}</div>
    {/if}
  </div>
</div>
