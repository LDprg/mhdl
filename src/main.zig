const std = @import("std");
const mecha = @import("mecha");

const lbracket = token(mecha.utf8.char('['));
const rbracket = token(mecha.utf8.char(']'));
const lcurly = token(mecha.utf8.char('{'));
const rcurly = token(mecha.utf8.char('}'));

// const scope

fn token(comptime parser: anytype) mecha.Parser(void) {
    return mecha.combine(.{ parser.discard(), ws });
}

const ws = mecha.oneOf(.{
    mecha.utf8.char(0x0020),
    mecha.utf8.char(0x000A),
    mecha.utf8.char(0x000D),
    mecha.utf8.char(0x0009),
}).many(.{ .collect = false }).discard();

pub fn main() !void {}
