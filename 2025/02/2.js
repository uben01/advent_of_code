const fs = require('node:fs');

const re = /^(\d+)\1+$/;

fs.readFile('input.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    const lines = data.trim().split('\n');
    const splitLines = lines.map(line => line.split(','));

    let sum = 0;
    for (const ranges of splitLines) {
        for (const range of ranges) {
            const [start, end] = range.split('-');

            if (end === undefined) {
                continue;
            }

            for (let i = parseInt(start); i <= end; i++) {
                if (re.test(i.toString())) {
                    sum += i;
                }
            }
        }
    }

    console.log(sum);
});
