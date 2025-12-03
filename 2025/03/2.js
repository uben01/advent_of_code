const fs = require('node:fs');

fs.readFile('input.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    let sum = 0;

    const lines = data.trim().split('\n');
    for (const line of lines) {


        let maxes = [];

        for (let k = 0; k < 12; k++) {
            let lastI = maxes.at(-1)?.index === undefined ? -1 : maxes.at(-1)?.index;

            let max = -1;
            let maxI = -1;
            for (let i = lastI + 1; i < line.length - (12 - k) + 1; i++) {
                const num = parseInt(line[i]);
                if (num > max) {
                    max = num;
                    maxI = i;
                }
            }
            maxes.push({"max": max, "index": maxI});
        }

        const result = maxes.map(x => x.max).join('');
        sum += parseInt(result);
    }

    console.log(sum);
});
