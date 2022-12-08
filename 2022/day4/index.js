const parse = $('*').innerText.split`\n`
	.filter((x) => x.length > 1)
	.map((line) => line.split(',').map((r) => r.split('-').map(Number)));

const part1 = parse
	.map(([a, b]) => [[a, b], [Math.min, Math.max].map((f, i) => f(a[i], b[i]))])
	.filter(([pair, range]) => pair.some((p) => p.every((v, i) => v === range[i]))).length;

const part2 = parse
	.map(([a, b]) => [
		[a, b],
		[b, a],
	])
	.filter((arr) => arr.some(([id, [start, end]]) => id.some((v) => v >= start && v <= end))).length;
