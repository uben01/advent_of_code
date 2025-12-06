const fs = require('node:fs');

fs.readFile('input.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    const map = [];
    let operators = [];

    const lines = data.split('\n');

    const operatorLine = lines[lines.length - 2];
    operatorLine.split('').forEach((char, i) => {
        if (char === '+' || char === '*') {
            operators.push({char, start: i});
            if (operators.length > 1) {
                operators.at(-2).end = i;
            }
        }
        if (i === operatorLine.length - 1) {
            operators.at(-1).end = i + 1;
        }
    });

    lines.forEach((line, i) => {
        operators.forEach((operator, operatorNum) => {
            if (i >= lines.length - 2) {
                return;
            }

            map[i] = map[i] || [];
            const isLastOperator = operatorNum === operators.length - 1;
            let segment = line.slice(operator.start, isLastOperator ? operator.end : operator.end - 1);

            segment = segment.replaceAll(/\s/g, 'x');

            map[i].push(segment);
        });
    });

    let total = 0;
    operators.forEach((operator, colIndex) => {
        const colValues = map.map(row => row[colIndex]);
        const maxLength = Math.max(...colValues.map(val => val.toString().length));

        let operand = [];
        for (let i = 0; i < maxLength; i++) {
            let subOperand = [];
            for (const element of colValues) {
                subOperand.push(element.toString()[i])
            }
            let num = subOperand.join('');
            num = num.replaceAll('x', '');
            if (num === '') {
                num = operator.char === '+' ? '0' : '1';
            }
            operand.push(num);
        }
        let subTotal = operator.char === '+' ? add(...operand) : multiply(...operand);

        total += subTotal;
    })

    console.log("Total:", total);
});

function add(...a) {
    return a.reduce((acc, val) => acc + Number.parseInt(val, 10), 0);
}

function multiply(...a) {
    return a.reduce((acc, val) => acc * Number.parseInt(val, 10), 1);
}
