const std = @import("std");

const parser = @import("parser.zig");

fn readFile(alloc: std.mem.Allocator, path: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    return file.readToEndAlloc(alloc, std.math.maxInt(usize));
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();

    const file = try readFile(alloc, "./test/simple.mhdl");
    defer alloc.free(file);

    const res = try parser.parse(alloc, file);

    std.debug.print("{any}", .{res});
}

test "simple.mhdl" {
    const alloc = std.testing.allocator;

    const file = try readFile(alloc, "./test/simple.mhdl");
    defer alloc.free(file);

    // std.debug.print("{s}", .{file});
}
