<script>
	import { onMount } from 'svelte';

	let isCapturing = false;

	async function checkStatus() {
		try {
			const response = await fetch('http://127.0.0.1:8000/status');
			const data = await response.text();
			isCapturing = data.includes('true');
		} catch (error) {
			console.error('Error checking status:', error);
		}
	}

	async function toggleCapture() {
		try {
			const response = await fetch('http://127.0.0.1:8000/capture/start', {
				method: 'POST'
			});
			if (response.ok) {
				await checkStatus(); // Refresh status after toggle
			}
		} catch (error) {
			console.error('Error toggling capture:', error);
		}
	}

	async function stopCapture() {
		try {
			const response = await fetch('http://127.0.0.1:8000/capture/stop', {
				method: 'POST'
			});
			if (response.ok) {
				await checkStatus(); // Refresh status after stopping
			}
		} catch (error) {
			console.error('Error stopping capture:', error);
		}
	}

	// Check status when component mounts
	onMount(() => {
		checkStatus();
	});
</script>

<div class="flex flex-col items-start gap-4 rounded-lg bg-white p-4 shadow">
	<div class="flex items-center gap-2">
		<div class="text-sm font-medium">Capture Status:</div>
		<div class="flex items-center gap-2">
			<div class={`h-2 w-2 rounded-full ${isCapturing ? 'bg-green-500' : 'bg-red-500'}`}></div>
			<span class="text-sm">{isCapturing ? 'Active' : 'Inactive'}</span>
		</div>
	</div>

	{#if !isCapturing}
		<button
			type="button"
			class="rounded bg-indigo-600 px-2 py-1 text-xs font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
			on:click={toggleCapture}
		>
			Start Capture
		</button>
	{:else}
		<button
			type="button"
			class="inline-flex items-center gap-x-1.5 rounded-md bg-red-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600"
			on:click={stopCapture}
		>
			Stop Capture
			<svg
				class="-mr-0.5 size-5"
				viewBox="0 0 20 20"
				fill="currentColor"
				aria-hidden="true"
				data-slot="icon"
			>
				<path
					d="M5.25 3A2.25 2.25 0 003 5.25v9.5A2.25 2.25 0 005.25 17h9.5A2.25 2.25 0 0017 14.75v-9.5A2.25 2.25 0 0014.75 3h-9.5z"
				/>
			</svg>
		</button>
	{/if}
</div>
