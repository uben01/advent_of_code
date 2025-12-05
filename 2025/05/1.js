const fs = require('node:fs');

fs.readFile('input.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    const ranges = [];

    let readingRanges = true;
    let count = 0;

    const lines = data.trim().split('\n');
    for (const line of lines) {
        if (line === '') {
            readingRanges = false;
            continue;
        }

        if (readingRanges) {
            ranges.push({
                start: Number.parseInt(line.split('-')[0], 10),
                end: Number.parseInt(line.split('-')[1], 10)
            })
        } else {
            const num = Number.parseInt(line, 10);
            for (const range of ranges) {
                if (num >= range.start && num <= range.end) {
                    count++;
                    break;
                }
            }
        }
    }

    console.log(count);
});
