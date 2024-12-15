<script lang="ts">
	import { onMount } from 'svelte';

	interface Address {
		address: string;
		netmask: string | null;
		broadcast: string | null;
	}

	interface Device {
		name: string;
		description: string | null;
		addresses: Address[];
		is_up: boolean;
		is_running: boolean;
		is_wireless: boolean;
	}

	interface ParsedDevice {
		name: string;
		addresses: string[];
		status: string;
		isWireless: boolean;
		isSelected?: boolean;
	}

	let interfaces: ParsedDevice[] = [];
	let error: string | null = null;
	let selectedDevice: string | null = null;

	async function fetchInterfaces() {
		try {
			const response = await fetch('http://127.0.0.1:8000/interfaces');
			const devices: Device[] = await response.json();

			interfaces = devices.map((device): ParsedDevice => {
				const status = device.is_up && device.is_running ? 'Connected' : 'Disconnected';

				return {
					name: device.name,
					addresses: device.addresses.map((addr) => addr.address),
					status,
					isWireless: device.is_wireless,
					isSelected: device.name === selectedDevice
				};
			});
		} catch (err) {
			error = 'Failed to fetch interfaces';
			console.error('Error:', err);
		}
	}

	async function selectDevice(deviceName: string) {
		try {
			const response = await fetch('http://127.0.0.1:8000/device/select', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ name: deviceName })
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}

			selectedDevice = deviceName;
			interfaces = interfaces.map((iface) => ({
				...iface,
				isSelected: iface.name === deviceName
			}));
		} catch (err) {
			error = 'Failed to select device';
			console.error('Error:', err);
		}
	}

	onMount(() => {
		fetchInterfaces();
	});
</script>

<div class="flex flex-col gap-4 rounded-lg bg-white p-4 shadow">
	<h2 class="text-lg font-semibold">Network Interfaces</h2>

	{#if error}
		<div class="text-red-500">{error}</div>
	{/if}

	<div class="grid gap-4">
		{#each interfaces as iface}
			<div class="rounded-md border p-3">
				<div class="flex items-center justify-between">
					<div class="font-medium">{iface.name}</div>
					<div class="flex items-center gap-2">
						<div
							class={`h-2 w-2 rounded-full ${
								iface.status === 'Connected'
									? 'bg-green-500'
									: iface.status === 'Disconnected'
										? 'bg-red-500'
										: 'bg-gray-500'
							}`}
						/>
						<span class="text-sm">{iface.status}</span>
						<button
							on:click={() => selectDevice(iface.name)}
							disabled={selectedDevice !== null}
							class={`ml-2 rounded-md px-3 py-1 text-sm text-white focus:outline-none focus:ring-2 focus:ring-offset-2 ${
								iface.isSelected
									? 'cursor-default bg-green-500'
									: selectedDevice
										? 'cursor-not-allowed bg-gray-400'
										: 'bg-blue-500 hover:bg-blue-600 focus:ring-blue-500'
							}`}
						>
							{iface.isSelected ? 'Selected' : 'Select'}
						</button>
					</div>
				</div>
				{#if iface.addresses.length > 0}
					<div class="mt-2 text-sm text-gray-600">
						<div class="font-medium">Addresses:</div>
						<ul class="ml-4 list-disc">
							{#each iface.addresses as address}
								<li>{address}</li>
							{/each}
						</ul>
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>
