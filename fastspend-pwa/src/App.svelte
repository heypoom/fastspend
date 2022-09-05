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
      class="focus:outline-none bg-neutral-900 text-white px-4 py-4 text-3xl font-extralight rounded-md shadow-xl text-center focus:shadow-blue-300 focus:shadow-xl focus:ring-4 focus:ring-offset-4 focus:ring-blue-400 focus:ring-opacity-50 w-full max-w-xs"
      class:error-input={error}
      class:loading-input={loading}
      bind:value={command}
      on:keypress={(e) => e.key === 'Enter' && submit()}
      autofocus
    />

    {#if error}
      <div class="text-red-500">{error}</div>
    {/if}
  </div>
</div>
