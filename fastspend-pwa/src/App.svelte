<script>
  let command = ''

  async function submit() {
    const res = await fetch('https://api.fastspend.poom.dev/command', {
      method: 'POST',
      body: JSON.stringify({command}),
    })

    console.log(res.status)
  }
</script>

<div class="flex flex-col items-center justify-between v-fill bg-stone-100">
  <div>A</div>
  <div class="w-full">
    <input
      type="text"
      class="w-full bg-neutral-900 text-white px-4 py-4 text-3xl font-extralight rounded-none"
      bind:value={command}
      on:keypress={(e) => e.key === 'Enter' && submit()}
    />
  </div>
</div>
