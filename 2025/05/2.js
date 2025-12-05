const fs = require('node:fs');

fs.readFile('input.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    let ranges = [];

    const lines = data.trim().split('\n');
    for (const line of lines) {
        if (line === '') {
            break;
        }
        const start = Number.parseInt(line.split('-')[0], 10);
        const end = Number.parseInt(line.split('-')[1], 10)

        ranges.push({start, end});
    }

    let didMerge = true;
    while (didMerge) {
        didMerge = false;
        for (let i = 0; i < ranges.length; i++) {
            for (let j = 0; j < ranges.length; j++) {
                const currentRange = ranges[i];
                const compareRange = ranges[j];

                if (currentRange === null || compareRange === null) {
                    continue;
                }

                if (currentRange === compareRange) {
                    continue;
                }

                if (currentRange.start > compareRange.end || currentRange.end < compareRange.start) {
                    continue;
                }

                if (currentRange.start >= compareRange.start && currentRange.end <= compareRange.end) {
                    ranges[i] = null;
                    didMerge = true;
                    continue;
                }

                if (currentRange.start <= compareRange.start && currentRange.end <= compareRange.end) {
                    ranges[i].end = compareRange.end;
                    ranges[j] = null;
                    didMerge = true;
                    continue;
                }

                if (currentRange.start >= compareRange.start && currentRange.end >= compareRange.end) {
                    ranges[i].start = compareRange.start;
                    ranges[j] = null;
                    didMerge = true;
                }
            }
        }
    }

    ranges = ranges.filter(r => r !== null);

    let count = 0;
    for (const range of ranges) {
        count += (range.end - range.start) + 1;
    }

    console.log(count);
});
