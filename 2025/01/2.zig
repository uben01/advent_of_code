const std = @import("std");
const fs = std.fs;

const print = std.debug.print;

pub fn main() !void {
    const file = try fs.cwd().openFile("./input.txt", .{});
    defer file.close();

    var buffer: [100]u8 = undefined;
    var reader = file.reader(&buffer);

    var num_of_0s: i32 = 0;

    var state: i32 = 50;
    while (try reader.interface.takeDelimiter('\n')) |line| {
        const num = try std.fmt.parseInt(u32, line[1..], 10);
        var plus = true;
        if (line[0] == 'L') {
            plus = false;
        }

        for (0..num) |_| {
            if (plus) {
                state += 1;
            } else {
                state -= 1;
            }

            if (@rem(state, 100) == 0) {
                num_of_0s += 1;
            }
        }

    }
    print("{}\n", .{ num_of_0s });
}
