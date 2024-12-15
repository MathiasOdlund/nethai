<script>
	import { onMount } from 'svelte';

	let interfaces = [];
	let error = null;

	async function fetchInterfaces() {
		try {
			const response = await fetch('http://127.0.0.1:8000/interfaces');
			const data = await response.text();

			// Parse the text response to extract device information
			const devicesMatch = data.match(/Available devices: \[(.*)\]/);
			if (devicesMatch) {
				// Split the devices string and process each device
				const devicesStr = devicesMatch[1];
				const devicesList = devicesStr
					.split('Device {')
					.filter((d) => d.trim())
					.map((d) => {
						const nameMatch = d.match(/name: "([^"]+)"/);
						const addressesMatch = d.match(/addresses: \[(.*?)\]/);
						const flagsMatch = d.match(
							/flags: DeviceFlags \{ if_flags: ([^,]+), connection_status: ([^\s]+) \}/
						);

						const addresses = addressesMatch
							? addressesMatch[1]
									.split('Address {')
									.filter((a) => a.trim())
									.map((a) => {
										const addrMatch = a.match(/addr: ([^,]+)/);
										return addrMatch ? addrMatch[1] : null;
									})
									.filter((a) => a)
							: [];

						return {
							name: nameMatch ? nameMatch[1] : 'Unknown',
							addresses: addresses,
							status: flagsMatch ? flagsMatch[2] : 'Unknown',
							flags: flagsMatch ? flagsMatch[1] : ''
						};
					});
				interfaces = devicesList;
			}
		} catch (err) {
			error = 'Failed to fetch interfaces';
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
						></div>
						<span class="text-sm">{iface.status}</span>
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
