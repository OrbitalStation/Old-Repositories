#include "../inc/creature.h"
#include "../inc/field.h"
#include "../inc/random.h"

#define xmax (width  - 1)
#define ymax (height - 1)
#define xmin 0
#define ymin 0

/* Flags */
#define OF 0b1
#define AF 0b10
#define ZF 0b100
#define DF 0b1000

#define getF(flag) (FP & flag)
#define setF(flag) (FP |= flag)
#define clearF(flag) (FP &= ~flag)

/* Registers */
#define IP c.regs.ip
#define XP c.pos.x
#define YP c.pos.y
#define HP c.health
#define AX c.regs.ax
#define FP c.regs.fp

std::vector <Creature> creatures;

void eval_cmd(Creature &c) {
    if (!c.alive()) return;

    /* Overflow is okay */
    switch (c.code[IP++]) {
        case 0x0: /* kill <-- do suicide */
            c.kill();
            break;
        case 0x1: /* mov %ax, %xp <-- move x position to ax */
            AX = XP;
            break;
        case 0x2: /* mov %ax, %yp <-- move y position to ax */
            AX = YP;
            break;
        case 0x3: /* mov %ax, %hp <-- move health to ax */
            AX = HP;
            break;
        case 0x4: /* rtl <-- rotate left */
            c.dir = Direction(u8(c.dir) + 1);
            break;
        case 0x5: /* rtr <-- rotate right */
            c.dir = Direction(u8(c.dir) - 1);
            break;
        case 0x6: /* rtt <-- rotate on 180 */
            c.dir = Direction(u8(c.dir) + 2);
            break;
        case 0x7: /* sct <-- stores front cell type in ax */
            AX = c.type();
            break;
        case 0x8: /* go <-- moves on cell in front */
            c.move();
            break;
        case 0x9: /* eat <-- if there's food on cell in front then eat it */
            c.eat();
            break;
        case 0xA: /* sse <-- if there's poison on cell in front then remove it */
            if (c.type() == Poison) c.set(Void);
            break;
        case 0xB: /* can <-- set AF if can move on cell in front */
            if (c.type() > Wall) setF(AF);
            else clearF(AF);
            break;
        case 0xC: /* beat <-- if there's creature on cell in front then beat it */
            c.beat();
            break;
        case 0xD: /* add %ax, N <-- add N to %ax, change OF */
            if (IP != 0) {
                if (u16(AX + c.code[IP]) > 255) setF(OF);
                else clearF(OF);
                AX += c.code[IP++];
            }
            break;
        case 0xE: /* and %ax, N <-- do logical and between %ax and N */
            if (IP != 0) AX &= c.code[IP++];
            break;
        case 0xF: /* sub %ax, N <-- sub N to %ax, change OF */
            if (IP != 0) {
                if (i16(AX - c.code[IP]) < 0) setF(OF);
                else clearF(OF);
                AX -= c.code[IP++];
            }
            break;
        case 0x10: /* or %ax, N <-- do logical or between %ax and N */
            if (IP != 0) AX |= c.code[IP++];
            break;
        case 0x11: /* xor %ax, N <-- do logical xor between %ax and N */
            if (IP != 0) AX ^= c.code[IP++];
            break;
        case 0x12: /* cmp %ax, N <-- compare %ax and N, change ZF */
            if (IP != 0) {
                if (AX == c.code[IP++]) setF(ZF);
                else clearF(ZF);
            }
            break;
        case 0x13: /* jmp ADDR <-- jump to ADDR-th byte of program */
            if (IP != 0) IP = c.code[IP];
            break;
        case 0x14: /* je/jz ADDR <-- jump to ADDR-th byte of program if ZF = 1 */
            if (IP != 0 and getF(ZF)) IP = c.code[IP];
            break;
        case 0x15: /* jne/jnz ADDR <-- jump to ADDR-th byte of program if ZF = 0 */
            if (IP != 0 and !getF(ZF)) IP = c.code[IP];
            break;
        case 0x16: /* jo ADDR <-- jump to ADDR-th byte of program if OF = 1 */
            if (IP != 0 and getF(OF)) IP = c.code[IP];
            break;
        case 0x17: /* jno ADDR <-- jump to ADDR-th byte of program if OF = 0 */
            if (IP != 0 and !getF(OF)) IP = c.code[IP];
            break;
        case 0x18: /* ja ADDR <-- jump to ADDR-th byte of program if AF = 1 */
            if (IP != 0 and getF(AF)) IP = c.code[IP];
            break;
        case 0x19: /* jna ADDR <-- jump to ADDR-th byte of program if AF = 0 */
            if (IP != 0 and !getF(AF)) IP = c.code[IP];
            break;
        case 0x1A: /* jd ADDR <-- jump to ADDR-th byte of program if DF = 1 */
            if (IP != 0 and getF(DF)) IP = c.code[IP];
            break;
        case 0x1B: /* jnd ADDR <-- jump to ADDR-th byte of program if DF = 0 */
            if (IP != 0 and !getF(DF)) IP = c.code[IP];
            break;
        case 0x1C: /* sta <-- AF = 1 */
            setF(AF);
            break;
        case 0x1D: /* cla <-- AF = 0 */
            clearF(AF);
            break;
        case 0x1E: /* sto <-- OF = 1 */
            setF(OF);
            break;
        case 0x1F: /* clo <-- OF = 0 */
            clearF(OF);
            break;
        case 0x20: /* stz <-- ZF = 1 */
            setF(ZF);
            break;
        case 0x21: /* clz <-- ZF = 0 */
            clearF(ZF);
            break;
        case 0x22: /* std <-- DF = 1 */
            setF(DF);
            break;
        case 0x23: /* cld <-- DF = 0 */
            clearF(DF);
            break;
        case 0x24: /* inc %ax <-- ++%ax */
            ++AX;
            break;
        case 0x25: /* dec %ax <-- --%ax */
            --AX;
            break;
        case 0x26: /* mul N <-- %ax *= N, change OF */
            if (IP != 0) {
                if (u16(AX) * c.code[IP] > 255) setF(OF);
                else clearF(OF);
                AX *= c.code[IP++];
            }
            break;
        case 0x27: /* div N <-- %ax /= N */
            if (IP != 0 and c.code[IP] != 0) AX /= c.code[IP++];
            break;
        case 0x28: /* loop N <-- --%ax; if %ax != 0 then jmp N */
            if (IP != 0) {
                if (--AX != 0) IP = c.code[IP];
                else ++IP;
            }
            break;
        case 0x29: /* mov %ax, N <-- %ax = N */
            if (IP != 0) AX = c.code[IP++];
            break;
        /* ... */
        case 0x2A: /* nop <-- do nothing */
        default:
            break;
    }
}

Creature::Creature(u8 x, u8 y): pos(x, y), health(255), regs{ .ip = 0, .ax = 0, .fp = 0 } { // NOLINT(cppcoreguidelines-pro-type-member-init)
    for (u64 i = 0; i < capacity / sizeof(u32); ++i) ((u32 *) code)[i] = mersenne();
}

u8 Creature::x() const {
    switch (dir) {
        case Right: return pos.x + 1;
        case Left:  return pos.x - 1;
        default:    return pos.x;
    }
}

u8 Creature::y() const {
    switch (dir) {
        case Up: return pos.y - 1;
        case Down:  return pos.y + 1;
        default: return pos.y;
    }
}

void Creature::move_case(u8 x, u8 y) {
    if (field[x][y] > Wall /* Poison || Void */) {
        field[pos.x][pos.y] = Void;
        if (field[pos.x = x][pos.y = y] == Poison) damage(poison_damage);
        field[pos.x][pos.y] = Alive;
    }
}

void Creature::move() {
    switch (dir) {
        case Up:
            if (pos.y != ymin) move_case(pos.x, pos.y - 1);
            break;
        case Right:
            if (pos.x != xmax) move_case(pos.x + 1, pos.y);
            break;
        case Down:
            if (pos.y != ymax) move_case(pos.x, pos.y + 1);
            break;
        case Left:
            if (pos.x != xmin) move_case(pos.x - 1, pos.y);
            break;
    }
}

void Creature::kill() {
    health = 0;
    field[pos.x][pos.y] = Void;
}

void Creature::damage(u8 damage) {
    if (damage >= health) kill();
    else health -= damage;
}

void Creature::eat() {
    if (type() != Food) return;
    set(Void);
    if (health > u16(max_health - food_heal)) health = max_health;
    else health += food_heal;
}

void Creature::beat() const {
    if (type() != Alive) return;
    for (auto &i: creatures) {
        if (i.pos.x == x() and i.pos.y == y()) {
            i.damage(beat_power);
            break;
        }
    }
}

void Creature::set(Cell type) const {
    switch (dir) {
        case Up:
            if (pos.y != ymin) field[pos.x][pos.y - 1] = type;
            break;
        case Right:
            if (pos.x != xmax) field[pos.x + 1][pos.y] = type;
            break;
        case Down:
            if (pos.y != ymax) field[pos.x][pos.y + 1] = type;
            break;
        case Left:
            if (pos.x != xmin) field[pos.x - 1][pos.y] = type;
            break;
    }
}

bool Creature::alive() const {
    return health != 0;
}

Cell Creature::type() const {
    switch (dir) {
        case Up:
            return (pos.y != ymin) ? field[pos.x][pos.y - 1] : Void;
        case Right:
            return (pos.x != xmax) ? field[pos.x + 1][pos.y] : Void;
        case Down:
            return (pos.y != ymax) ? field[pos.x][pos.y + 1] : Void;
        case Left:
            return (pos.x != ymin) ? field[pos.x - 1][pos.y] : Void;
    }
}
