const std = @import("std");
const mecha = @import("mecha");

const lex = @import("lex.zig");

pub fn parse(alloc: std.mem.Allocator, data: []const u8) !lex.Ast {
    std.debug.print("{s}\n\n", .{data});

    const lex_scope = try lex.scopes.parse(alloc, data);
    const value: lex.Ast = lex_scope.value.ok;

    std.debug.print("Cnt: {}\n", .{value.scopes.?.len});
    std.debug.print("{any}\n", .{lex_scope.value});
    std.debug.print("-----------\n", .{});
    for (value.scopes.?) |scope| {
        std.debug.print("{any}\n", .{scope});
    }
    std.debug.print("-----------\n", .{});

    return value;
}
