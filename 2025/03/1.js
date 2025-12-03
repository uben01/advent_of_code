const fs = require('node:fs');

fs.readFile('input.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    let sum = 0;

    const lines = data.trim().split('\n');
    for (const line of lines) {
        let max = 0;
        let maxI = 0;
        let secMax = 0;

        for (let i = 0; i < line.length - 1; i++) {
            const num = parseInt(line[i]);
            if (num > max) {
                max = num;
                maxI = i;
            }
        }
        for (let i = maxI+1; i < line.length; i++) {
            const num = parseInt(line[i]);
            if (num > secMax) {
                secMax = num;
            }
        }

        let subSum = (max * 10) + secMax;
        sum += subSum;
    }
    console.log(sum);
});
