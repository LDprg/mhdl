const std = @import("std");
const mecha = @import("mecha");

pub const Ast = struct {
    scopes: ?[]Scope,
};

const ScopeTag = enum {
    input,
    output,
    testing,
    logic,
    process,
};

pub const Scope = union(ScopeTag) {
    input: []const u8,
    output: []const u8,
    testing: []const u8,
    logic: []const u8,
    process: struct { []const u8, []const u8 },
};

pub const scopes = scope.many(.{}).map(mecha.toStruct(Ast));

const scope = mecha.oneOf(.{ scope_input, scope_ouput, scope_testing, scope_logic, scope_process });

const scope_input = mecha.combine(.{
    ws,
    mecha.string("input").discard(),
    ws,
    lcurly,
    chars,
    rcurly,
}).map(mecha.unionInit(Scope, ScopeTag.input));
const scope_ouput = mecha.combine(.{
    ws,
    mecha.string("output").discard(),
    ws,
    lcurly,
    chars,
    rcurly,
}).map(mecha.unionInit(Scope, ScopeTag.output));
const scope_testing = mecha.combine(.{
    ws,
    mecha.string("testing").discard(),
    ws,
    lcurly,
    chars,
    rcurly,
}).map(mecha.unionInit(Scope, ScopeTag.testing));
const scope_logic = mecha.combine(.{
    ws,
    mecha.string("logic").discard(),
    ws,
    lcurly,
    chars,
    rcurly,
}).map(mecha.unionInit(Scope, ScopeTag.logic));
const scope_process = mecha.combine(.{
    ws,
    mecha.string("process").discard(),
    ws,
    chars,
    ws,
    lcurly,
    chars,
    rcurly,
}).map(mecha.unionInit(Scope, ScopeTag.process));

const chars = char.many(mecha.ManyOptions{ .collect = false });
const char = mecha.utf8.range(0x0, 0x7a);

const lcurly = token(mecha.utf8.char('{'));
const rcurly = token(mecha.utf8.char('}'));

fn token(comptime parser: anytype) mecha.Parser(void) {
    return mecha.combine(.{ parser.discard(), ws });
}

const ws = mecha.oneOf(.{
    mecha.utf8.char(0x0020), // Space
    mecha.utf8.char(0x000A), // LF
    mecha.utf8.char(0x000D), // CR
    mecha.utf8.char(0x0009), // Tab
}).many(mecha.ManyOptions{ .collect = false }).discard();
