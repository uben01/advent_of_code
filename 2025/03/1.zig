const std = @import("std");

const fs = std.fs;
const parseInt = std.fmt.parseInt;
const ArrayList = std.ArrayList;

const print = std.debug.print;

pub fn main() !void {
    const file = try fs.cwd().openFile("./03/test.txt", .{});
    defer file.close();

    var fileBuffer: [1000]u8 = undefined;
    var reader = file.reader(&fileBuffer);

    var max: usize = 0;
    var maxI: usize = 0;
    var secMax: usize = 0;

    var sum: usize = 0;
    while (try reader.interface.takeDelimiter('\n')) |line| {
        for (0..line.len - 2) |i| {
            const array = line[i..i+1];
            print("{any}", .{array});
            const element = try std.fmt.parseInt(usize, array, 10);
            if (element > max) {
                max = element;
                maxI = i;
            }
        }
        for (maxI+1..line.len - 1) |i| {
            const array = line[i..i+1];
            const element = try std.fmt.parseInt(usize, array, 10);
            if (element > secMax) {
                secMax = element;
            }
        }
        const subSum = (10 * max) + secMax;
        print("SubSum: {d}\n", .{subSum});

        sum += subSum;
    }

    print("Sum: {d}", .{sum});
}
