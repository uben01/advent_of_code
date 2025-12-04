const fs = require('node:fs');

fs.readFile('input.txt', 'utf8', (err, data) => {
    if (err) {
        console.error(err);
        return;
    }

    let grid = [];
    let maxX = 0;
    let maxY = 0;

    const lines = data.trim().split('\n');
    for (const line of lines) {
        grid.push(line.split(''));
        maxX = line.length;
    }
    maxY = grid.length;

    let sum = 0;
    for (let y = 0; y < maxY; y++) {
        outer: for (let x = 0; x < maxX; x++) {
            const current = grid[y][x];
            if (current !== '@') {
                continue;
            }

            let aroundCound = 0;
            for (let dy = -1; dy <= 1; dy++) {
                for (let dx = -1; dx <= 1; dx++) {
                    if (dx === 0 && dy === 0) {
                        continue;
                    }
                    if (x + dx < 0 || x + dx >= maxX || y + dy < 0 || y + dy >= maxY) {
                        continue;
                    }

                    if (grid[y + dy][x + dx] === '@') {
                        aroundCound++;
                    }

                    if (aroundCound >= 4) {
                        continue outer;
                    }
                }
            }
            sum++;
        }
    }

    console.log(sum);
});
