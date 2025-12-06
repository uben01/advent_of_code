const fs = require('node:fs');

fs.readFile('input.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    const rows = [];
    let operators;

    const lines = data.trim().split('\n');
    lines.forEach((line, i) => {
        const cols = line.trim().split(/\s+/);

        if (i === lines.length - 1) {
            operators = cols;
            return;
        }

        const row = cols.map(col => Number.parseInt(col, 10));
        rows.push(row);
    })

    let total = 0;
    operators.forEach((operator, colIndex) => {
        const colValues = rows.map(row => row[colIndex]);
        if (operator === '+') {
            total += add(...colValues);
        } else if (operator === '*') {
            total += multiply(...colValues);
        }
    })

    console.log(total);
});

function add(...a) {
    return a.reduce((acc, val) => acc + val, 0);
}

function multiply(...a) {
    return a.reduce((acc, val) => acc * val, 1);
}
