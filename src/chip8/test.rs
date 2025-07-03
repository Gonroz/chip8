use super::Chip8;

// const KEYCODE_FOR_TEST = KeyCode::Key1;

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
    assert_eq!(chip8.registers[2], 0x10);
    assert_eq!(chip8.program_counter, 0x204);

    chip8.registers[2] = 0x11;
    chip8.program_counter = 0x200;
    chip8.perform_opcode(0x3210);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_4xkk() {
    let mut chip8 = Chip8::new();
    chip8.registers[2] = 0x10;
    chip8.perform_opcode(0x4210);
    assert_eq!(chip8.registers[2], 0x10);
    assert_eq!(chip8.program_counter, 0x202);

    chip8.registers[2] = 0x11;
    chip8.program_counter = 0x200;
    chip8.perform_opcode(0x4210);
    assert_eq!(chip8.program_counter, 0x204);
}

#[test]
fn test_opcode_5xy0() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0x1;
    chip8.registers[1] = 0x1;
    chip8.perform_opcode(0x5010);
    assert_eq!(chip8.program_counter, 0x204);

    chip8.program_counter = 0x200;
    chip8.registers[1] = 0x0;
    chip8.perform_opcode(0x5010);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_6xkk() {
    let mut chip8 = Chip8::new();
    chip8.perform_opcode(0x6011);
    assert_eq!(chip8.registers[0], 0x11);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_7xkk() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0x10;
    chip8.perform_opcode(0x7001);
    assert_eq!(chip8.registers[0], 0x11);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy0() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0x10;
    chip8.registers[1] = 0x0;
    chip8.perform_opcode(0x8010);
    assert_eq!(chip8.registers[0], 0x0);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy1() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0x10;
    chip8.registers[1] = 0x01;
    chip8.perform_opcode(0x8011);
    assert_eq!(chip8.registers[0], 0x11);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy2() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0x01;
    chip8.registers[1] = 0x10;
    chip8.perform_opcode(0x8012);
    assert_eq!(chip8.registers[0], 0x0);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy3() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0x01;
    chip8.registers[1] = 0x11;
    chip8.perform_opcode(0x8013);
    assert_eq!(chip8.registers[0], 0x10);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy4() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0xFF;
    chip8.registers[1] = 0x01;
    chip8.perform_opcode(0x8014);
    assert_eq!(chip8.registers[0], 0x00);
    assert_eq!(chip8.vf, 1);
    assert_eq!(chip8.program_counter, 0x202);

    chip8.program_counter = 0x200;
    chip8.registers[0] = 0x01;
    chip8.registers[1] = 0x01;
    chip8.perform_opcode(0x8014);
    assert_eq!(chip8.registers[0], 0x02);
    assert_eq!(chip8.vf, 0);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy5() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 0xFF;
    chip8.registers[1] = 0x01;
    chip8.perform_opcode(0x8015);
    assert_eq!(chip8.registers[0], 0xFE);
    assert_eq!(chip8.vf, 1);
    assert_eq!(chip8.program_counter, 0x202);

    chip8.program_counter = 0x200;
    chip8.registers[0] = 0x00;
    chip8.registers[1] = 0xFF;
    chip8.perform_opcode(0x8015);
    assert_eq!(chip8.registers[0], 0x01);
    assert_eq!(chip8.vf, 0);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy6() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 10;
    chip8.registers[1] = 2;
    chip8.perform_opcode(0x8016);
    assert_eq!(chip8.registers[0], 5);
    assert_eq!(chip8.vf, 0);
    assert_eq!(chip8.program_counter, 0x202);

    chip8.program_counter = 0x200;
    chip8.registers[0] = 11;
    chip8.registers[1] = 2;
    chip8.perform_opcode(0x8016);
    assert_eq!(chip8.registers[0], 5);
    assert_eq!(chip8.vf, 1);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xy7() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 1;
    chip8.registers[1] = 255;
    chip8.perform_opcode(0x8017);
    assert_eq!(chip8.registers[0], 254);
    assert_eq!(chip8.vf, 1);
    assert_eq!(chip8.program_counter, 0x202);

    chip8.program_counter = 0x200;
    chip8.registers[0] = 11;
    chip8.registers[1] = 2;
    chip8.perform_opcode(0x8017);
    assert_eq!(chip8.registers[0], 247);
    assert_eq!(chip8.vf, 0);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_8xyE() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 1;
    chip8.perform_opcode(0x801E);
    assert_eq!(chip8.registers[0], 2);
    assert_eq!(chip8.vf, 0);
    assert_eq!(chip8.program_counter, 0x202);

    chip8.program_counter = 0x200;
    chip8.registers[0] = 128;
    chip8.perform_opcode(0x801E);
    assert_eq!(chip8.registers[0], 0);
    assert_eq!(chip8.vf, 1);
    assert_eq!(chip8.program_counter, 0x202);
}

#[test]
fn test_opcode_9xy0() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 1;
    chip8.registers[1] = 1;
    chip8.perform_opcode(0x9010);
    assert_eq!(chip8.program_counter, 0x202);

    chip8.registers[0] = 0;
    chip8.program_counter = 0x200;
    chip8.perform_opcode(0x9010);
    assert_eq!(chip8.program_counter, 0x204);
}

#[test]
fn test_opcode_Annn() {
    let mut chip8 = Chip8::new();
    chip8.perform_opcode(0xA001);
    assert_eq!(chip8.i, 1);
}

#[test]
fn test_opcode_Bnnn() {
    let mut chip8 = Chip8::new();
    chip8.registers[0] = 2;
    chip8.perform_opcode(0xB1FE);
    assert_eq!(chip8.program_counter, 0x200);
}

#[test]
fn test_opcode_Cxkk() {
    // random int
}

#[test]
fn test_opcode_Dxyn() {
    let mut chip8 = Chip8::new();
    chip8.ram[0x200] = 0b11111111;
    chip8.i = 0x200;
    chip8.registers[0] = 0;
    chip8.registers[1] = 0;

    // Make sure that screen is blank
    for i in 0..7 {
        assert_eq!(chip8.screen[0][i], 0);
    }

    chip8.perform_opcode(0xD011);
    chip8.screen[0][0] = 1;
    assert_eq!(chip8.screen[0][0], 1);

    // Make sure that it alternates being on and off
    for i in 0..8 {
        if i % 2 == 0 {
            assert_eq!(chip8.screen[0][i], 1)
        } else {
            // println!("{}", i);
            assert_eq!(chip8.screen[0][i], 1);
        }
    }
}
