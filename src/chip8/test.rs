// use super::Chip8;

// // const KEYCODE_FOR_TEST = KeyCode::Key1;

// #[test]
// fn test_init() {
//     let chip8 = Chip8::new();
//     assert_eq!(chip8.program_counter, 0x200);
// }

// #[test]
// fn test_opcode_00E0() {
//     let mut chip8 = Chip8::new();
//     chip8.screen[0][0] = 1;
//     assert_eq!(chip8.screen[0][0], 1);
//     chip8.perform_opcode(0x00E0);
//     assert_eq!(chip8.screen[0][0], 0);
// }

// #[test]
// fn test_opcode_1nnn() {
//     let mut chip8 = Chip8::new();
//     chip8.perform_opcode(0x1001);
//     assert_eq!(chip8.program_counter, 0x1);
// }

// #[test]
// fn test_opcode_2nnn() {
//     let mut chip8 = Chip8::new();
//     chip8.perform_opcode(0x2010);
//     assert_eq!(chip8.stack_pointer, 1);
//     assert_eq!(chip8.program_counter, 0x10);
// }

// #[test]
// fn test_opcode_3xkk() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[2] = 0x10;
//     chip8.perform_opcode(0x3210);
//     assert_eq!(chip8.registers[2], 0x10);
//     assert_eq!(chip8.program_counter, 0x204);

//     chip8.registers[2] = 0x11;
//     chip8.program_counter = 0x200;
//     chip8.perform_opcode(0x3210);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_4xkk() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[2] = 0x10;
//     chip8.perform_opcode(0x4210);
//     assert_eq!(chip8.registers[2], 0x10);
//     assert_eq!(chip8.program_counter, 0x202);

//     chip8.registers[2] = 0x11;
//     chip8.program_counter = 0x200;
//     chip8.perform_opcode(0x4210);
//     assert_eq!(chip8.program_counter, 0x204);
// }

// #[test]
// fn test_opcode_5xy0() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 0x1;
//     chip8.registers[1] = 0x1;
//     chip8.perform_opcode(0x5010);
//     assert_eq!(chip8.program_counter, 0x204);

//     chip8.program_counter = 0x200;
//     chip8.registers[1] = 0x0;
//     chip8.perform_opcode(0x5010);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_6xkk() {
//     let mut chip8 = Chip8::new();
//     chip8.perform_opcode(0x6011);
//     assert_eq!(chip8.registers[0], 0x11);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_7xkk() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 0x10;
//     chip8.perform_opcode(0x7001);
//     assert_eq!(chip8.registers[0], 0x11);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xy0() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 0x10;
//     chip8.registers[1] = 0x0;
//     chip8.perform_opcode(0x8010);
//     assert_eq!(chip8.registers[0], 0x0);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xy1() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 0x10;
//     chip8.registers[1] = 0x01;
//     chip8.perform_opcode(0x8011);
//     assert_eq!(chip8.registers[0], 0x11);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xy2() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 0x01;
//     chip8.registers[1] = 0x10;
//     chip8.perform_opcode(0x8012);
//     assert_eq!(chip8.registers[0], 0x0);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xy3() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 0x01;
//     chip8.registers[1] = 0x11;
//     chip8.perform_opcode(0x8013);
//     assert_eq!(chip8.registers[0], 0x10);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xy4() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 0xFF;
//     chip8.registers[1] = 0x01;
//     chip8.perform_opcode(0x8014);
//     assert_eq!(chip8.registers[0], 0x00);
//     assert_eq!(chip8.vf, 1);
//     assert_eq!(chip8.program_counter, 0x202);

//     chip8.program_counter = 0x200;
//     chip8.registers[0] = 0x01;
//     chip8.registers[1] = 0x01;
//     chip8.perform_opcode(0x8014);
//     assert_eq!(chip8.registers[0], 0x02);
//     assert_eq!(chip8.vf, 0);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xy5() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 0xFF;
//     chip8.registers[1] = 0x01;
//     chip8.perform_opcode(0x8015);
//     assert_eq!(chip8.registers[0], 0xFE);
//     assert_eq!(chip8.vf, 1);
//     assert_eq!(chip8.program_counter, 0x202);

//     chip8.program_counter = 0x200;
//     chip8.registers[0] = 0x00;
//     chip8.registers[1] = 0xFF;
//     chip8.perform_opcode(0x8015);
//     assert_eq!(chip8.registers[0], 0x01);
//     assert_eq!(chip8.vf, 0);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xy6() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 10;
//     chip8.registers[1] = 2;
//     chip8.perform_opcode(0x8016);
//     assert_eq!(chip8.registers[0], 5);
//     assert_eq!(chip8.vf, 0);
//     assert_eq!(chip8.program_counter, 0x202);

//     chip8.program_counter = 0x200;
//     chip8.registers[0] = 11;
//     chip8.registers[1] = 2;
//     chip8.perform_opcode(0x8016);
//     assert_eq!(chip8.registers[0], 5);
//     assert_eq!(chip8.vf, 1);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xy7() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 1;
//     chip8.registers[1] = 255;
//     chip8.perform_opcode(0x8017);
//     assert_eq!(chip8.registers[0], 254);
//     assert_eq!(chip8.vf, 1);
//     assert_eq!(chip8.program_counter, 0x202);

//     chip8.program_counter = 0x200;
//     chip8.registers[0] = 11;
//     chip8.registers[1] = 2;
//     chip8.perform_opcode(0x8017);
//     assert_eq!(chip8.registers[0], 247);
//     assert_eq!(chip8.vf, 0);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_8xyE() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 1;
//     chip8.perform_opcode(0x801E);
//     assert_eq!(chip8.registers[0], 2);
//     assert_eq!(chip8.vf, 0);
//     assert_eq!(chip8.program_counter, 0x202);

//     chip8.program_counter = 0x200;
//     chip8.registers[0] = 128;
//     chip8.perform_opcode(0x801E);
//     assert_eq!(chip8.registers[0], 0);
//     assert_eq!(chip8.vf, 1);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_9xy0() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 1;
//     chip8.registers[1] = 1;
//     chip8.perform_opcode(0x9010);
//     assert_eq!(chip8.program_counter, 0x202);

//     chip8.registers[0] = 0;
//     chip8.program_counter = 0x200;
//     chip8.perform_opcode(0x9010);
//     assert_eq!(chip8.program_counter, 0x204);
// }

// #[test]
// fn test_opcode_Annn() {
//     let mut chip8 = Chip8::new();
//     chip8.perform_opcode(0xA001);
//     assert_eq!(chip8.i, 1);
// }

// #[test]
// fn test_opcode_Bnnn() {
//     let mut chip8 = Chip8::new();
//     chip8.registers[0] = 2;
//     chip8.perform_opcode(0xB1FE);
//     assert_eq!(chip8.program_counter, 0x200);
// }

// #[test]
// fn test_opcode_Cxkk() {
//     // random int
// }

// #[test]
// fn test_opcode_Dxyn() {
//     let mut chip8 = Chip8::new();
//     chip8.ram[0x200] = 0b01010101;
//     chip8.i = 0x200;
//     chip8.registers[0] = 0;
//     chip8.registers[1] = 0;

//     // Make sure that screen is blank
//     for i in 0..7 {
//         assert_eq!(chip8.screen[0][i], 0);
//     }

//     chip8.perform_opcode(0xD011);
//     // chip8.screen[0][0] = 1;
//     // assert_eq!(chip8.screen[0][0], 0);

//     // Make sure that it alternates being on and off
//     for i in 0..8 {
//         if i % 2 == 0 {
//             assert_eq!(chip8.screen[0][i], 0)
//         } else {
//             // println!("{}", i);
//             assert_eq!(chip8.screen[0][i], 1);
//         }
//     }

//     chip8.perform_opcode(0xD011);
//     assert_eq!(chip8.vf, 1);
// }

// #[test]
// fn test_opcode_Ex9E() {
//     let mut chip8 = Chip8::new();

//     chip8.keyboard = 0x0;
//     chip8.registers[0] = 0x0;

//     chip8.perform_opcode(0xE09E);
//     assert_eq!(chip8.program_counter, 0x204);

//     chip8.program_counter = 0x200;
//     chip8.keyboard = 0x1;
//     chip8.registers[0] = 0x0;

//     chip8.perform_opcode(0xE09E);
//     assert_eq!(chip8.program_counter, 0x202);
// }

// #[test]
// fn test_opcode_ExA1() {
//     let mut chip8 = Chip8::new();

//     chip8.keyboard = 0x0;
//     chip8.registers[0] = 0x0;

//     chip8.perform_opcode(0xE0A1);
//     assert_eq!(chip8.program_counter, 0x202);

//     chip8.program_counter = 0x200;
//     chip8.keyboard = 0x1;
//     chip8.registers[0] = 0x0;

//     chip8.perform_opcode(0xE0A1);
//     assert_eq!(chip8.program_counter, 0x204);
// }

// #[test]
// fn test_opcode_Fx07() {
//     let mut chip8 = Chip8::new();

//     chip8.delay_timer = 10;
//     chip8.perform_opcode(0xF007);
//     assert_eq!(chip8.registers[0], 10);
// }

// // test Fx0A

// #[test]
// fn test_opcode_Fx15() {
//     let mut chip8 = Chip8::new();

//     chip8.registers[0] = 10;
//     chip8.perform_opcode(0xF015);
//     assert_eq!(chip8.delay_timer, 10);
// }

// #[test]
// fn test_opcode_Fx18() {
//     let mut chip8 = Chip8::new();

//     chip8.registers[0] = 10;
//     chip8.perform_opcode(0xF018);
//     assert_eq!(chip8.sound_timer, 10);
// }

// #[test]
// fn test_opcode_Fx1E() {
//     let mut chip8 = Chip8::new();

//     chip8.i = 15;
//     chip8.registers[0] = 5;

//     chip8.perform_opcode(0xF01E);

//     assert_eq!(chip8.i, 20);
// }

// #[test]
// fn test_opcode_Fx29() {
//     let mut chip8 = Chip8::new();

//     // font stuff
// }

// #[test]
// fn test_opcode_Fx33() {
//     let mut chip8 = Chip8::new();

//     chip8.registers[0] = 123;
//     chip8.i = 0x200;

//     chip8.perform_opcode(0xF033);

//     assert_eq!(chip8.ram[0x200], 1);
//     assert_eq!(chip8.ram[0x201], 2);
//     assert_eq!(chip8.ram[0x202], 3);
// }

// #[test]
// fn test_opcode_Fx55() {
//     // t
// }

use super::Chip8;

// const KEYCODE_FOR_TEST = KeyCode::Key1; // This line might be causing issues if not defined elsewhere or used.

#[test]
fn test_init() {
    let chip8 = Chip8::new();
    assert_eq!(chip8.program_counter, 0x200);
}

#[test]
fn test_opcode_00E0() {
    let mut chip8 = Chip8::new();
    chip8.screen[0][0] = 1;
    assert_eq!(chip8.screen[0][0], 1);
    chip8.perform_opcode(0x00E0);
    assert_eq!(chip8.screen[0][0], 0);
}

#[test]
fn test_opcode_1nnn() {
    let mut chip8 = Chip8::new();
    chip8.perform_opcode(0x1001);
    assert_eq!(chip8.program_counter, 0x1);
}

#[test]
fn test_opcode_2nnn() {
    let mut chip8 = Chip8::new();
    chip8.perform_opcode(0x2010);
    assert_eq!(chip8.stack_pointer, 1);
    assert_eq!(chip8.program_counter, 0x10);
}

#[test]
fn test_opcode_3xkk() {
    let mut chip8 = Chip8::new();
    chip8.registers[2] = 0x10;
    chip8.perform_opcode(0x3210);
    assert_eq!(chip8.registers[2], 0x10); // Vx (V2) equals kk (0x10), PC should skip
    assert_eq!(chip8.program_counter, 0x204); // PC increments by 4 (skip)

    let mut chip8 = Chip8::new(); // Reset for next test case
    chip8.registers[2] = 0x10;
    chip8.perform_opcode(0x3211);
    assert_eq!(chip8.registers[2], 0x10); // Vx (V2) does not equal kk (0x11), PC should not skip
    assert_eq!(chip8.program_counter, 0x202); // PC increments by 2 (no skip)
}

#[test]
fn test_opcode_4xkk() {
    let mut chip8 = Chip8::new();
    chip8.registers[2] = 0x10;
    chip8.perform_opcode(0x4211); // Vx (0x10) does not equal kk (0x11), PC should skip
    assert_eq!(chip8.registers[2], 0x10);
    assert_eq!(chip8.program_counter, 0x204);

    let mut chip8 = Chip8::new(); // Reset for next test case
    chip8.registers[2] = 0x10;
    chip8.perform_opcode(0x4210); // Vx (0x10) equals kk (0x10), PC should not skip
    assert_eq!(chip8.registers[2], 0x10);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_5xy0() {
    let mut chip8 = Chip8::new();
    chip8.registers[2] = 0x10;
    chip8.registers[3] = 0x10;
    chip8.perform_opcode(0x5230); // Vx (0x10) equals Vy (0x10), PC should skip
    assert_eq!(chip8.program_counter, 0x204);

    let mut chip8 = Chip8::new(); // Reset for next test case
    chip8.registers[2] = 0x10;
    chip8.registers[3] = 0x11;
    chip8.perform_opcode(0x5230); // Vx (0x10) does not equal Vy (0x11), PC should not skip
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_6xkk() {
    let mut chip8 = Chip8::new();
    chip8.perform_opcode(0x6123);
    assert_eq!(chip8.registers[1], 0x23);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_7xkk() {
    let mut chip8 = Chip8::new();
    chip8.registers[1] = 0x01;
    chip8.perform_opcode(0x7105);
    assert_eq!(chip8.registers[1], 0x06); // 1 + 5 = 6
    assert_eq!(chip8.program_counter, 0x202);

    let mut chip8 = Chip8::new();
    chip8.registers[1] = 0xFF; // 255
    chip8.perform_opcode(0x7101); // Add 1
    assert_eq!(chip8.registers[1], 0x00); // Should wrap around
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy0() {
    let mut chip8 = Chip8::new();
    chip8.registers[1] = 0x10;
    chip8.registers[2] = 0x20;
    chip8.perform_opcode(0x8120); // Set V1 = V2
    assert_eq!(chip8.registers[1], 0x20);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy1() {
    let mut chip8 = Chip8::new();
    chip8.registers[1] = 0b00001111;
    chip8.registers[2] = 0b11110000;
    chip8.perform_opcode(0x8121); // Set V1 = V1 OR V2
    assert_eq!(chip8.registers[1], 0b11111111);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy2() {
    let mut chip8 = Chip8::new();
    chip8.registers[1] = 0b00001111;
    chip8.registers[2] = 0b11110110;
    chip8.perform_opcode(0x8122); // Set V1 = V1 AND V2
    assert_eq!(chip8.registers[1], 0b00000110);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy3() {
    let mut chip8 = Chip8::new();
    chip8.registers[1] = 0b00001111;
    chip8.registers[2] = 0b11110000;
    chip8.perform_opcode(0x8123); // Set V1 = V1 XOR V2
    assert_eq!(chip8.registers[1], 0b11111111);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy4() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 10;
    chip8.registers[1] = 5;
    chip8.perform_opcode(0x8014); // V0 = V0 + V1 (10 + 5 = 15, no carry)
    assert_eq!(chip8.registers[0], 15);
    assert_eq!(chip8.registers[0xF], 0); // No carry, VF should be 0
    assert_eq!(chip8.program_counter, 0x202);

    let mut chip8 = Chip8::new();
    chip8.registers[0] = 250;
    chip8.registers[1] = 10;
    chip8.perform_opcode(0x8014); // V0 = V0 + V1 (250 + 10 = 260, carry)
    assert_eq!(chip8.registers[0], 4); // 260 wrapped around 256 is 4
    assert_eq!(chip8.registers[0xF], 1); // Carry, VF should be 1
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy5() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 10;
    chip8.registers[1] = 5;
    chip8.perform_opcode(0x8015); // V0 = V0 - V1 (10 - 5 = 5, no borrow)
    assert_eq!(chip8.registers[0], 5);
    assert_eq!(chip8.registers[0xF], 1); // No borrow (Vx > Vy), VF should be 1
    assert_eq!(chip8.program_counter, 0x202);

    let mut chip8 = Chip8::new();
    chip8.registers[0] = 5;
    chip8.registers[1] = 10;
    chip8.perform_opcode(0x8015); // V0 = V0 - V1 (5 - 10 = -5, borrow)
    assert_eq!(chip8.registers[0], 251); // 5 - 10 = -5, wrapped around 256 is 251
    assert_eq!(chip8.registers[0xF], 0); // Borrow (Vx < Vy), VF should be 0
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy6() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0b00001010; // 10
    chip8.perform_opcode(0x8006); // V0 = V0 SHR 1, LSB is 0, no quirk
    assert_eq!(chip8.registers[0], 0b00000101); // 5
    assert_eq!(chip8.registers[0xF], 0);
    assert_eq!(chip8.program_counter, 0x202);

    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0b00001011; // 11
    chip8.perform_opcode(0x8006); // V0 = V0 SHR 1, LSB is 1, no quirk
    assert_eq!(chip8.registers[0], 0b00000101); // 5
    assert_eq!(chip8.registers[0xF], 1);
    assert_eq!(chip8.program_counter, 0x202);

    // Test with quirk: Vx=Vy and VF taken *before* overwrite
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0b10101011; // Vx = 171, LSB 1
    chip8.registers[1] = 0b00000010; // Vy = 2
    chip8.shift_quirk_vx_eq_vy = true;
    chip8.perform_opcode(0x8016); // V0 = V1 then SHR 1
    assert_eq!(chip8.registers[0], 0b00000001); // V0 should be 1 after taking V1 and shifting
    assert_eq!(chip8.registers[0xF], 1); // VF should be LSB of ORIGINAL V0 (171), which is 1
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy7() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 5;
    chip8.registers[1] = 10;
    chip8.perform_opcode(0x8017); // V0 = V1 - V0 (10 - 5 = 5, no borrow)
    assert_eq!(chip8.registers[0], 5);
    assert_eq!(chip8.registers[0xF], 1); // No borrow (Vy > Vx), VF should be 1
    assert_eq!(chip8.program_counter, 0x202);

    let mut chip8 = Chip8::new();
    chip8.registers[0] = 10;
    chip8.registers[1] = 5;
    chip8.perform_opcode(0x8017); // V0 = V1 - V0 (5 - 10 = -5, borrow)
    assert_eq!(chip8.registers[0], 251); // 5 - 10 = -5, wrapped around 256 is 251
    assert_eq!(chip8.registers[0xF], 0); // Borrow (Vy < Vx), VF should be 0
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xyE() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0b01000000; // 64
    chip8.perform_opcode(0x800E); // V0 = V0 SHL 1, MSB is 0, no quirk
    assert_eq!(chip8.registers[0], 0b10000000); // 128
    assert_eq!(chip8.registers[0xF], 0);
    assert_eq!(chip8.program_counter, 0x202);

    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0b11000000; // 192
    chip8.perform_opcode(0x800E); // V0 = V0 SHL 1, MSB is 1, no quirk
    assert_eq!(chip8.registers[0], 0b10000000); // 192 << 1 = 384, wrapped around 256 is 128
    assert_eq!(chip8.registers[0xF], 1);
    assert_eq!(chip8.program_counter, 0x202);

    // Test with quirk: Vx=Vy and VF taken *before* overwrite
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0b11000000; // Vx = 192, MSB 1
    chip8.registers[1] = 0b00000001; // Vy = 1
    chip8.shift_quirk_vx_eq_vy = true;
    chip8.perform_opcode(0x801E); // V0 = V1 then SHL 1
    assert_eq!(chip8.registers[0], 0b00000010); // V0 should be 2 after taking V1 and shifting
    assert_eq!(chip8.registers[0xF], 1); // VF should be MSB of ORIGINAL V0 (192), which is 1
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_9xy0() {
    let mut chip8 = Chip8::new();
    chip8.registers[2] = 0x10;
    chip8.registers[3] = 0x11;
    chip8.perform_opcode(0x9230); // Vx (0x10) does not equal Vy (0x11), PC should skip
    assert_eq!(chip8.program_counter, 0x204);

    let mut chip8 = Chip8::new(); // Reset for next test case
    chip8.registers[2] = 0x10;
    chip8.registers[3] = 0x10;
    chip8.perform_opcode(0x9230); // Vx (0x10) equals Vy (0x10), PC should not skip
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Annn() {
    let mut chip8 = Chip8::new();
    chip8.perform_opcode(0xA123);
    assert_eq!(chip8.i, 0x123);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Bnnn() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0x10;
    chip8.perform_opcode(0xB100); // PC = V0 + 0x100 (0x10 + 0x100 = 0x110)
    assert_eq!(chip8.program_counter, 0x110);
}

#[test]
fn test_opcode_Cxkk() {
    let mut chip8 = Chip8::new();
    // This test is tricky as rand::Rng is used.
    // For a deterministic test, you would need to mock or control the RNG.
    // Assuming a test where a specific outcome is expected for random number generation.
    // For now, let's just ensure PC increments.
    chip8.perform_opcode(0xC0FF); // V0 = rand & 0xFF
    assert_ne!(chip8.registers[0], 0); // V0 should not be 0 often
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Dxyn() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0; // Vx = 0
    chip8.registers[1] = 0; // Vy = 0
    chip8.i = 0x300; // I points to sprite data
    chip8.ram[0x300] = 0xF0; // Simple 1x1 sprite (top row only for simplicity in test)
    chip8.screen[0][0] = 0; // Ensure pixel is off initially

    // Draw sprite, no collision
    chip8.perform_opcode(0xD011); // Draw 1-byte sprite at (V0, V1) from I
    assert_eq!(chip8.registers[0xF], 0); // No collision, VF should be 0
    assert_eq!(chip8.screen[0][0], 1); // Pixel should be on
    assert_eq!(chip8.program_counter, 0x202);

    // Collision (draw same sprite again)
    chip8.perform_opcode(0xD011);
    assert_eq!(chip8.registers[0xF], 1); // Collision, VF should be 1
    assert_eq!(chip8.screen[0][0], 0); // Pixel should be off (erased)
    assert_eq!(chip8.program_counter, 0x204);
}

#[test]
fn test_opcode_Ex9E() {
    let mut chip8 = Chip8::new();

    chip8.registers[0] = 0x1; // V0 holds key 1
    chip8.keyboard = 0x1; // Key 1 is pressed
    chip8.perform_opcode(0xE09E); // Skip if key in V0 is pressed
    assert_eq!(chip8.program_counter, 0x204); // PC should skip

    chip8.program_counter = 0x200; // Reset PC
    chip8.registers[0] = 0x2; // V0 holds key 2
    chip8.keyboard = 0x1; // Key 1 is pressed, not key 2
    chip8.perform_opcode(0xE09E);
    assert_eq!(chip8.program_counter, 0x202); // PC should not skip
}

#[test]
fn test_opcode_ExA1() {
    let mut chip8 = Chip8::new();

    // Scenario 1: Key in Vx IS pressed (e.g., Vx=0x5, keyboard=0x5) - Should NOT skip (PC + 2)
    chip8.program_counter = 0x200;
    chip8.registers[0] = 0x5; // V0 holds key 5
    chip8.keyboard = 0x5; // Key 5 IS pressed
    chip8.perform_opcode(0xE0A1); // Opcode is E0A1 (ExA1 where X=0)
    assert_eq!(
        chip8.program_counter, 0x202,
        "ExA1: Should NOT skip when Vx key IS pressed."
    );

    // Scenario 2: Key in Vx IS NOT pressed (no keys pressed at all) - Should SKIP (PC + 4)
    chip8.program_counter = 0x200;
    chip8.registers[0] = 0xA; // V0 holds key A
    chip8.keyboard = 0xFF; // No keys are pressed (as set by your input function)
    chip8.perform_opcode(0xE0A1); // Opcode is E0A1 (ExA1 where X=0)
    assert_eq!(
        chip8.program_counter, 0x204,
        "ExA1: Should SKIP when Vx key is NOT pressed (no keys active)."
    );

    // Scenario 3: Key in Vx IS NOT pressed (a DIFFERENT key is pressed, e.g., Vx=0x3, keyboard=0x7) - Should SKIP (PC + 4)
    chip8.program_counter = 0x200;
    chip8.registers[0] = 0x3; // V0 holds key 3
    chip8.keyboard = 0x7; // Key 7 IS pressed, but not key 3
    chip8.perform_opcode(0xE0A1); // Opcode is E0A1 (ExA1 where X=0)
    assert_eq!(
        chip8.program_counter, 0x204,
        "ExA1: Should SKIP when Vx key is NOT pressed (different key active)."
    );

    // Your original test case 1, with an adjusted comment for clarity of intent
    chip8.program_counter = 0x200;
    chip8.keyboard = 0x0; // Simulating key 0 as the ONLY pressed key
    chip8.registers[0] = 0x0; // V0 also holds key 0
    chip8.perform_opcode(0xE0A1);
    assert_eq!(
        chip8.program_counter, 0x202,
        "ExA1: Original TC1: Vx key IS pressed (key 0 pressed). Should NOT skip."
    );

    // Your original test case 2
    chip8.program_counter = 0x200;
    chip8.keyboard = 0x1; // Simulating key 1 as the ONLY pressed key
    chip8.registers[0] = 0x0; // V0 holds key 0 (which is NOT key 1)
    chip8.perform_opcode(0xE0A1);
    assert_eq!(
        chip8.program_counter, 0x204,
        "ExA1: Original TC2: Vx key is NOT pressed (key 1 is pressed). Should SKIP."
    );
}

#[test]
fn test_opcode_Fx07() {
    let mut chip8 = Chip8::new();

    chip8.delay_timer = 10;
    chip8.perform_opcode(0xF007);
    assert_eq!(chip8.registers[0], 10);
    assert_eq!(chip8.program_counter, 0x202);
}

// test Fx0A (Needs blocking input simulation, often more complex for unit tests)

#[test]
fn test_opcode_Fx15() {
    let mut chip8 = Chip8::new();

    chip8.registers[0] = 10;
    chip8.perform_opcode(0xF015);
    assert_eq!(chip8.delay_timer, 10);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Fx18() {
    let mut chip8 = Chip8::new();

    chip8.registers[0] = 10;
    chip8.perform_opcode(0xF018);
    assert_eq!(chip8.sound_timer, 10);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Fx1E() {
    let mut chip8 = Chip8::new();
    chip8.i = 0x100;
    chip8.registers[0] = 0x05;
    chip8.perform_opcode(0xF01E); // I = I + V0 (0x100 + 0x05 = 0x105)
    assert_eq!(chip8.i, 0x105);
    assert_eq!(chip8.program_counter, 0x202);

    let mut chip8 = Chip8::new();
    chip8.i = 0xFF0;
    chip8.registers[0] = 0x20;
    chip8.perform_opcode(0xF01E); // I = I + V0 (0xFF0 + 0x20 = 0x1010, no overflow beyond 0xFFF)
    assert_eq!(chip8.i, 0x1010);
    // VF should not be set for Fx1E, so it should remain 0 (or whatever it was)
    assert_eq!(chip8.registers[0xF], 0); // Assuming VF is 0 initially and not changed by Fx1E
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Fx29() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0x5; // V0 = 5 (character '5')
    chip8.perform_opcode(0xF029);
    // Character '5' font data starts at (5 * 5) = 25 in CHIP8_FONT
    assert_eq!(chip8.i, 25);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Fx33() {
    let mut chip8 = Chip8::new();
    chip8.i = 0x300;
    chip8.registers[0] = 123; // V0 = 123
    chip8.perform_opcode(0xF033);
    assert_eq!(chip8.ram[0x300], 1); // Hundreds digit
    assert_eq!(chip8.ram[0x301], 2); // Tens digit
    assert_eq!(chip8.ram[0x302], 3); // Ones digit
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Fx55() {
    let mut chip8 = Chip8::new();
    chip8.i = 0x300;
    chip8.registers[0] = 10;
    chip8.registers[1] = 20;
    chip8.registers[2] = 30;
    chip8.perform_opcode(0xF255); // Store V0, V1, V2
    assert_eq!(chip8.ram[0x300], 10);
    assert_eq!(chip8.ram[0x301], 20);
    assert_eq!(chip8.ram[0x302], 30);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_Fx65() {
    let mut chip8 = Chip8::new();
    chip8.i = 0x300;
    chip8.ram[0x300] = 10;
    chip8.ram[0x301] = 20;
    chip8.ram[0x302] = 30;
    chip8.perform_opcode(0xF265); // Read into V0, V1, V2
    assert_eq!(chip8.registers[0], 10);
    assert_eq!(chip8.registers[1], 20);
    assert_eq!(chip8.registers[2], 30);
    assert_eq!(chip8.program_counter, 0x202);
}
