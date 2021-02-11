mod monster_generated;
use flatbuffers::FlatBufferBuilder;
use monster_generated::my_game::{
    finish_monster_buffer, Any, Color, Monster, MonsterArgs,
};
use std::fs::write;

fn main() {
    let mut builder = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    bytes.clear();
    builder.reset();

    let monster_args = MonsterArgs {
        pos: None,
        mana: 10,
        hp: 8,
        name: Some((builder).create_string("Cute girl")),
        inventory: None,
        color: Color::Blue,
        test_type: Any::NONE,
        test: None,
    };
    let monster_offset = Monster::create(&mut builder, &monster_args);

    finish_monster_buffer(&mut builder, monster_offset);
    bytes.extend_from_slice(builder.finished_data());

    write("monster.raw", bytes).unwrap();
}
