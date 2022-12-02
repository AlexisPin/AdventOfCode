const part1_points = {
	A: {
		X: 4,
		Y: 8,
		Z: 3,
	},
	B: {
		X: 1,
		Y: 5,
		Z: 9,
	},
	C: {
		X: 7,
		Y: 2,
		Z: 6,
	},
};

const part2_points = {
	A: {
		X: 3,
		Y: 4,
		Z: 8,
	},
	B: {
		X: 1,
		Y: 5,
		Z: 9,
	},
	C: {
		X: 2,
		Y: 6,
		Z: 7,
	},
};

//Part 1
l = $('*').innerText.split`\n`
	.map((x) => x.split` `)
	.filter((x) => x.length > 1)
	.reduce((a, c) => a + part1_points[c[0]][c[1]], 0);

//Part 2
l = $('*').innerText.split`\n`
	.map((x) => x.split` `)
	.filter((x) => x.length > 1)
	.reduce((a, c) => a + part2_points[c[0]][c[1]], 0);
