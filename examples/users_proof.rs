#![no_main]
#![no_std]

use kyaaa::prelude::*;

#[inline(always)]
fn black_box<T>(dummy: T) -> T {
    core::hint::black_box(dummy)
}

page!(
    align(8) struct User {
        id: u64,
        value: u64,
    }
);

kmain!(
    align(64);

    fn kmain(_sp: *const usize) -> i32 {
        let hlist = book!(
            u01: User = User { id: 1, value: 0 },
            u02: User = User { id: 2, value: 0 },
            u03: User = User { id: 3, value: 0 },
            u04: User = User { id: 4, value: 0 },
            u05: User = User { id: 5, value: 0 },
            u06: User = User { id: 6, value: 0 },
            u07: User = User { id: 7, value: 0 },
            u08: User = User { id: 8, value: 0 },
            u09: User = User { id: 9, value: 0 },
            u10: User = User { id: 10, value: 0 },
            u11: User = User { id: 11, value: 0 },
            u12: User = User { id: 12, value: 0 },
            u13: User = User { id: 13, value: 0 },
            u14: User = User { id: 14, value: 0 },
            u15: User = User { id: 15, value: 0 },
            u16: User = User { id: 16, value: 0 },
            u17: User = User { id: 17, value: 0 },
            u18: User = User { id: 18, value: 0 },
            u19: User = User { id: 19, value: 0 },
            u20: User = User { id: 20, value: 0 },
            u21: User = User { id: 21, value: 0 },
            u22: User = User { id: 22, value: 0 },
            u23: User = User { id: 23, value: 0 },
            u24: User = User { id: 24, value: 0 },
            u25: User = User { id: 25, value: 0 },
            u26: User = User { id: 26, value: 0 },
            u27: User = User { id: 27, value: 0 },
            u28: User = User { id: 28, value: 0 },
            u29: User = User { id: 29, value: 0 },
            u30: User = User { id: 30, value: 0 },
            u31: User = User { id: 31, value: 0 },
            u32: User = User { id: 32, value: 0 },
            u33: User = User { id: 33, value: 0 },
            u34: User = User { id: 34, value: 0 },
            u35: User = User { id: 35, value: 0 },
            u36: User = User { id: 36, value: 0 },
            u37: User = User { id: 37, value: 0 },
            u38: User = User { id: 38, value: 0 },
            u39: User = User { id: 39, value: 0 },
            u40: User = User { id: 40, value: 0 },
            u41: User = User { id: 41, value: 0 },
            u42: User = User { id: 42, value: 0 },
            u43: User = User { id: 43, value: 0 },
            u44: User = User { id: 44, value: 0 },
            u45: User = User { id: 45, value: 0 },
            u46: User = User { id: 46, value: 0 },
            u47: User = User { id: 47, value: 0 },
            u48: User = User { id: 48, value: 0 },
            u49: User = User { id: 49, value: 0 },
            u50: User = User { id: 50, value: 0 },
        );

            hlist.u01().set().value(1001);
            hlist.u02().set().value(1002);
            hlist.u03().set().value(1003);
            hlist.u04().set().value(1004);
            hlist.u05().set().value(1005);
            hlist.u06().set().value(1006);
            hlist.u07().set().value(1007);
            hlist.u08().set().value(1008);
            hlist.u09().set().value(1009);
            hlist.u10().set().value(1010);
            hlist.u11().set().value(1011);
            hlist.u12().set().value(1012);
            hlist.u13().set().value(1013);
            hlist.u14().set().value(1014);
            hlist.u15().set().value(1015);
            hlist.u16().set().value(1016);
            hlist.u17().set().value(1017);
            hlist.u18().set().value(1018);
            hlist.u19().set().value(1019);
            hlist.u20().set().value(1020);
            hlist.u21().set().value(1021);
            hlist.u22().set().value(1022);
            hlist.u23().set().value(1023);
            hlist.u24().set().value(1024);
            hlist.u25().set().value(1025);
            hlist.u26().set().value(1026);
            hlist.u27().set().value(1027);
            hlist.u28().set().value(1028);
            hlist.u29().set().value(1029);
            hlist.u30().set().value(1030);
            hlist.u31().set().value(1031);
            hlist.u32().set().value(1032);
            hlist.u33().set().value(1033);
            hlist.u34().set().value(1034);
            hlist.u35().set().value(1035);
            hlist.u36().set().value(1036);
            hlist.u37().set().value(1037);
            hlist.u38().set().value(1038);
            hlist.u39().set().value(1039);
            hlist.u40().set().value(1040);
            hlist.u41().set().value(1041);
            hlist.u42().set().value(1042);
            hlist.u43().set().value(1043);
            hlist.u44().set().value(1044);
            hlist.u45().set().value(1045);
            hlist.u46().set().value(1046);
            hlist.u47().set().value(1047);
            hlist.u48().set().value(1048);
            hlist.u49().set().value(1049);
            hlist.u50().set().value(1050);

            black_box(hlist.u01().get());
            black_box(hlist.u02().get());
            black_box(hlist.u03().get());
            black_box(hlist.u04().get());
            black_box(hlist.u05().get());
            black_box(hlist.u06().get());
            black_box(hlist.u07().get());
            black_box(hlist.u08().get());
            black_box(hlist.u09().get());
            black_box(hlist.u10().get());
            black_box(hlist.u11().get());
            black_box(hlist.u12().get());
            black_box(hlist.u13().get());
            black_box(hlist.u14().get());
            black_box(hlist.u15().get());
            black_box(hlist.u16().get());
            black_box(hlist.u17().get());
            black_box(hlist.u18().get());
            black_box(hlist.u19().get());
            black_box(hlist.u20().get());
            black_box(hlist.u21().get());
            black_box(hlist.u22().get());
            black_box(hlist.u23().get());
            black_box(hlist.u24().get());
            black_box(hlist.u25().get());
            black_box(hlist.u26().get());
            black_box(hlist.u27().get());
            black_box(hlist.u28().get());
            black_box(hlist.u29().get());
            black_box(hlist.u30().get());
            black_box(hlist.u31().get());
            black_box(hlist.u32().get());
            black_box(hlist.u33().get());
            black_box(hlist.u34().get());
            black_box(hlist.u35().get());
            black_box(hlist.u36().get());
            black_box(hlist.u37().get());
            black_box(hlist.u38().get());
            black_box(hlist.u39().get());
            black_box(hlist.u40().get());
            black_box(hlist.u41().get());
            black_box(hlist.u42().get());
            black_box(hlist.u43().get());
            black_box(hlist.u44().get());
            black_box(hlist.u45().get());
            black_box(hlist.u46().get());
            black_box(hlist.u47().get());
            black_box(hlist.u48().get());
            black_box(hlist.u49().get());
            black_box(hlist.u50().get());

        0
    }
);
