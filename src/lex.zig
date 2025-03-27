const std = @import("std");
const mecha = @import("mecha");

pub const Ast = struct {
    scopes: ?[]Scope,
};

pub const ScopeTag = enum {
    input,
    output,
    testing,
    logic,
    process,
};

pub const Scope = union(ScopeTag) {
    input: ?[]Variable,
    output: ?[]Variable,
    testing: []const u8,
    logic: []const u8,
    process: struct { []const u8, []const u8 },
};

pub const Access = enum {
    global,
    local,
};

pub const VariableType = enum { bool };
pub const VariableDefault = union(VariableType) {
    bool: bool,
};

pub const Variable = struct {
    global: Access,
    name: []const u8,
    type: VariableType,
    default: []const u8,
};

pub const scopes = scope.many(.{}).map(mecha.toStruct(Ast));

const scope = mecha.oneOf(.{ scope_input, scope_ouput, scope_testing, scope_logic, scope_process });

const scope_input = mecha.combine(.{
    ws,
    mecha.string("input").discard(),
    ws,
    lcurly,
    ws,
    variable.many(.{}),
    ws,
    rcurly,
    ws,
}).map(mecha.unionInit(Scope, ScopeTag.input));
const scope_ouput = mecha.combine(.{
    ws,
    mecha.string("output").discard(),
    ws,
    lcurly,
    ws,
    variable.many(.{}),
    ws,
    rcurly,
    ws,
}).map(mecha.unionInit(Scope, ScopeTag.output));
const scope_testing = mecha.combine(.{
    ws,
    mecha.string("testing").discard(),
    ws,
    lcurly,
    ws,
    chars,
    ws,
    rcurly,
    ws,
}).map(mecha.unionInit(Scope, ScopeTag.testing));
const scope_logic = mecha.combine(.{
    ws,
    mecha.string("logic").discard(),
    ws,
    lcurly,
    ws,
    chars,
    ws,
    rcurly,
    ws,
}).map(mecha.unionInit(Scope, ScopeTag.logic));
const scope_process = mecha.combine(.{
    ws,
    mecha.string("process").discard(),
    ws,
    chars,
    ws,
    lcurly,
    ws,
    chars,
    ws,
    rcurly,
    ws,
}).map(mecha.unionInit(Scope, ScopeTag.process));

pub const variable = mecha.combine(.{
    ws,
    name.convert(mecha.toEnum(Access)),
    ws,
    name,
    ws,
    colon,
    ws,
    name.convert(mecha.toEnum(VariableType)),
    ws,
    mecha.opt(mecha.combine(.{
        equal,
        ws,
        number,
    })),
    ws,
    comma,
    ws,
}).map(mecha.toStruct(Variable));

const name = mecha.combine(.{
    nameChars,
    nameStartChar,
});
const nameChars = mecha.oneOf(.{
    nameStartChar,
    digit,
}).many(.{});
const nameStartChar = mecha.oneOf(.{
    mecha.utf8.range('a', 'z'),
    mecha.utf8.range('A', 'Z'),
    mecha.utf8.char('_'),
});

const chars = char.many(mecha.ManyOptions{ .collect = false });
const char = mecha.utf8.range(0x0, 0x7a);

const number = mecha.combine(.{
    mecha.opt(sign),
    digit,
});
const sign =
    mecha.oneOf(.{ plus, minus });
const digit = mecha.utf8.range('0', '9');

const lcurly = mecha.utf8.char('{').discard();
const rcurly = mecha.utf8.char('}').discard();
const colon = mecha.utf8.char(':').discard();
const comma = mecha.utf8.char(',').discard();
const equal = mecha.utf8.char('=').discard();
const minus = mecha.utf8.char('-').discard();
const plus = mecha.utf8.char('+').discard();

const ws = mecha.oneOf(.{
    mecha.utf8.char(0x0020), // Space
    mecha.utf8.char(0x000A), // LF
    mecha.utf8.char(0x000D), // CR
    mecha.utf8.char(0x0009), // Tab
}).many(mecha.ManyOptions{ .collect = false }).discard();
