const std = @import("std");

const fs = std.fs;
const parseInt = std.fmt.parseInt;
const ArrayList = std.ArrayList;

const print = std.debug.print;

pub fn main() !void {
    const file = try fs.cwd().openFile("./02/input.txt", .{});
    defer file.close();

    var fileBuffer: [1000]u8 = undefined;
    var reader = file.reader(&fileBuffer);

    const allocator = std.heap.page_allocator;
    var list = ArrayList(usize).empty;
    defer list.deinit(allocator);

    while (try reader.interface.takeDelimiter('\n')) |line| {
        var ranges = std.mem.splitScalar(u8, line, ',');
        while (ranges.next()) |range| {
            if (range.len == 0) {
                continue;
            }

            var borders = std.mem.splitScalar(u8, range, '-');
            const left = try parseInt(u64, borders.first(), 10);
            const right = try parseInt(u64, borders.next().?, 10);

            var numBuf: [20]u8 = undefined;
            for (left..right+1) |value| {
                const numAsString = try std.fmt.bufPrint(&numBuf, "{d}", .{value});
                if (numAsString.len % 2 != 0) {
                    continue;
                }

                var broken = true;
                const halfPoint = numAsString.len / 2;
                for (0..halfPoint) |digit| {
                    if (numAsString[digit] != numAsString[halfPoint + digit]) {
                        broken = false;
                        break;
                    }
                }

                if (broken) {
                    try list.append(allocator, value);
                }
            }
        }
    }
    print("{any}", .{list.items});

    var sum: usize = 0;
    for (list.items) |value| {
        sum += value;
    }
    print("{any}", .{sum});
}
