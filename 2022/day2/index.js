l = $('*').innerText.split`\n`
	.map((x) => x.split` `)
	.reduce(
		(a, c) =>
			a +
			(c[0] == 'A' && c[1] == 'X'
				? 4
				: c[0] == 'A' && c[1] == 'Y'
				? 8
				: c[0] == 'A' && c[1] == 'Z'
				? 3
				: c[0] == 'B' && c[1] == 'X'
				? 1
				: c[0] == 'B' && c[1] == 'Y'
				? 5
				: c[0] == 'B' && c[1] == 'Z'
				? 9
				: c[0] == 'C' && c[1] == 'X'
				? 7
				: c[0] == 'C' && c[1] == 'Y'
				? 2
				: c[0] == 'C' && c[1] == 'Z'
				? 6
				: 0),
		0,
	);

l = $('*').innerText.split`\n`
	.map((x) => x.split` `)
	.reduce(
		(a, c) =>
			a +
			(c[0] == 'A' && c[1] == 'X'
				? 3
				: c[0] == 'A' && c[1] == 'Y'
				? 4
				: c[0] == 'A' && c[1] == 'Z'
				? 8
				: c[0] == 'B' && c[1] == 'X'
				? 1
				: c[0] == 'B' && c[1] == 'Y'
				? 5
				: c[0] == 'B' && c[1] == 'Z'
				? 9
				: c[0] == 'C' && c[1] == 'X'
				? 2
				: c[0] == 'C' && c[1] == 'Y'
				? 6
				: c[0] == 'C' && c[1] == 'Z'
				? 7
				: 0),
		0,
	);
