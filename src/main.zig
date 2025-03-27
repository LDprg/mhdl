const std = @import("std");
const mecha = @import("mecha");

const mhdl = mecha.combine(.{
    scope_input,
    lcurly,
    chars,
    rcurly,
}).many(.{ .collect = false });

const scope = mecha.oneOf(.{ scope_input, scope_ouput, scope_test, scope_logic, scope_process });

const scope_input = token(mecha.string("input"));
const scope_ouput = token(mecha.string("output"));
const scope_test = token(mecha.string("test"));
const scope_logic = token(mecha.string("logic"));
const scope_process = token(mecha.string("process"));

const lbracket = token(mecha.utf8.char('['));
const rbracket = token(mecha.utf8.char(']'));
const lcurly = token(mecha.utf8.char('{'));
const rcurly = token(mecha.utf8.char('}'));

fn token(comptime parser: anytype) mecha.Parser(void) {
    return mecha.combine(.{ parser.discard(), ws });
}

const ws = mecha.oneOf(.{
    mecha.utf8.char(0x0020),
    mecha.utf8.char(0x000A),
    mecha.utf8.char(0x000D),
    mecha.utf8.char(0x0009),
}).many(.{ .collect = false }).discard();

const chars = char.many(.{ .collect = false }).discard();

const char = mecha.oneOf(.{
    mecha.utf8.range('a', 'z').discard(),
    mecha.utf8.range('A', 'Z').discard(),
});

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

    const res = try mhdl.parse(alloc, file);

    std.debug.print("{any}\n\n", .{res});
    std.debug.print("Value: {s}\n\n", .{res.value.ok});
}

test "simple.mhdl" {
    const alloc = std.testing.allocator;

    const file = try readFile(alloc, "./test/simple.mhdl");
    defer alloc.free(file);

    // std.debug.print("{s}", .{file});
}
