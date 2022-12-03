l = $('*').innerText.split`\n`
	.map((x) => [x.slice(0, x.length / 2), x.slice(x.length / 2)])
	.filter((x) => x[0].length > 1)
	.reduce((a, c) => {
		const x = c[0].split``.filter((x) => c[1].includes(x))[0];
		return a + x.charCodeAt(0) - (x.toUpperCase() === x ? 39 : 97) + 1;
	}, 0);

l = $('*').innerText.split`\n`
	.filter((x) => x.length > 1)
	.reduce((a, c, i, arr) => {
		if (i % 3 === 0) {
			const x = arr[i].split``.filter((x) => arr[i + 1].includes(x) && arr[i + 2].includes(x))[0];
			return a + x.charCodeAt(0) - (x.toUpperCase() === x ? 39 : 97) + 1;
		}
		return a;
	}, 0);
