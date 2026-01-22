#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, Address, Env, String};
mod imported {
    pub const WASM: &[u8] = b"\0asm\x01\0\0\0\x019\n`\x01~\x01~`\x02~~\x01~`\x03~~~\x01~`\x02\x7f~\0`\x03\x7f~~\0`\x04\x7f\x7f\x7f\x7f\x01~`\x02\x7f\x7f\x01~`\0\x01~`\x01\x7f\0`\x01\x7f\x01~\x02I\x0c\x01i\x015\0\0\x01i\x014\0\0\x01i\x013\0\x01\x01x\x011\0\x01\x01i\x018\0\0\x01i\x017\0\0\x01i\x016\0\x01\x01v\x01g\0\x01\x01b\x01j\0\x01\x01m\x019\0\x02\x01i\x012\0\0\x01i\x011\0\0\x03\x16\x15\x03\x04\x01\x05\x01\x03\x03\x01\x06\x01\x07\x08\0\x07\x07\0\0\0\x01\t\x02\x05\x03\x01\0\x11\x06!\x04\x7f\x01A\x80\x80\xc0\0\x0b\x7f\0A\x82\x80\xc0\0\x0b\x7f\0A\xdc\x81\xc0\0\x0b\x7f\0A\xe0\x81\xc0\0\x0b\x07\xf5\x01\x11\x06memory\x02\0\x0fcreate_struct_a\0\x0e\x0fcreate_struct_b\0\x10\x15create_struct_tuple_a\0\x13\x15create_struct_tuple_b\0\x15\nget_enum_a\0\x16\nget_enum_b\0\x18\x0eget_enum_int_a\0\x19\x0eget_enum_int_b\0\x1a\x07check_a\0\x1b\x07check_b\0\x1c\x07check_c\0\x1d\x0cemit_event_a\0\x1e\x0cemit_event_b\0 \x01_\x03\x01\n__data_end\x03\x02\x0b__heap_base\x03\x03\n\xae\x13\x15{\x02\x01\x7f\x01~\x02@\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc4\0F\r\0 \x02A\nG\r\x01B\0!\x03 \0B\07\x03\x10 \0 \x01B\x08\x887\x03\x08\x0c\x02\x0b \x01\x10\x80\x80\x80\x80\0!\x03 \x01\x10\x81\x80\x80\x80\0!\x01 \0 \x037\x03\x10 \0 \x017\x03\x08B\0!\x03\x0c\x01\x0b \0B\x83\x90\x80\x80\x80\x017\x03\x08B\x01!\x03\x0b \0 \x037\x03\0\x0bF\0\x02@\x02@ \x01B\xff\xff\xff\xff\xff\xff\xff\xff\0V \x02B\0R \x02P\x1b\r\0 \x01B\x08\x86B\n\x84!\x02\x0c\x01\x0b \x02 \x01\x10\x82\x80\x80\x80\0!\x02\x0b \0B\07\x03\0 \0 \x027\x03\x08\x0b\x84\x01\x01\x02\x7f#\x80\x80\x80\x80\0A\x10k\"\x02$\x80\x80\x80\x80\0\x02@ \0B\xff\x01\x83B\x04R\r\0A\x01 \x01\xa7A\xff\x01q\"\x03A\0GA\x01t \x03A\x01F\x1b\"\x03A\x02F\r\0A\0-\0\x82\x80\xc0\x80\0\x1a \x02 \x03\xad7\x03\x08 \x02 \0B\x84\x80\x80\x80p\x837\x03\0A\x94\x80\xc0\x80\0A\x02 \x02A\x02\x10\x8f\x80\x80\x80\0!\0 \x02A\x10j$\x80\x80\x80\x80\0 \0\x0f\x0b\0\x0b.\0\x02@ \x01 \x03F\r\0\0\x0b \0\xadB \x86B\x04\x84 \x02\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x89\x80\x80\x80\0\x0b\x9b\x01\x01\x01\x7f#\x80\x80\x80\x80\0A k\"\x02$\x80\x80\x80\x80\0 \x02A\x10j \0\x10\x91\x80\x80\x80\0\x02@ \x02(\x02\x10A\x01F\r\0 \x01B\xff\x01\x83B\xc9\0R\r\0 \x02)\x03\x18!\0A\0-\0\xa4\x80\xc0\x80\0\x1a \x02A\x10j \0\x10\x92\x80\x80\x80\0 \x02(\x02\x10A\x01F\r\0 \x02)\x03\x18!\0 \x02 \x017\x03\x08 \x02 \07\x03\0A\x94\x80\xc0\x80\0A\x02 \x02A\x02\x10\x8f\x80\x80\x80\0!\x01 \x02A j$\x80\x80\x80\x80\0 \x01\x0f\x0b\0\x0b]\x02\x01\x7f\x01~\x02@\x02@ \x01\xa7A\xff\x01q\"\x02A\xc1\0F\r\0\x02@ \x02A\x07F\r\0B\x01!\x03B\x83\x90\x80\x80\x80\x01!\x01\x0c\x02\x0b \x01B\x08\x87!\x01B\0!\x03\x0c\x01\x0bB\0!\x03 \x01\x10\x8a\x80\x80\x80\0!\x01\x0b \0 \x037\x03\0 \0 \x017\x03\x08\x0bF\0\x02@\x02@ \x01B\x80\x80\x80\x80\x80\x80\x80\xc0\0|B\xff\xff\xff\xff\xff\xff\xff\xff\0V\r\0 \x01B\x08\x86B\x07\x84!\x01\x0c\x01\x0b \x01\x10\x8b\x80\x80\x80\0!\x01\x0b \0B\07\x03\0 \0 \x017\x03\x08\x0b\xbc\x01\x01\x01\x7f#\x80\x80\x80\x80\0A k\"\x02$\x80\x80\x80\x80\0 \x02A\x10j \0\x10\x91\x80\x80\x80\0\x02@ \x02(\x02\x10A\x01F\r\0 \x02)\x03\x18!\0 \x02A\x10j \x01\x10\x91\x80\x80\x80\0 \x02(\x02\x10A\x01F\r\0 \x02)\x03\x18!\x01A\0-\0\xb0\x80\xc0\x80\0\x1a \x02A\x10j \0\x10\x92\x80\x80\x80\0 \x02(\x02\x10\r\0 \x02)\x03\x18!\0 \x02A\x10j \x01\x10\x92\x80\x80\x80\0 \x02(\x02\x10A\x01F\r\0 \x02 \x02)\x03\x187\x03\x08 \x02 \07\x03\0 \x02A\x02\x10\x94\x80\x80\x80\0!\0 \x02A j$\x80\x80\x80\x80\0 \0\x0f\x0b\0\x0b\x1a\0 \0\xadB \x86B\x04\x84 \x01\xadB \x86B\x04\x84\x10\x87\x80\x80\x80\0\x0b\xd8\x01\x02\x02\x7f\x02~#\x80\x80\x80\x80\0A0k\"\x02$\x80\x80\x80\x80\0 \x02A\x08j \0\x10\x8c\x80\x80\x80\0\x02@ \x02(\x02\x08A\x01F\r\0 \x02A\x18j\"\x03)\x03\0!\0 \x02)\x03\x10!\x04 \x02A\x08j \x01\x10\x8c\x80\x80\x80\0 \x02(\x02\x08A\x01F\r\0A\0-\0\xbc\x80\xc0\x80\0\x1a \x03)\x03\0!\x01 \x02)\x03\x10!\x05 \x02A\x08j \x04 \0\x10\x8d\x80\x80\x80\0 \x02(\x02\x08\r\0 \x02)\x03\x10!\0 \x02A\x08j \x05 \x01\x10\x8d\x80\x80\x80\0 \x02(\x02\x08A\x01F\r\0 \x02 \x02)\x03\x107\x03( \x02 \07\x03  \x02A jA\x02\x10\x94\x80\x80\x80\0!\0 \x02A0j$\x80\x80\x80\x80\0 \0\x0f\x0b\0\x0bZ\x02\x01\x7f\x01~#\x80\x80\x80\x80\0A\x10k\"\0$\x80\x80\x80\x80\0A\0-\0\xc8\x80\xc0\x80\0\x1a \0\x10\x97\x80\x80\x80\0\x02@ \0(\x02\0A\x01G\r\0\0\x0b \0 \0)\x03\x087\x03\0 \0A\x01\x10\x94\x80\x80\x80\0!\x01 \0A\x10j$\x80\x80\x80\x80\0 \x01\x0b\x83\x02\x03\x01\x7f\x01~\x03\x7f#\x80\x80\x80\x80\0A\x10k\"\x01$\x80\x80\x80\x80\0B\0!\x02A~!\x03\x02@\x02@\x02@\x03@ \x03E\r\x01A\x01!\x04\x02@ \x03A\x82\x80\xc0\x80\0j-\0\0\"\x05A\xdf\0F\r\0\x02@ \x05APjA\xff\x01qA\nI\r\0\x02@ \x05A\xbf\x7fjA\xff\x01qA\x1aI\r\0 \x05A\x9f\x7fjA\xff\x01qA\x19K\r\x05 \x05AEj!\x04\x0c\x02\x0b \x05AKj!\x04\x0c\x01\x0b \x05ARj!\x04\x0b \x02B\x06\x86 \x04\xadB\xff\x01\x83\x84!\x02 \x03A\x01j!\x03\x0c\0\x0b\x0b \x01 \x02B\x08\x86B\x0e\x84\"\x027\x02\x04\x0c\x01\x0b \x01 \x05\xadB\x08\x86B\x01\x847\x03\0A\x80\x80\xc0\x80\0\xadB \x86B\x04\x84B\x84\x80\x80\x80 \x10\x88\x80\x80\x80\0!\x02\x0b \0B\07\x03\0 \0 \x027\x03\x08 \x01A\x10j$\x80\x80\x80\x80\0\x0b\x95\x01\x02\x01\x7f\x01~#\x80\x80\x80\x80\0A\x10k\"\x01$\x80\x80\x80\x80\0 \x01 \0\x10\x91\x80\x80\x80\0\x02@ \x01(\x02\0A\x01F\r\0 \x01)\x03\x08!\0A\0-\0\xd4\x80\xc0\x80\0\x1a \x01\x10\x97\x80\x80\x80\0 \x01(\x02\0\r\0 \x01)\x03\x08!\x02 \x01 \0\x10\x92\x80\x80\x80\0 \x01(\x02\0A\x01F\r\0 \x01 \x01)\x03\x087\x03\x08 \x01 \x027\x03\0 \x01A\x02\x10\x94\x80\x80\x80\0!\0 \x01A\x10j$\x80\x80\x80\x80\0 \0\x0f\x0b\0\x0b\x12\0A\0-\0\xe0\x80\xc0\x80\0\x1aB\x84\x80\x80\x800\x0b\x13\0A\0-\0\xec\x80\xc0\x80\0\x1aB\x84\x80\x80\x80\xc0\x02\x0b4\0\x02@ \0B\xff\x01\x83B\x04Q\r\0\0\x0bA\0-\0\xf8\x80\xc0\x80\0\x1aB\x83\x80\x80\x80  \0B\x84\x80\x80\x80p\x83 \0B\x80\x80\x80\x80\x10T\x1b\x0b9\0\x02@ \0B\xff\x01\x83B\x04Q\r\0\0\x0bA\0-\0\x84\x81\xc0\x80\0\x1aB\x83\x80\x80\x80\xc0\x01 \0B\x84\x80\x80\x80\xf0\xff\0\x83 \0B\xff\xff\xff\xff\x8f\xfd\0V\x1b\x0b6\0\x02@ \0B\xff\x01\x83B\x04Q\r\0\0\x0bA\0-\0\x90\x81\xc0\x80\0\x1aB\x83\x80\x80\x80\xc0\x0c \0B\x84\x80\x80\x80p\x83 \0B\x80\x80\x80\x80\xa0\x01T\x1b\x0b\xef\x01\x02\x02\x7f\x01~#\x80\x80\x80\x80\0A k\"\x02$\x80\x80\x80\x80\0\x02@ \0B\xff\x01\x83B\xcd\0R\r\0 \x01B\xff\x01\x83B\xc9\0R\r\0A\0!\x03A\0-\0\x9c\x81\xc0\x80\0\x1aA\xa8\x81\xc0\x80\0\x10\x9f\x80\x80\x80\0!\x04 \x02 \07\x03\x08 \x02 \x047\x03\0\x03@\x02@ \x03A\x10G\r\0A\0!\x03\x02@\x03@ \x03A\x10F\r\x01 \x02A\x10j \x03j \x02 \x03j)\x03\07\x03\0 \x03A\x08j!\x03\x0c\0\x0b\x0b \x02A\x10jA\x02\x10\x94\x80\x80\x80\0!\0 \x02 \x017\x03\x10 \0A\xb0\x81\xc0\x80\0A\x01 \x02A\x10jA\x01\x10\x8f\x80\x80\x80\0\x10\x83\x80\x80\x80\0\x1a \x02A j$\x80\x80\x80\x80\0B\x02\x0f\x0b \x02A\x10j \x03jB\x027\x03\0 \x03A\x08j!\x03\x0c\0\x0b\x0b\0\x0b\x07\0 \0)\x03\0\x0b\x81\x03\x02\x02\x7f\x02~#\x80\x80\x80\x80\0A0k\"\x03$\x80\x80\x80\x80\0\x02@ \0B\xff\x01\x83B\xcd\0R\r\0 \x01B\xff\x01\x83B\xcd\0R\r\0\x02@\x02@ \x02\xa7A\xff\x01q\"\x04A\xc5\0F\r\0 \x04A\x0bG\r\x02 \x02B?\x87!\x05 \x02B\x08\x87!\x02\x0c\x01\x0b \x02\x10\x84\x80\x80\x80\0!\x05 \x02\x10\x85\x80\x80\x80\0!\x02\x0bA\0!\x04A\0-\0\xb8\x81\xc0\x80\0\x1aA\xc8\x81\xc0\x80\0\x10\x9f\x80\x80\x80\0!\x06 \x03 \x017\x03\x10 \x03 \07\x03\x08 \x03 \x067\x03\0\x03@\x02@ \x04A\x18G\r\0A\0!\x04\x02@\x03@ \x04A\x18F\r\x01 \x03A\x18j \x04j \x03 \x04j)\x03\07\x03\0 \x04A\x08j!\x04\x0c\0\x0b\x0b \x03A\x18jA\x03\x10\x94\x80\x80\x80\0!\0\x02@\x02@ \x02B\x80\x80\x80\x80\x80\x80\x80\xc0\0|B\xff\xff\xff\xff\xff\xff\xff\xff\0V\r\0 \x02 \x02\x85 \x05 \x02B?\x87\x85\x84B\0R\r\0 \x02B\x08\x86B\x0b\x84!\x02\x0c\x01\x0b \x05 \x02\x10\x86\x80\x80\x80\0!\x02\x0b \x03 \x027\x03\x18 \0A\xd4\x81\xc0\x80\0A\x01 \x03A\x18jA\x01\x10\x8f\x80\x80\x80\0\x10\x83\x80\x80\x80\0\x1a \x03A0j$\x80\x80\x80\x80\0B\x02\x0f\x0b \x03A\x18j \x04jB\x027\x03\0 \x04A\x08j!\x04\x0c\0\x0b\x0b\0\x0b\x0b\xe6\x01\x01\0A\x80\x80\xc0\0\x0b\xdc\x01V2SpEc\xb6\x1c\xfd\xdfhY-df1f2\0\0\x0e\0\x10\0\x02\0\0\0\x10\0\x10\0\x02\0\0\0SpEc\xf3\xc4\xd3\x8c\xc1w\xe9\x18SpEc\xcf)\x97]S\xb2\xfd)SpEcx\xd98\x9c\x1ao\xac\x8cSpEc\xa2=N\xc1p\x95\x90\xb2SpEc'\x1b\0DSH^\xccSpEcV]\x80\\~\x1a\x08/SpEc,\x9c\xc0_\xed_)\x85SpEc\xe9R\xa7\xe8b\x99\xa2\xc3SpEc\x1d1\xd6\xfb\x88\xd2=\xe3SpEc\xb9\x01\xafj\xe0c\xa3\rSpEcK\xe6\x8ej\x19\x9en\xbd\x0ef\x90\xcf\xea\xae\x02\0\x10\0\x10\0\x02\0\0\0SpEc\xe6\xaa\xefz\x17i$\x15\0\0\0\0\x0eg\x90\xcf\xea\xae\x02\0f3\0\0\xd0\0\x10\0\x02\0\0\0\0\xcf\x12\x0econtractspecv0\0\0\0\0\0\0\0\0\0\0\0\x0fcreate_struct_a\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x01\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructA\0\0\0\0\0\0\0\0\0\0\0\0\x0fcreate_struct_b\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructB\0\0\0\0\0\0\0\0\0\0\0\0\x15create_struct_tuple_a\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleA\0\0\0\0\0\0\0\0\0\0\0\x15create_struct_tuple_b\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\n\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleB\0\0\0\0\0\0\0\0\0\0\0\nget_enum_a\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x05EnumA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\nget_enum_b\0\0\0\0\0\x01\0\0\0\0\0\0\0\x05value\0\0\0\0\0\0\x07\0\0\0\x01\0\0\x07\xd0\0\0\0\x05EnumB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0eget_enum_int_a\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x08EnumIntA\0\0\0\0\0\0\0\0\0\0\0\x0eget_enum_int_b\0\0\0\0\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x08EnumIntB\0\0\0\0\0\0\0\0\0\0\0\x07check_a\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorA\0\0\0\0\0\0\0\0\0\0\0\0\0\x07check_b\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorB\0\0\0\0\0\0\0\0\0\0\0\0\0\x07check_c\0\0\0\0\x01\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\x03\xe9\0\0\0\x04\0\0\x07\xd0\0\0\0\x06ErrorC\0\0\0\0\0\0\0\0\0\0\0\0\0\x0cemit_event_a\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x0cemit_event_b\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructA\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x01\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructB\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructC\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\x03\xea\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleA\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\x07\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleB\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\n\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\n\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleC\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x13\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\x0b\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumA\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\0\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\0\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumB\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x01\0\0\0\x07\0\0\0\x01\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x02\0\0\0\x07\0\0\0\x07\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumC\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructA\0\0\0\0\x01\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleA\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntA\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x03\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntB\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x14\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x1e\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntC\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0d\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\xc8\0\0\0\0\0\0\0\x02V3\0\0\0\0\x01,\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorA\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02E2\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02E3\0\0\0\0\0\x03\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorB\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02E2\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02E3\0\0\0\0\0\x0c\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorC\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0d\0\0\0\0\0\0\0\x02E2\0\0\0\0\0e\0\0\0\0\0\0\0\x02E3\0\0\0\0\0f\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventA\0\0\0\0\0\x01\0\0\0\x07event_a\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0\0\0\0\x02\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventB\0\0\0\0\0\x01\0\0\0\x07event_b\0\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventC\0\0\0\0\0\x01\0\0\0\x07event_c\0\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x11\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02\0\x1e\x11contractenvmetav0\0\0\0\0\0\0\0\x17\0\0\0\0\0+\x0econtractmetav0\0\0\0\0\0\0\0\x05rsver\0\0\0\0\0\0\x061.84.0\0\0";
    pub trait Contract {
        fn create_struct_a(env: soroban_sdk::Env, f1: u32, f2: bool) -> StructA;
        fn create_struct_b(env: soroban_sdk::Env, f1: i64, f2: soroban_sdk::String) -> StructB;
        fn create_struct_tuple_a(env: soroban_sdk::Env, f1: i64, f2: i64) -> StructTupleA;
        fn create_struct_tuple_b(env: soroban_sdk::Env, f1: u128, f2: u128) -> StructTupleB;
        fn get_enum_a(env: soroban_sdk::Env) -> EnumA;
        fn get_enum_b(env: soroban_sdk::Env, value: i64) -> EnumB;
        fn get_enum_int_a(env: soroban_sdk::Env) -> EnumIntA;
        fn get_enum_int_b(env: soroban_sdk::Env) -> EnumIntB;
        fn check_a(env: soroban_sdk::Env, input: u32) -> Result<u32, ErrorA>;
        fn check_b(env: soroban_sdk::Env, input: u32) -> Result<u32, ErrorB>;
        fn check_c(env: soroban_sdk::Env, input: u32) -> Result<u32, ErrorC>;
        fn emit_event_a(env: soroban_sdk::Env, f1: soroban_sdk::Address, f2: soroban_sdk::String);
        fn emit_event_b(
            env: soroban_sdk::Env,
            f1: soroban_sdk::Address,
            f2: soroban_sdk::Address,
            f3: i128,
        );
    }
    ///Client is a client for calling the contract defined in "Contract".
    pub struct Client<'a> {
        pub env: soroban_sdk::Env,
        pub address: soroban_sdk::Address,
        #[doc(hidden)]
        set_auths: Option<&'a [soroban_sdk::xdr::SorobanAuthorizationEntry]>,
        #[doc(hidden)]
        mock_auths: Option<&'a [soroban_sdk::testutils::MockAuth<'a>]>,
        #[doc(hidden)]
        mock_all_auths: bool,
        #[doc(hidden)]
        allow_non_root_auth: bool,
    }
    impl<'a> Client<'a> {
        pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
            Self {
                env: env.clone(),
                address: address.clone(),
                set_auths: None,
                mock_auths: None,
                mock_all_auths: false,
                allow_non_root_auth: false,
            }
        }
        /// Set authorizations in the environment which will be consumed by
        /// contracts when they invoke `Address::require_auth` or
        /// `Address::require_auth_for_args` functions.
        ///
        /// Requires valid signatures for the authorization to be successful.
        /// To mock auth without requiring valid signatures, use `mock_auths`.
        ///
        /// See `soroban_sdk::Env::set_auths` for more details and examples.
        pub fn set_auths(&self, auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry]) -> Self {
            Self {
                env: self.env.clone(),
                address: self.address.clone(),
                set_auths: Some(auths),
                mock_auths: self.mock_auths.clone(),
                mock_all_auths: false,
                allow_non_root_auth: false,
            }
        }
        /// Mock authorizations in the environment which will cause matching invokes
        /// of `Address::require_auth` and `Address::require_auth_for_args` to
        /// pass.
        ///
        /// See `soroban_sdk::Env::set_auths` for more details and examples.
        pub fn mock_auths(&self, mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>]) -> Self {
            Self {
                env: self.env.clone(),
                address: self.address.clone(),
                set_auths: self.set_auths.clone(),
                mock_auths: Some(mock_auths),
                mock_all_auths: false,
                allow_non_root_auth: false,
            }
        }
        /// Mock all calls to the `Address::require_auth` and
        /// `Address::require_auth_for_args` functions in invoked contracts,
        /// having them succeed as if authorization was provided.
        ///
        /// See `soroban_sdk::Env::mock_all_auths` for more details and
        /// examples.
        pub fn mock_all_auths(&self) -> Self {
            Self {
                env: self.env.clone(),
                address: self.address.clone(),
                set_auths: None,
                mock_auths: None,
                mock_all_auths: true,
                allow_non_root_auth: false,
            }
        }
        /// A version of `mock_all_auths` that allows authorizations that
        /// are not present in the root invocation.
        ///
        /// Refer to `mock_all_auths` documentation for details and
        /// prefer using `mock_all_auths` unless non-root authorization is
        /// required.
        ///
        /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
        /// for more details and examples.
        pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
            Self {
                env: self.env.clone(),
                address: self.address.clone(),
                set_auths: None,
                mock_auths: None,
                mock_all_auths: true,
                allow_non_root_auth: true,
            }
        }
    }
    impl<'a> Client<'a> {
        pub fn create_struct_a(&self, f1: &u32, f2: &bool) -> StructA {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_create_struct_a(
            &self,
            f1: &u32,
            f2: &bool,
        ) -> Result<
            Result<
                StructA,
                <StructA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn create_struct_b(&self, f1: &i64, f2: &soroban_sdk::String) -> StructB {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_create_struct_b(
            &self,
            f1: &i64,
            f2: &soroban_sdk::String,
        ) -> Result<
            Result<
                StructB,
                <StructB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn create_struct_tuple_a(&self, f1: &i64, f2: &i64) -> StructTupleA {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_create_struct_tuple_a(
            &self,
            f1: &i64,
            f2: &i64,
        ) -> Result<
            Result<
                StructTupleA,
                <StructTupleA as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        >{
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn create_struct_tuple_b(&self, f1: &u128, f2: &u128) -> StructTupleB {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_create_struct_tuple_b(
            &self,
            f1: &u128,
            f2: &u128,
        ) -> Result<
            Result<
                StructTupleB,
                <StructTupleB as soroban_sdk::TryFromVal<
                    soroban_sdk::Env,
                    soroban_sdk::Val,
                >>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        >{
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "create_struct_tuple_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn get_enum_a(&self) -> EnumA {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_a") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_get_enum_a(
            &self,
        ) -> Result<
            Result<
                EnumA,
                <EnumA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_a") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn get_enum_b(&self, value: &i64) -> EnumB {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_b") },
                ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_get_enum_b(
            &self,
            value: &i64,
        ) -> Result<
            Result<
                EnumB,
                <EnumB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_b") },
                ::soroban_sdk::Vec::from_array(&self.env, [value.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn get_enum_int_a(&self) -> EnumIntA {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_a") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_get_enum_int_a(
            &self,
        ) -> Result<
            Result<
                EnumIntA,
                <EnumIntA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_a") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn get_enum_int_b(&self) -> EnumIntB {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_b") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_get_enum_int_b(
            &self,
        ) -> Result<
            Result<
                EnumIntB,
                <EnumIntB as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "get_enum_int_b") },
                ::soroban_sdk::Vec::new(&self.env),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn check_a(&self, input: &u32) -> u32 {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_a");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_check_a(
            &self,
            input: &u32,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<ErrorA, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_a");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn check_b(&self, input: &u32) -> u32 {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_b");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_check_b(
            &self,
            input: &u32,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<ErrorB, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_b");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn check_c(&self, input: &u32) -> u32 {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_c");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_check_c(
            &self,
            input: &u32,
        ) -> Result<
            Result<
                u32,
                <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
            >,
            Result<ErrorC, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("check_c");
                    SYMBOL
                },
                ::soroban_sdk::Vec::from_array(&self.env, [input.into_val(&self.env)]),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn emit_event_a(&self, f1: &soroban_sdk::Address, f2: &soroban_sdk::String) -> () {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "emit_event_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_emit_event_a(
            &self,
            f1: &soroban_sdk::Address,
            f2: &soroban_sdk::String,
        ) -> Result<
            Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "emit_event_a") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [f1.into_val(&self.env), f2.into_val(&self.env)],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn emit_event_b(
            &self,
            f1: &soroban_sdk::Address,
            f2: &soroban_sdk::Address,
            f3: &i128,
        ) -> () {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    if self.allow_non_root_auth {
                        self.env.mock_all_auths_allowing_non_root_auth();
                    } else {
                        self.env.mock_all_auths();
                    }
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "emit_event_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        f1.into_val(&self.env),
                        f2.into_val(&self.env),
                        f3.into_val(&self.env),
                    ],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
        pub fn try_emit_event_b(
            &self,
            f1: &soroban_sdk::Address,
            f2: &soroban_sdk::Address,
            f3: &i128,
        ) -> Result<
            Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
            Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
        > {
            use core::ops::Not;
            let old_auth_manager = self
                .env
                .in_contract()
                .not()
                .then(|| self.env.host().snapshot_auth_manager().unwrap());
            {
                if let Some(set_auths) = self.set_auths {
                    self.env.set_auths(set_auths);
                }
                if let Some(mock_auths) = self.mock_auths {
                    self.env.mock_auths(mock_auths);
                }
                if self.mock_all_auths {
                    self.env.mock_all_auths();
                }
            }
            use soroban_sdk::{FromVal, IntoVal};
            let res = self.env.try_invoke_contract(
                &self.address,
                &{ soroban_sdk::Symbol::new(&self.env, "emit_event_b") },
                ::soroban_sdk::Vec::from_array(
                    &self.env,
                    [
                        f1.into_val(&self.env),
                        f2.into_val(&self.env),
                        f3.into_val(&self.env),
                    ],
                ),
            );
            if let Some(old_auth_manager) = old_auth_manager {
                self.env.host().set_auth_manager(old_auth_manager).unwrap();
            }
            res
        }
    }
    ///Args is a type for building arg lists for functions defined in "Contract".
    pub struct Args;
    impl Args {
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn create_struct_a<'i>(f1: &'i u32, f2: &'i bool) -> (&'i u32, &'i bool) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn create_struct_b<'i>(
            f1: &'i i64,
            f2: &'i soroban_sdk::String,
        ) -> (&'i i64, &'i soroban_sdk::String) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn create_struct_tuple_a<'i>(f1: &'i i64, f2: &'i i64) -> (&'i i64, &'i i64) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn create_struct_tuple_b<'i>(f1: &'i u128, f2: &'i u128) -> (&'i u128, &'i u128) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn get_enum_a<'i>() -> () {
            ()
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn get_enum_b<'i>(value: &'i i64) -> (&'i i64,) {
            (value,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn get_enum_int_a<'i>() -> () {
            ()
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn get_enum_int_b<'i>() -> () {
            ()
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn check_a<'i>(input: &'i u32) -> (&'i u32,) {
            (input,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn check_b<'i>(input: &'i u32) -> (&'i u32,) {
            (input,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn check_c<'i>(input: &'i u32) -> (&'i u32,) {
            (input,)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn emit_event_a<'i>(
            f1: &'i soroban_sdk::Address,
            f2: &'i soroban_sdk::String,
        ) -> (&'i soroban_sdk::Address, &'i soroban_sdk::String) {
            (f1, f2)
        }
        #[inline(always)]
        #[allow(clippy::unused_unit)]
        pub fn emit_event_b<'i>(
            f1: &'i soroban_sdk::Address,
            f2: &'i soroban_sdk::Address,
            f3: &'i i128,
        ) -> (&'i soroban_sdk::Address, &'i soroban_sdk::Address, &'i i128) {
            (f1, f2, f3)
        }
    }
    pub struct StructA {
        pub f1: u32,
        pub f2: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StructA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "StructA", "f1", &self.f1, "f2", &&self.f2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructA {
        #[inline]
        fn clone(&self) -> StructA {
            StructA {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructA {
        #[inline]
        fn eq(&self, other: &StructA) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructA {
        #[inline]
        fn cmp(&self, other: &StructA) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructA {
        #[inline]
        fn partial_cmp(&self, other: &StructA) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_TYPE_STRUCTA: [u8; 60usize] = StructA::spec_xdr();
    impl StructA {
        pub const fn spec_xdr() -> [u8; 60usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructA\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x01"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for StructA {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <u32 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <bool as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructA {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                f1: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                f2: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let vals: [Val; 2usize] = [
                (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructA>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for StructA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScMap,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let map = val;
            if map.len() != 2usize {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            map.validate()?;
            Ok(Self {
                f1: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "f1".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let idx = map
                        .binary_search_by_key(&key, |entry| entry.key.clone())
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let rv: soroban_sdk::Val = (&map[idx].val.clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                f2: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "f2".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let idx = map
                        .binary_search_by_key(&key, |entry| entry.key.clone())
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let rv: soroban_sdk::Val = (&map[idx].val.clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for StructA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Map(Some(map)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&StructA> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructA) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "f1".try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.f1)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "f2".try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.f2)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ]),
            ))
        }
    }
    impl TryFrom<StructA> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructA) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&StructA> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructA) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
        }
    }
    impl TryFrom<StructA> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructA) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryStructA {
            f1: <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            f2: <bool as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryStructA {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ArbitraryStructA",
                    "f1",
                    &self.f1,
                    "f2",
                    &&self.f2,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryStructA {
            #[inline]
            fn clone(&self) -> ArbitraryStructA {
                ArbitraryStructA {
                    f1: ::core::clone::Clone::clone(&self.f1),
                    f2: ::core::clone::Clone::clone(&self.f2),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryStructA {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <bool as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryStructA {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryStructA {
            #[inline]
            fn eq(&self, other: &ArbitraryStructA) -> bool {
                self.f1 == other.f1 && self.f2 == other.f2
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryStructA {
            #[inline]
            fn cmp(&self, other: &ArbitraryStructA) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryStructA {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryStructA,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryStructA: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryStructA {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructA.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructA {
                            f1: arbitrary::Arbitrary::arbitrary(u)?,
                            f2: arbitrary::Arbitrary::arbitrary(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructA.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructA.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructA {
                            f1: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            f2: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructA.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::recursion_guard(depth, |depth| {
                        arbitrary::size_hint::and_all(
                            &[
                                <<u32 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                                <<bool as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for StructA {
            type Prototype = ArbitraryStructA;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryStructA> for StructA {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryStructA,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(StructA {
                    f1: soroban_sdk::IntoVal::into_val(&v.f1, env),
                    f2: soroban_sdk::IntoVal::into_val(&v.f2, env),
                })
            }
        }
    };
    pub struct StructB {
        pub f1: i64,
        pub f2: soroban_sdk::String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StructB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "StructB", "f1", &self.f1, "f2", &&self.f2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructB {
        #[inline]
        fn clone(&self) -> StructB {
            StructB {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i64>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructB {
        #[inline]
        fn eq(&self, other: &StructB) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructB {
        #[inline]
        fn cmp(&self, other: &StructB) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructB {
        #[inline]
        fn partial_cmp(&self, other: &StructB) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_TYPE_STRUCTB: [u8; 60usize] = StructB::spec_xdr();
    impl StructB {
        pub const fn spec_xdr() -> [u8; 60usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructB\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for StructB {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <soroban_sdk::String as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructB {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                f1: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                f2: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let vals: [Val; 2usize] = [
                (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructB>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for StructB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScMap,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let map = val;
            if map.len() != 2usize {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            map.validate()?;
            Ok(Self {
                f1: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "f1".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let idx = map
                        .binary_search_by_key(&key, |entry| entry.key.clone())
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let rv: soroban_sdk::Val = (&map[idx].val.clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                f2: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "f2".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let idx = map
                        .binary_search_by_key(&key, |entry| entry.key.clone())
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let rv: soroban_sdk::Val = (&map[idx].val.clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for StructB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Map(Some(map)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&StructB> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructB) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "f1".try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.f1)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "f2".try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.f2)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ]),
            ))
        }
    }
    impl TryFrom<StructB> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructB) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&StructB> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructB) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
        }
    }
    impl TryFrom<StructB> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructB) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryStructB {
            f1: <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            f2: <soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryStructB {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ArbitraryStructB",
                    "f1",
                    &self.f1,
                    "f2",
                    &&self.f2,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryStructB {
            #[inline]
            fn clone(&self) -> ArbitraryStructB {
                ArbitraryStructB {
                    f1: ::core::clone::Clone::clone(&self.f1),
                    f2: ::core::clone::Clone::clone(&self.f2),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryStructB {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryStructB {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryStructB {
            #[inline]
            fn eq(&self, other: &ArbitraryStructB) -> bool {
                self.f1 == other.f1 && self.f2 == other.f2
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryStructB {
            #[inline]
            fn cmp(&self, other: &ArbitraryStructB) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryStructB {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryStructB,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryStructB: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryStructB {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructB.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructB {
                            f1: arbitrary::Arbitrary::arbitrary(u)?,
                            f2: arbitrary::Arbitrary::arbitrary(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructB.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructB.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructB {
                            f1: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            f2: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructB.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::recursion_guard(depth, |depth| {
                        arbitrary::size_hint::and_all(
                            &[
                                <<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                                <<soroban_sdk::String as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for StructB {
            type Prototype = ArbitraryStructB;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryStructB> for StructB {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryStructB,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(StructB {
                    f1: soroban_sdk::IntoVal::into_val(&v.f1, env),
                    f2: soroban_sdk::IntoVal::into_val(&v.f2, env),
                })
            }
        }
    };
    pub struct StructC {
        pub f1: soroban_sdk::Vec<u32>,
        pub f2: soroban_sdk::Address,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StructC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "StructC", "f1", &self.f1, "f2", &&self.f2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructC {
        #[inline]
        fn clone(&self) -> StructC {
            StructC {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Vec<u32>>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructC {
        #[inline]
        fn eq(&self, other: &StructC) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructC {
        #[inline]
        fn cmp(&self, other: &StructC) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructC {
        #[inline]
        fn partial_cmp(&self, other: &StructC) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_TYPE_STRUCTC: [u8; 64usize] = StructC::spec_xdr();
    impl StructC {
        pub const fn spec_xdr() -> [u8; 64usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x07StructC\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\x03\xea\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for StructC {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <soroban_sdk::Vec<u32> as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <soroban_sdk::Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructC {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, MapObject, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
            env.map_unpack_to_slice(map, &KEYS, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                f1: vals[0]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
                f2: vals[1]
                    .try_into_val(env)
                    .map_err(|_| soroban_sdk::ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            const KEYS: [&'static str; 2usize] = ["f1", "f2"];
            let vals: [Val; 2usize] = [
                (&val.f1).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.f2).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .map_new_from_slices(&KEYS, &vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructC>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for StructC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScMap,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let map = val;
            if map.len() != 2usize {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            map.validate()?;
            Ok(Self {
                f1: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "f1".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let idx = map
                        .binary_search_by_key(&key, |entry| entry.key.clone())
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let rv: soroban_sdk::Val = (&map[idx].val.clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                f2: {
                    let key: soroban_sdk::xdr::ScVal = soroban_sdk::xdr::ScSymbol(
                        "f2".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                    .into();
                    let idx = map
                        .binary_search_by_key(&key, |entry| entry.key.clone())
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let rv: soroban_sdk::Val = (&map[idx].val.clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for StructC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Map(Some(map)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&StructC> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructC) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            soroban_sdk::xdr::ScMap::sorted_from(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "f1".try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.f1)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                    soroban_sdk::xdr::ScMapEntry {
                        key: soroban_sdk::xdr::ScSymbol(
                            "f2".try_into()
                                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        )
                        .into(),
                        val: (&val.f2)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    },
                ]),
            ))
        }
    }
    impl TryFrom<StructC> for soroban_sdk::xdr::ScMap {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructC) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&StructC> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructC) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Map(Some(val.try_into()?)))
        }
    }
    impl TryFrom<StructC> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructC) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryStructC {
            f1: <soroban_sdk::Vec<
                u32,
            > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            f2: <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryStructC {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ArbitraryStructC",
                    "f1",
                    &self.f1,
                    "f2",
                    &&self.f2,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryStructC {
            #[inline]
            fn clone(&self) -> ArbitraryStructC {
                ArbitraryStructC {
                    f1: ::core::clone::Clone::clone(&self.f1),
                    f2: ::core::clone::Clone::clone(&self.f2),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryStructC {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <soroban_sdk::Vec<
                        u32,
                    > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryStructC {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryStructC {
            #[inline]
            fn eq(&self, other: &ArbitraryStructC) -> bool {
                self.f1 == other.f1 && self.f2 == other.f2
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryStructC {
            #[inline]
            fn cmp(&self, other: &ArbitraryStructC) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryStructC {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryStructC,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryStructC: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryStructC {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructC.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructC {
                            f1: arbitrary::Arbitrary::arbitrary(u)?,
                            f2: arbitrary::Arbitrary::arbitrary(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructC.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructC.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructC {
                            f1: arbitrary::Arbitrary::arbitrary(&mut u)?,
                            f2: arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        })
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructC.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::recursion_guard(depth, |depth| {
                        arbitrary::size_hint::and_all(
                            &[
                                <<soroban_sdk::Vec<
                                    u32,
                                > as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                                <<soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for StructC {
            type Prototype = ArbitraryStructC;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryStructC> for StructC {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryStructC,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(StructC {
                    f1: soroban_sdk::IntoVal::into_val(&v.f1, env),
                    f2: soroban_sdk::IntoVal::into_val(&v.f2, env),
                })
            }
        }
    };
    pub struct StructTupleA(pub i64, pub i64);
    #[automatically_derived]
    impl ::core::fmt::Debug for StructTupleA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleA", &self.0, &&self.1)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructTupleA {
        #[inline]
        fn clone(&self) -> StructTupleA {
            StructTupleA(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructTupleA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i64>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructTupleA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructTupleA {
        #[inline]
        fn eq(&self, other: &StructTupleA) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructTupleA {
        #[inline]
        fn cmp(&self, other: &StructTupleA) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructTupleA {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StructTupleA,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_TYPE_STRUCTTUPLEA: [u8; 64usize] = StructTupleA::spec_xdr();
    impl StructTupleA {
        pub const fn spec_xdr() -> [u8; 64usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleA\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\x07"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for StructTupleA {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleA {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
            let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            env.vec_unpack_to_slice(vec, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
                1: vals[1].try_into_val(env).map_err(|_| ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructTupleA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            let vals: [Val; 2usize] = [
                (&val.0).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.1).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .vec_new_from_slice(&vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructTupleA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleA>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for StructTupleA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVec,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let vec = val;
            if vec.len() != 2usize {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            Ok(Self {
                0: {
                    let rv: soroban_sdk::Val = (&vec[0].clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                1: {
                    let rv: soroban_sdk::Val = (&vec[1].clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for StructTupleA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&StructTupleA> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructTupleA) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            Ok(soroban_sdk::xdr::ScVec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        (&val.0)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        (&val.1)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    ]),
                )
                .try_into()?,
            ))
        }
    }
    impl TryFrom<StructTupleA> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructTupleA) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&StructTupleA> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructTupleA) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
        }
    }
    impl TryFrom<StructTupleA> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructTupleA) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryStructTupleA(
            <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        );
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryStructTupleA {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "ArbitraryStructTupleA",
                    &self.0,
                    &&self.1,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryStructTupleA {
            #[inline]
            fn clone(&self) -> ArbitraryStructTupleA {
                ArbitraryStructTupleA(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryStructTupleA {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryStructTupleA {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryStructTupleA {
            #[inline]
            fn eq(&self, other: &ArbitraryStructTupleA) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryStructTupleA {
            #[inline]
            fn cmp(&self, other: &ArbitraryStructTupleA) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryStructTupleA {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryStructTupleA,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryStructTupleA: ::std::thread::LocalKey<
                std::cell::Cell<u32>,
            > = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryStructTupleA {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleA.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructTupleA(
                            arbitrary::Arbitrary::arbitrary(u)?,
                            arbitrary::Arbitrary::arbitrary(u)?,
                        ))
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleA.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleA.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructTupleA(
                            arbitrary::Arbitrary::arbitrary(&mut u)?,
                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        ))
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleA.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::recursion_guard(depth, |depth| {
                        arbitrary::size_hint::and_all(
                            &[
                                <<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                                <<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for StructTupleA {
            type Prototype = ArbitraryStructTupleA;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryStructTupleA> for StructTupleA {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryStructTupleA,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(StructTupleA(
                    soroban_sdk::IntoVal::into_val(&v.0, env),
                    soroban_sdk::IntoVal::into_val(&v.1, env),
                ))
            }
        }
    };
    pub struct StructTupleB(pub u128, pub u128);
    #[automatically_derived]
    impl ::core::fmt::Debug for StructTupleB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleB", &self.0, &&self.1)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructTupleB {
        #[inline]
        fn clone(&self) -> StructTupleB {
            StructTupleB(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructTupleB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u128>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructTupleB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructTupleB {
        #[inline]
        fn eq(&self, other: &StructTupleB) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructTupleB {
        #[inline]
        fn cmp(&self, other: &StructTupleB) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructTupleB {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StructTupleB,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_TYPE_STRUCTTUPLEB: [u8; 64usize] = StructTupleB::spec_xdr();
    impl StructTupleB {
        pub const fn spec_xdr() -> [u8; 64usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleB\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\n\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\n"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for StructTupleB {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <u128 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <u128 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleB {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
            let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            env.vec_unpack_to_slice(vec, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
                1: vals[1].try_into_val(env).map_err(|_| ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructTupleB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            let vals: [Val; 2usize] = [
                (&val.0).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.1).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .vec_new_from_slice(&vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructTupleB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleB>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for StructTupleB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVec,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let vec = val;
            if vec.len() != 2usize {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            Ok(Self {
                0: {
                    let rv: soroban_sdk::Val = (&vec[0].clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                1: {
                    let rv: soroban_sdk::Val = (&vec[1].clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for StructTupleB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&StructTupleB> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructTupleB) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            Ok(soroban_sdk::xdr::ScVec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        (&val.0)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        (&val.1)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    ]),
                )
                .try_into()?,
            ))
        }
    }
    impl TryFrom<StructTupleB> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructTupleB) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&StructTupleB> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructTupleB) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
        }
    }
    impl TryFrom<StructTupleB> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructTupleB) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryStructTupleB(
            <u128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            <u128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        );
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryStructTupleB {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "ArbitraryStructTupleB",
                    &self.0,
                    &&self.1,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryStructTupleB {
            #[inline]
            fn clone(&self) -> ArbitraryStructTupleB {
                ArbitraryStructTupleB(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryStructTupleB {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <u128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <u128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryStructTupleB {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryStructTupleB {
            #[inline]
            fn eq(&self, other: &ArbitraryStructTupleB) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryStructTupleB {
            #[inline]
            fn cmp(&self, other: &ArbitraryStructTupleB) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryStructTupleB {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryStructTupleB,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryStructTupleB: ::std::thread::LocalKey<
                std::cell::Cell<u32>,
            > = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryStructTupleB {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleB.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructTupleB(
                            arbitrary::Arbitrary::arbitrary(u)?,
                            arbitrary::Arbitrary::arbitrary(u)?,
                        ))
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleB.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleB.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructTupleB(
                            arbitrary::Arbitrary::arbitrary(&mut u)?,
                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        ))
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleB.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::recursion_guard(depth, |depth| {
                        arbitrary::size_hint::and_all(
                            &[
                                <<u128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                                <<u128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for StructTupleB {
            type Prototype = ArbitraryStructTupleB;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryStructTupleB> for StructTupleB {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryStructTupleB,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(StructTupleB(
                    soroban_sdk::IntoVal::into_val(&v.0, env),
                    soroban_sdk::IntoVal::into_val(&v.1, env),
                ))
            }
        }
    };
    pub struct StructTupleC(pub soroban_sdk::Address, pub i128);
    #[automatically_derived]
    impl ::core::fmt::Debug for StructTupleC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(f, "StructTupleC", &self.0, &&self.1)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StructTupleC {
        #[inline]
        fn clone(&self) -> StructTupleC {
            StructTupleC(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StructTupleC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<i128>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StructTupleC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StructTupleC {
        #[inline]
        fn eq(&self, other: &StructTupleC) -> bool {
            self.0 == other.0 && self.1 == other.1
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StructTupleC {
        #[inline]
        fn cmp(&self, other: &StructTupleC) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StructTupleC {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StructTupleC,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_TYPE_STRUCTTUPLEC: [u8; 64usize] = StructTupleC::spec_xdr();
    impl StructTupleC {
        pub const fn spec_xdr() -> [u8; 64usize] {
            *b"\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x0cStructTupleC\0\0\0\x02\0\0\0\0\0\0\0\x010\0\0\0\0\0\0\x13\0\0\0\0\0\0\0\x011\0\0\0\0\0\0\x0b"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for StructTupleC {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <soroban_sdk::Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <i128 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for StructTupleC {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val, VecObject};
            let vec: VecObject = (*val).try_into().map_err(|_| ConversionError)?;
            let mut vals: [Val; 2usize] = [Val::VOID.to_val(); 2usize];
            env.vec_unpack_to_slice(vec, &mut vals)
                .map_err(|_| ConversionError)?;
            Ok(Self {
                0: vals[0].try_into_val(env).map_err(|_| ConversionError)?,
                1: vals[1].try_into_val(env).map_err(|_| ConversionError)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &StructTupleC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{ConversionError, EnvBase, TryIntoVal, Val};
            let vals: [Val; 2usize] = [
                (&val.0).try_into_val(env).map_err(|_| ConversionError)?,
                (&val.1).try_into_val(env).map_err(|_| ConversionError)?,
            ];
            Ok(env
                .vec_new_from_slice(&vals)
                .map_err(|_| ConversionError)?
                .into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &StructTupleC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&StructTupleC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, StructTupleC>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for StructTupleC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVec,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let vec = val;
            if vec.len() != 2usize {
                return Err(soroban_sdk::xdr::Error::Invalid);
            }
            Ok(Self {
                0: {
                    let rv: soroban_sdk::Val = (&vec[0].clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
                1: {
                    let rv: soroban_sdk::Val = (&vec[1].clone())
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                },
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for StructTupleC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&StructTupleC> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructTupleC) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            use soroban_sdk::TryFromVal;
            Ok(soroban_sdk::xdr::ScVec(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        (&val.0)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        (&val.1)
                            .try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    ]),
                )
                .try_into()?,
            ))
        }
    }
    impl TryFrom<StructTupleC> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructTupleC) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&StructTupleC> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &StructTupleC) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
        }
    }
    impl TryFrom<StructTupleC> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: StructTupleC) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub struct ArbitraryStructTupleC(
            <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            <i128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
        );
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryStructTupleC {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "ArbitraryStructTupleC",
                    &self.0,
                    &&self.1,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryStructTupleC {
            #[inline]
            fn clone(&self) -> ArbitraryStructTupleC {
                ArbitraryStructTupleC(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryStructTupleC {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <i128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryStructTupleC {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryStructTupleC {
            #[inline]
            fn eq(&self, other: &ArbitraryStructTupleC) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryStructTupleC {
            #[inline]
            fn cmp(&self, other: &ArbitraryStructTupleC) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.0, &other.0) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.1, &other.1),
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryStructTupleC {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryStructTupleC,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(&self.1, &other.1)
                    }
                    cmp => cmp,
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryStructTupleC: ::std::thread::LocalKey<
                std::cell::Cell<u32>,
            > = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryStructTupleC {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleC.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructTupleC(
                            arbitrary::Arbitrary::arbitrary(u)?,
                            arbitrary::Arbitrary::arbitrary(u)?,
                        ))
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleC.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleC.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(ArbitraryStructTupleC(
                            arbitrary::Arbitrary::arbitrary(&mut u)?,
                            arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                        ))
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryStructTupleC.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::recursion_guard(depth, |depth| {
                        arbitrary::size_hint::and_all(
                            &[
                                <<soroban_sdk::Address as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                                <<i128 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                    depth,
                                ),
                            ],
                        )
                    })
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for StructTupleC {
            type Prototype = ArbitraryStructTupleC;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryStructTupleC> for StructTupleC {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryStructTupleC,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(StructTupleC(
                    soroban_sdk::IntoVal::into_val(&v.0, env),
                    soroban_sdk::IntoVal::into_val(&v.1, env),
                ))
            }
        }
    };
    pub enum EnumA {
        V1,
        V2,
        V3,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EnumA::V1 => "V1",
                    EnumA::V2 => "V2",
                    EnumA::V3 => "V3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EnumA {
        #[inline]
        fn clone(&self) -> EnumA {
            match self {
                EnumA::V1 => EnumA::V1,
                EnumA::V2 => EnumA::V2,
                EnumA::V3 => EnumA::V3,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumA {
        #[inline]
        fn eq(&self, other: &EnumA) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumA {
        #[inline]
        fn cmp(&self, other: &EnumA) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumA {
        #[inline]
        fn partial_cmp(&self, other: &EnumA) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    pub static __SPEC_XDR_TYPE_ENUMA: [u8; 76usize] = EnumA::spec_xdr();
    impl EnumA {
        pub const fn spec_xdr() -> [u8; 76usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumA\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\0\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\0\0\0\0\0\0\0\0\x02V3\0\0"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EnumA {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {}
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumA {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
            const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
            let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
            let mut iter = vec.try_iter();
            let discriminant: soroban_sdk::Symbol = iter
                .next()
                .ok_or(soroban_sdk::ConversionError)??
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?;
            Ok(
                match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                    as usize
                {
                    0 => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V1
                    }
                    1 => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V2
                    }
                    2 => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V3
                    }
                    _ => Err(soroban_sdk::ConversionError {})?,
                },
            )
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{TryFromVal, TryIntoVal};
            match val {
                EnumA::V1 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumA::V2 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumA::V3 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumA>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for EnumA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVec,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let vec = val;
            let mut iter = vec.iter();
            let discriminant: soroban_sdk::xdr::ScSymbol = iter
                .next()
                .ok_or(soroban_sdk::xdr::Error::Invalid)?
                .clone()
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
            let discriminant_name: &str = &discriminant.to_utf8_string()?;
            Ok(match discriminant_name {
                "V1" => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    Self::V1
                }
                "V2" => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    Self::V2
                }
                "V3" => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    Self::V3
                }
                _ => Err(soroban_sdk::xdr::Error::Invalid)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for EnumA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&EnumA> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &EnumA) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            Ok(match val {
                EnumA::V1 => {
                    let symbol = soroban_sdk::xdr::ScSymbol(
                        "V1".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    );
                    let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                    (val,)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                }
                EnumA::V2 => {
                    let symbol = soroban_sdk::xdr::ScSymbol(
                        "V2".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    );
                    let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                    (val,)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                }
                EnumA::V3 => {
                    let symbol = soroban_sdk::xdr::ScSymbol(
                        "V3".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    );
                    let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                    (val,)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                }
            })
        }
    }
    impl TryFrom<EnumA> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: EnumA) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&EnumA> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &EnumA) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
        }
    }
    impl TryFrom<EnumA> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: EnumA) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub enum ArbitraryEnumA {
            V1,
            V2,
            V3,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryEnumA {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ArbitraryEnumA::V1 => "V1",
                        ArbitraryEnumA::V2 => "V2",
                        ArbitraryEnumA::V3 => "V3",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryEnumA {
            #[inline]
            fn clone(&self) -> ArbitraryEnumA {
                match self {
                    ArbitraryEnumA::V1 => ArbitraryEnumA::V1,
                    ArbitraryEnumA::V2 => ArbitraryEnumA::V2,
                    ArbitraryEnumA::V3 => ArbitraryEnumA::V3,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryEnumA {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryEnumA {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryEnumA {
            #[inline]
            fn eq(&self, other: &ArbitraryEnumA) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryEnumA {
            #[inline]
            fn cmp(&self, other: &ArbitraryEnumA) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryEnumA {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryEnumA,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryEnumA: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryEnumA {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumA.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumA::V1,
                                1u64 => ArbitraryEnumA::V2,
                                2u64 => ArbitraryEnumA::V3,
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumA.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumA.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumA::V1,
                                1u64 => ArbitraryEnumA::V2,
                                2u64 => ArbitraryEnumA::V3,
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumA.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::and(
                        <u32 as arbitrary::Arbitrary>::size_hint(depth),
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::or_all(&[
                                arbitrary::size_hint::and_all(&[]),
                                arbitrary::size_hint::and_all(&[]),
                                arbitrary::size_hint::and_all(&[]),
                            ])
                        }),
                    )
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for EnumA {
            type Prototype = ArbitraryEnumA;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryEnumA> for EnumA {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryEnumA,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(match v {
                    ArbitraryEnumA::V1 => EnumA::V1,
                    ArbitraryEnumA::V2 => EnumA::V2,
                    ArbitraryEnumA::V3 => EnumA::V3,
                })
            }
        }
    };
    pub enum EnumB {
        V1,
        V2(i64),
        V3(i64, i64),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                EnumB::V1 => ::core::fmt::Formatter::write_str(f, "V1"),
                EnumB::V2(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
                }
                EnumB::V3(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(f, "V3", __self_0, &__self_1)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EnumB {
        #[inline]
        fn clone(&self) -> EnumB {
            match self {
                EnumB::V1 => EnumB::V1,
                EnumB::V2(__self_0) => EnumB::V2(::core::clone::Clone::clone(__self_0)),
                EnumB::V3(__self_0, __self_1) => EnumB::V3(
                    ::core::clone::Clone::clone(__self_0),
                    ::core::clone::Clone::clone(__self_1),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<i64>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumB {
        #[inline]
        fn eq(&self, other: &EnumB) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (EnumB::V2(__self_0), EnumB::V2(__arg1_0)) => __self_0 == __arg1_0,
                    (EnumB::V3(__self_0, __self_1), EnumB::V3(__arg1_0, __arg1_1)) => {
                        __self_0 == __arg1_0 && __self_1 == __arg1_1
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumB {
        #[inline]
        fn cmp(&self, other: &EnumB) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (EnumB::V2(__self_0), EnumB::V2(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (EnumB::V3(__self_0, __self_1), EnumB::V3(__arg1_0, __arg1_1)) => {
                        match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                            }
                            cmp => cmp,
                        }
                    }
                    _ => ::core::cmp::Ordering::Equal,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumB {
        #[inline]
        fn partial_cmp(&self, other: &EnumB) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (EnumB::V2(__self_0), EnumB::V2(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (EnumB::V3(__self_0, __self_1), EnumB::V3(__arg1_0, __arg1_1)) => {
                    match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                        }
                        cmp => cmp,
                    }
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    pub static __SPEC_XDR_TYPE_ENUMB: [u8; 96usize] = EnumB::spec_xdr();
    impl EnumB {
        pub const fn spec_xdr() -> [u8; 96usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumB\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x01\0\0\0\x07\0\0\0\x01\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x02\0\0\0\x07\0\0\0\x07"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EnumB {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumB {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
            const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
            let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
            let mut iter = vec.try_iter();
            let discriminant: soroban_sdk::Symbol = iter
                .next()
                .ok_or(soroban_sdk::ConversionError)??
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?;
            Ok(
                match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                    as usize
                {
                    0 => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V1
                    }
                    1 => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V2(
                            iter.next()
                                .ok_or(soroban_sdk::ConversionError)??
                                .try_into_val(env)?,
                        )
                    }
                    2 => {
                        if iter.len() > 2usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V3(
                            iter.next()
                                .ok_or(soroban_sdk::ConversionError)??
                                .try_into_val(env)?,
                            iter.next()
                                .ok_or(soroban_sdk::ConversionError)??
                                .try_into_val(env)?,
                        )
                    }
                    _ => Err(soroban_sdk::ConversionError {})?,
                },
            )
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{TryFromVal, TryIntoVal};
            match val {
                EnumB::V1 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumB::V2(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumB::V3(ref value0, ref value1) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),
                        value0.try_into_val(env)?,
                        value1.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumB>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for EnumB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVec,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let vec = val;
            let mut iter = vec.iter();
            let discriminant: soroban_sdk::xdr::ScSymbol = iter
                .next()
                .ok_or(soroban_sdk::xdr::Error::Invalid)?
                .clone()
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
            let discriminant_name: &str = &discriminant.to_utf8_string()?;
            Ok(match discriminant_name {
                "V1" => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    Self::V1
                }
                "V2" => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    let rv0: soroban_sdk::Val = iter
                        .next()
                        .ok_or(soroban_sdk::xdr::Error::Invalid)?
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    Self::V2(
                        rv0.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                }
                "V3" => {
                    if iter.len() > 2usize {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    let rv0: soroban_sdk::Val = iter
                        .next()
                        .ok_or(soroban_sdk::xdr::Error::Invalid)?
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let rv1: soroban_sdk::Val = iter
                        .next()
                        .ok_or(soroban_sdk::xdr::Error::Invalid)?
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    Self::V3(
                        rv0.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                        rv1.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                }
                _ => Err(soroban_sdk::xdr::Error::Invalid)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for EnumB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&EnumB> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &EnumB) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            Ok(match val {
                EnumB::V1 => {
                    let symbol = soroban_sdk::xdr::ScSymbol(
                        "V1".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    );
                    let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                    (val,)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                }
                EnumB::V2(value0) => (
                    soroban_sdk::xdr::ScSymbol(
                        "V2".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    ),
                    value0,
                )
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                EnumB::V3(value0, value1) => (
                    soroban_sdk::xdr::ScSymbol(
                        "V3".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    ),
                    value0,
                    value1,
                )
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            })
        }
    }
    impl TryFrom<EnumB> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: EnumB) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&EnumB> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &EnumB) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
        }
    }
    impl TryFrom<EnumB> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: EnumB) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub enum ArbitraryEnumB {
            V1,
            V2(<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
            V3(
                <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
            ),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryEnumB {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ArbitraryEnumB::V1 => ::core::fmt::Formatter::write_str(f, "V1"),
                    ArbitraryEnumB::V2(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
                    }
                    ArbitraryEnumB::V3(__self_0, __self_1) => {
                        ::core::fmt::Formatter::debug_tuple_field2_finish(
                            f, "V3", __self_0, &__self_1,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryEnumB {
            #[inline]
            fn clone(&self) -> ArbitraryEnumB {
                match self {
                    ArbitraryEnumB::V1 => ArbitraryEnumB::V1,
                    ArbitraryEnumB::V2(__self_0) => {
                        ArbitraryEnumB::V2(::core::clone::Clone::clone(__self_0))
                    }
                    ArbitraryEnumB::V3(__self_0, __self_1) => ArbitraryEnumB::V3(
                        ::core::clone::Clone::clone(__self_0),
                        ::core::clone::Clone::clone(__self_1),
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryEnumB {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryEnumB {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryEnumB {
            #[inline]
            fn eq(&self, other: &ArbitraryEnumB) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (ArbitraryEnumB::V2(__self_0), ArbitraryEnumB::V2(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            ArbitraryEnumB::V3(__self_0, __self_1),
                            ArbitraryEnumB::V3(__arg1_0, __arg1_1),
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryEnumB {
            #[inline]
            fn cmp(&self, other: &ArbitraryEnumB) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (ArbitraryEnumB::V2(__self_0), ArbitraryEnumB::V2(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (
                            ArbitraryEnumB::V3(__self_0, __self_1),
                            ArbitraryEnumB::V3(__arg1_0, __arg1_1),
                        ) => match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                            ::core::cmp::Ordering::Equal => {
                                ::core::cmp::Ord::cmp(__self_1, __arg1_1)
                            }
                            cmp => cmp,
                        },
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryEnumB {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryEnumB,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (ArbitraryEnumB::V2(__self_0), ArbitraryEnumB::V2(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    (
                        ArbitraryEnumB::V3(__self_0, __self_1),
                        ArbitraryEnumB::V3(__arg1_0, __arg1_1),
                    ) => match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1)
                        }
                        cmp => cmp,
                    },
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryEnumB: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryEnumB {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumB.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumB::V1,
                                1u64 => ArbitraryEnumB::V2(arbitrary::Arbitrary::arbitrary(u)?),
                                2u64 => ArbitraryEnumB::V3(
                                    arbitrary::Arbitrary::arbitrary(u)?,
                                    arbitrary::Arbitrary::arbitrary(u)?,
                                ),
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumB.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumB.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumB::V1,
                                1u64 => ArbitraryEnumB::V2(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                2u64 => ArbitraryEnumB::V3(
                                    arbitrary::Arbitrary::arbitrary(&mut u)?,
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumB.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::and(
                        <u32 as arbitrary::Arbitrary>::size_hint(depth),
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::or_all(
                                    &[
                                        arbitrary::size_hint::and_all(&[]),
                                        arbitrary::size_hint::and_all(
                                            &[
                                                <<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                    depth,
                                                ),
                                            ],
                                        ),
                                        arbitrary::size_hint::and_all(
                                            &[
                                                <<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                    depth,
                                                ),
                                                <<i64 as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                    depth,
                                                ),
                                            ],
                                        ),
                                    ],
                                )
                        }),
                    )
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for EnumB {
            type Prototype = ArbitraryEnumB;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryEnumB> for EnumB {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryEnumB,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(match v {
                    ArbitraryEnumB::V1 => EnumB::V1,
                    ArbitraryEnumB::V2(field_0) => {
                        EnumB::V2(soroban_sdk::IntoVal::into_val(field_0, env))
                    }
                    ArbitraryEnumB::V3(field_0, field_1) => EnumB::V3(
                        soroban_sdk::IntoVal::into_val(field_0, env),
                        soroban_sdk::IntoVal::into_val(field_1, env),
                    ),
                })
            }
        }
    };
    pub enum EnumC {
        V1,
        V2(StructA),
        V3(StructTupleA),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                EnumC::V1 => ::core::fmt::Formatter::write_str(f, "V1"),
                EnumC::V2(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
                }
                EnumC::V3(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V3", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EnumC {
        #[inline]
        fn clone(&self) -> EnumC {
            match self {
                EnumC::V1 => EnumC::V1,
                EnumC::V2(__self_0) => EnumC::V2(::core::clone::Clone::clone(__self_0)),
                EnumC::V3(__self_0) => EnumC::V3(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<StructA>;
            let _: ::core::cmp::AssertParamIsEq<StructTupleA>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumC {
        #[inline]
        fn eq(&self, other: &EnumC) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (EnumC::V2(__self_0), EnumC::V2(__arg1_0)) => __self_0 == __arg1_0,
                    (EnumC::V3(__self_0), EnumC::V3(__arg1_0)) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumC {
        #[inline]
        fn cmp(&self, other: &EnumC) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (EnumC::V2(__self_0), EnumC::V2(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    (EnumC::V3(__self_0), EnumC::V3(__arg1_0)) => {
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::Ordering::Equal,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumC {
        #[inline]
        fn partial_cmp(&self, other: &EnumC) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (EnumC::V2(__self_0), EnumC::V2(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (EnumC::V3(__self_0), EnumC::V3(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
            }
        }
    }
    pub static __SPEC_XDR_TYPE_ENUMC: [u8; 120usize] = EnumC::spec_xdr();
    impl EnumC {
        pub const fn spec_xdr() -> [u8; 120usize] {
            *b"\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\x05EnumC\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructA\0\0\0\0\x01\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleA"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EnumC {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <StructA as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <StructTupleA as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumC {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{EnvBase, TryFromVal, TryIntoVal};
            const CASES: &'static [&'static str] = &["V1", "V2", "V3"];
            let vec: soroban_sdk::Vec<soroban_sdk::Val> = val.try_into_val(env)?;
            let mut iter = vec.try_iter();
            let discriminant: soroban_sdk::Symbol = iter
                .next()
                .ok_or(soroban_sdk::ConversionError)??
                .try_into_val(env)
                .map_err(|_| soroban_sdk::ConversionError)?;
            Ok(
                match u32::from(env.symbol_index_in_strs(discriminant.to_symbol_val(), CASES)?)
                    as usize
                {
                    0 => {
                        if iter.len() > 0 {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V1
                    }
                    1 => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V2(
                            iter.next()
                                .ok_or(soroban_sdk::ConversionError)??
                                .try_into_val(env)?,
                        )
                    }
                    2 => {
                        if iter.len() > 1usize {
                            return Err(soroban_sdk::ConversionError);
                        }
                        Self::V3(
                            iter.next()
                                .ok_or(soroban_sdk::ConversionError)??
                                .try_into_val(env)?,
                        )
                    }
                    _ => Err(soroban_sdk::ConversionError {})?,
                },
            )
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::{TryFromVal, TryIntoVal};
            match val {
                EnumC::V1 => {
                    let tup: (soroban_sdk::Val,) =
                        (soroban_sdk::Symbol::try_from_val(env, &"V1")?.to_val(),);
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumC::V2(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"V2")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
                EnumC::V3(ref value0) => {
                    let tup: (soroban_sdk::Val, soroban_sdk::Val) = (
                        soroban_sdk::Symbol::try_from_val(env, &"V3")?.to_val(),
                        value0.try_into_val(env)?,
                    );
                    tup.try_into_val(env).map_err(Into::into)
                }
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumC>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVec> for EnumC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVec,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            use soroban_sdk::xdr::Validate;
            use soroban_sdk::TryIntoVal;
            let vec = val;
            let mut iter = vec.iter();
            let discriminant: soroban_sdk::xdr::ScSymbol = iter
                .next()
                .ok_or(soroban_sdk::xdr::Error::Invalid)?
                .clone()
                .try_into()
                .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
            let discriminant_name: &str = &discriminant.to_utf8_string()?;
            Ok(match discriminant_name {
                "V1" => {
                    if iter.len() > 0 {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    Self::V1
                }
                "V2" => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    let rv0: soroban_sdk::Val = iter
                        .next()
                        .ok_or(soroban_sdk::xdr::Error::Invalid)?
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    Self::V2(
                        rv0.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                }
                "V3" => {
                    if iter.len() > 1usize {
                        return Err(soroban_sdk::xdr::Error::Invalid);
                    }
                    let rv0: soroban_sdk::Val = iter
                        .next()
                        .ok_or(soroban_sdk::xdr::Error::Invalid)?
                        .try_into_val(env)
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    Self::V3(
                        rv0.try_into_val(env)
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    )
                }
                _ => Err(soroban_sdk::xdr::Error::Invalid)?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for EnumC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::Vec(Some(vec)) = val {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, vec)
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryFrom<&EnumC> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &EnumC) -> Result<Self, soroban_sdk::xdr::Error> {
            extern crate alloc;
            Ok(match val {
                EnumC::V1 => {
                    let symbol = soroban_sdk::xdr::ScSymbol(
                        "V1".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    );
                    let val = soroban_sdk::xdr::ScVal::Symbol(symbol);
                    (val,)
                        .try_into()
                        .map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                }
                EnumC::V2(value0) => (
                    soroban_sdk::xdr::ScSymbol(
                        "V2".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    ),
                    value0,
                )
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                EnumC::V3(value0) => (
                    soroban_sdk::xdr::ScSymbol(
                        "V3".try_into()
                            .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    ),
                    value0,
                )
                    .try_into()
                    .map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
            })
        }
    }
    impl TryFrom<EnumC> for soroban_sdk::xdr::ScVec {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: EnumC) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    impl TryFrom<&EnumC> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: &EnumC) -> Result<Self, soroban_sdk::xdr::Error> {
            Ok(soroban_sdk::xdr::ScVal::Vec(Some(val.try_into()?)))
        }
    }
    impl TryFrom<EnumC> for soroban_sdk::xdr::ScVal {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from(val: EnumC) -> Result<Self, soroban_sdk::xdr::Error> {
            (&val).try_into()
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub enum ArbitraryEnumC {
            V1,
            V2(<StructA as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
            V3(<StructTupleA as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryEnumC {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ArbitraryEnumC::V1 => ::core::fmt::Formatter::write_str(f, "V1"),
                    ArbitraryEnumC::V2(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V2", &__self_0)
                    }
                    ArbitraryEnumC::V3(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "V3", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryEnumC {
            #[inline]
            fn clone(&self) -> ArbitraryEnumC {
                match self {
                    ArbitraryEnumC::V1 => ArbitraryEnumC::V1,
                    ArbitraryEnumC::V2(__self_0) => {
                        ArbitraryEnumC::V2(::core::clone::Clone::clone(__self_0))
                    }
                    ArbitraryEnumC::V3(__self_0) => {
                        ArbitraryEnumC::V3(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryEnumC {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    <StructA as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    <StructTupleA as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype,
                >;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryEnumC {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryEnumC {
            #[inline]
            fn eq(&self, other: &ArbitraryEnumC) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (ArbitraryEnumC::V2(__self_0), ArbitraryEnumC::V2(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (ArbitraryEnumC::V3(__self_0), ArbitraryEnumC::V3(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryEnumC {
            #[inline]
            fn cmp(&self, other: &ArbitraryEnumC) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                    ::core::cmp::Ordering::Equal => match (self, other) {
                        (ArbitraryEnumC::V2(__self_0), ArbitraryEnumC::V2(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        (ArbitraryEnumC::V3(__self_0), ArbitraryEnumC::V3(__arg1_0)) => {
                            ::core::cmp::Ord::cmp(__self_0, __arg1_0)
                        }
                        _ => ::core::cmp::Ordering::Equal,
                    },
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryEnumC {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryEnumC,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                match (self, other) {
                    (ArbitraryEnumC::V2(__self_0), ArbitraryEnumC::V2(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    (ArbitraryEnumC::V3(__self_0), ArbitraryEnumC::V3(__arg1_0)) => {
                        ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    }
                    _ => ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr),
                }
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryEnumC: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryEnumC {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumC.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumC::V1,
                                1u64 => ArbitraryEnumC::V2(arbitrary::Arbitrary::arbitrary(u)?),
                                2u64 => ArbitraryEnumC::V3(arbitrary::Arbitrary::arbitrary(u)?),
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumC.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumC.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumC::V1,
                                1u64 => ArbitraryEnumC::V2(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                2u64 => ArbitraryEnumC::V3(
                                    arbitrary::Arbitrary::arbitrary_take_rest(u)?,
                                ),
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumC.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::and(
                        <u32 as arbitrary::Arbitrary>::size_hint(depth),
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::or_all(
                                    &[
                                        arbitrary::size_hint::and_all(&[]),
                                        arbitrary::size_hint::and_all(
                                            &[
                                                <<StructA as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                    depth,
                                                ),
                                            ],
                                        ),
                                        arbitrary::size_hint::and_all(
                                            &[
                                                <<StructTupleA as soroban_sdk::testutils::arbitrary::SorobanArbitrary>::Prototype as arbitrary::Arbitrary>::size_hint(
                                                    depth,
                                                ),
                                            ],
                                        ),
                                    ],
                                )
                        }),
                    )
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for EnumC {
            type Prototype = ArbitraryEnumC;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryEnumC> for EnumC {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryEnumC,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(match v {
                    ArbitraryEnumC::V1 => EnumC::V1,
                    ArbitraryEnumC::V2(field_0) => {
                        EnumC::V2(soroban_sdk::IntoVal::into_val(field_0, env))
                    }
                    ArbitraryEnumC::V3(field_0) => {
                        EnumC::V3(soroban_sdk::IntoVal::into_val(field_0, env))
                    }
                })
            }
        }
    };
    pub enum EnumIntA {
        V1 = 1,
        V2 = 2,
        V3 = 3,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumIntA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EnumIntA::V1 => "V1",
                    EnumIntA::V2 => "V2",
                    EnumIntA::V3 => "V3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EnumIntA {}
    #[automatically_derived]
    impl ::core::clone::Clone for EnumIntA {
        #[inline]
        fn clone(&self) -> EnumIntA {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumIntA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumIntA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumIntA {
        #[inline]
        fn eq(&self, other: &EnumIntA) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumIntA {
        #[inline]
        fn cmp(&self, other: &EnumIntA) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumIntA {
        #[inline]
        fn partial_cmp(&self, other: &EnumIntA) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    pub static __SPEC_XDR_TYPE_ENUMINTA: [u8; 76usize] = EnumIntA::spec_xdr();
    impl EnumIntA {
        pub const fn spec_xdr() -> [u8; 76usize] {
            *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntA\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x03"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EnumIntA {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {}
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntA {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let discriminant: u32 = val.try_into_val(env)?;
            Ok(match discriminant {
                1u32 => Self::V1,
                2u32 => Self::V2,
                3u32 => Self::V3,
                _ => Err(soroban_sdk::ConversionError {})?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumIntA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            Ok(match val {
                EnumIntA::V1 => 1u32.into(),
                EnumIntA::V2 => 2u32.into(),
                EnumIntA::V3 => 3u32.into(),
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumIntA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntA>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for EnumIntA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::U32(discriminant) = val {
                Ok(match *discriminant {
                    1u32 => Self::V1,
                    2u32 => Self::V2,
                    3u32 => Self::V3,
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryInto<soroban_sdk::xdr::ScVal> for &EnumIntA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
            Ok(match self {
                EnumIntA::V1 => 1u32.into(),
                EnumIntA::V2 => 2u32.into(),
                EnumIntA::V3 => 3u32.into(),
            })
        }
    }
    impl TryInto<soroban_sdk::xdr::ScVal> for EnumIntA {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
            Ok(match self {
                EnumIntA::V1 => 1u32.into(),
                EnumIntA::V2 => 2u32.into(),
                EnumIntA::V3 => 3u32.into(),
            })
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub enum ArbitraryEnumIntA {
            V1,
            V2,
            V3,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryEnumIntA {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ArbitraryEnumIntA::V1 => "V1",
                        ArbitraryEnumIntA::V2 => "V2",
                        ArbitraryEnumIntA::V3 => "V3",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryEnumIntA {
            #[inline]
            fn clone(&self) -> ArbitraryEnumIntA {
                match self {
                    ArbitraryEnumIntA::V1 => ArbitraryEnumIntA::V1,
                    ArbitraryEnumIntA::V2 => ArbitraryEnumIntA::V2,
                    ArbitraryEnumIntA::V3 => ArbitraryEnumIntA::V3,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryEnumIntA {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryEnumIntA {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryEnumIntA {
            #[inline]
            fn eq(&self, other: &ArbitraryEnumIntA) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryEnumIntA {
            #[inline]
            fn cmp(&self, other: &ArbitraryEnumIntA) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryEnumIntA {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryEnumIntA,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryEnumIntA: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryEnumIntA {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntA.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumIntA::V1,
                                1u64 => ArbitraryEnumIntA::V2,
                                2u64 => ArbitraryEnumIntA::V3,
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntA.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntA.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumIntA::V1,
                                1u64 => ArbitraryEnumIntA::V2,
                                2u64 => ArbitraryEnumIntA::V3,
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntA.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::and(
                        <u32 as arbitrary::Arbitrary>::size_hint(depth),
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::or_all(&[
                                arbitrary::size_hint::and_all(&[]),
                                arbitrary::size_hint::and_all(&[]),
                                arbitrary::size_hint::and_all(&[]),
                            ])
                        }),
                    )
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for EnumIntA {
            type Prototype = ArbitraryEnumIntA;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryEnumIntA> for EnumIntA {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryEnumIntA,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(match v {
                    ArbitraryEnumIntA::V1 => EnumIntA::V1,
                    ArbitraryEnumIntA::V2 => EnumIntA::V2,
                    ArbitraryEnumIntA::V3 => EnumIntA::V3,
                })
            }
        }
    };
    pub enum EnumIntB {
        V1 = 10,
        V2 = 20,
        V3 = 30,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumIntB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EnumIntB::V1 => "V1",
                    EnumIntB::V2 => "V2",
                    EnumIntB::V3 => "V3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EnumIntB {}
    #[automatically_derived]
    impl ::core::clone::Clone for EnumIntB {
        #[inline]
        fn clone(&self) -> EnumIntB {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumIntB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumIntB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumIntB {
        #[inline]
        fn eq(&self, other: &EnumIntB) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumIntB {
        #[inline]
        fn cmp(&self, other: &EnumIntB) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumIntB {
        #[inline]
        fn partial_cmp(&self, other: &EnumIntB) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    pub static __SPEC_XDR_TYPE_ENUMINTB: [u8; 76usize] = EnumIntB::spec_xdr();
    impl EnumIntB {
        pub const fn spec_xdr() -> [u8; 76usize] {
            *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntB\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\x14\0\0\0\0\0\0\0\x02V3\0\0\0\0\0\x1e"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EnumIntB {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {}
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntB {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let discriminant: u32 = val.try_into_val(env)?;
            Ok(match discriminant {
                10u32 => Self::V1,
                20u32 => Self::V2,
                30u32 => Self::V3,
                _ => Err(soroban_sdk::ConversionError {})?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumIntB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            Ok(match val {
                EnumIntB::V1 => 10u32.into(),
                EnumIntB::V2 => 20u32.into(),
                EnumIntB::V3 => 30u32.into(),
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumIntB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntB>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for EnumIntB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::U32(discriminant) = val {
                Ok(match *discriminant {
                    10u32 => Self::V1,
                    20u32 => Self::V2,
                    30u32 => Self::V3,
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryInto<soroban_sdk::xdr::ScVal> for &EnumIntB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
            Ok(match self {
                EnumIntB::V1 => 10u32.into(),
                EnumIntB::V2 => 20u32.into(),
                EnumIntB::V3 => 30u32.into(),
            })
        }
    }
    impl TryInto<soroban_sdk::xdr::ScVal> for EnumIntB {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
            Ok(match self {
                EnumIntB::V1 => 10u32.into(),
                EnumIntB::V2 => 20u32.into(),
                EnumIntB::V3 => 30u32.into(),
            })
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub enum ArbitraryEnumIntB {
            V1,
            V2,
            V3,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryEnumIntB {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ArbitraryEnumIntB::V1 => "V1",
                        ArbitraryEnumIntB::V2 => "V2",
                        ArbitraryEnumIntB::V3 => "V3",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryEnumIntB {
            #[inline]
            fn clone(&self) -> ArbitraryEnumIntB {
                match self {
                    ArbitraryEnumIntB::V1 => ArbitraryEnumIntB::V1,
                    ArbitraryEnumIntB::V2 => ArbitraryEnumIntB::V2,
                    ArbitraryEnumIntB::V3 => ArbitraryEnumIntB::V3,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryEnumIntB {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryEnumIntB {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryEnumIntB {
            #[inline]
            fn eq(&self, other: &ArbitraryEnumIntB) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryEnumIntB {
            #[inline]
            fn cmp(&self, other: &ArbitraryEnumIntB) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryEnumIntB {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryEnumIntB,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryEnumIntB: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryEnumIntB {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntB.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumIntB::V1,
                                1u64 => ArbitraryEnumIntB::V2,
                                2u64 => ArbitraryEnumIntB::V3,
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntB.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntB.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumIntB::V1,
                                1u64 => ArbitraryEnumIntB::V2,
                                2u64 => ArbitraryEnumIntB::V3,
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntB.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::and(
                        <u32 as arbitrary::Arbitrary>::size_hint(depth),
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::or_all(&[
                                arbitrary::size_hint::and_all(&[]),
                                arbitrary::size_hint::and_all(&[]),
                                arbitrary::size_hint::and_all(&[]),
                            ])
                        }),
                    )
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for EnumIntB {
            type Prototype = ArbitraryEnumIntB;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryEnumIntB> for EnumIntB {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryEnumIntB,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(match v {
                    ArbitraryEnumIntB::V1 => EnumIntB::V1,
                    ArbitraryEnumIntB::V2 => EnumIntB::V2,
                    ArbitraryEnumIntB::V3 => EnumIntB::V3,
                })
            }
        }
    };
    pub enum EnumIntC {
        V1 = 100,
        V2 = 200,
        V3 = 300,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EnumIntC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EnumIntC::V1 => "V1",
                    EnumIntC::V2 => "V2",
                    EnumIntC::V3 => "V3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EnumIntC {}
    #[automatically_derived]
    impl ::core::clone::Clone for EnumIntC {
        #[inline]
        fn clone(&self) -> EnumIntC {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EnumIntC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EnumIntC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EnumIntC {
        #[inline]
        fn eq(&self, other: &EnumIntC) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EnumIntC {
        #[inline]
        fn cmp(&self, other: &EnumIntC) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EnumIntC {
        #[inline]
        fn partial_cmp(&self, other: &EnumIntC) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    pub static __SPEC_XDR_TYPE_ENUMINTC: [u8; 76usize] = EnumIntC::spec_xdr();
    impl EnumIntC {
        pub const fn spec_xdr() -> [u8; 76usize] {
            *b"\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\x08EnumIntC\0\0\0\x03\0\0\0\0\0\0\0\x02V1\0\0\0\0\0d\0\0\0\0\0\0\0\x02V2\0\0\0\0\0\xc8\0\0\0\0\0\0\0\x02V3\0\0\0\0\x01,"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EnumIntC {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {}
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for EnumIntC {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let discriminant: u32 = val.try_into_val(env)?;
            Ok(match discriminant {
                100u32 => Self::V1,
                200u32 => Self::V2,
                300u32 => Self::V3,
                _ => Err(soroban_sdk::ConversionError {})?,
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &EnumIntC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            Ok(match val {
                EnumIntC::V1 => 100u32.into(),
                EnumIntC::V2 => 200u32.into(),
                EnumIntC::V3 => 300u32.into(),
            })
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &EnumIntC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&EnumIntC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, EnumIntC>>::try_from_val(env, *val)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for EnumIntC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::xdr::ScVal,
        ) -> Result<Self, soroban_sdk::xdr::Error> {
            if let soroban_sdk::xdr::ScVal::U32(discriminant) = val {
                Ok(match *discriminant {
                    100u32 => Self::V1,
                    200u32 => Self::V2,
                    300u32 => Self::V3,
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            } else {
                Err(soroban_sdk::xdr::Error::Invalid)
            }
        }
    }
    impl TryInto<soroban_sdk::xdr::ScVal> for &EnumIntC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
            Ok(match self {
                EnumIntC::V1 => 100u32.into(),
                EnumIntC::V2 => 200u32.into(),
                EnumIntC::V3 => 300u32.into(),
            })
        }
    }
    impl TryInto<soroban_sdk::xdr::ScVal> for EnumIntC {
        type Error = soroban_sdk::xdr::Error;
        #[inline(always)]
        fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, soroban_sdk::xdr::Error> {
            Ok(match self {
                EnumIntC::V1 => 100u32.into(),
                EnumIntC::V2 => 200u32.into(),
                EnumIntC::V3 => 300u32.into(),
            })
        }
    }
    const _: () = {
        use soroban_sdk::testutils::arbitrary::arbitrary;
        use soroban_sdk::testutils::arbitrary::std;
        pub enum ArbitraryEnumIntC {
            V1,
            V2,
            V3,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArbitraryEnumIntC {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ArbitraryEnumIntC::V1 => "V1",
                        ArbitraryEnumIntC::V2 => "V2",
                        ArbitraryEnumIntC::V3 => "V3",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ArbitraryEnumIntC {
            #[inline]
            fn clone(&self) -> ArbitraryEnumIntC {
                match self {
                    ArbitraryEnumIntC::V1 => ArbitraryEnumIntC::V1,
                    ArbitraryEnumIntC::V2 => ArbitraryEnumIntC::V2,
                    ArbitraryEnumIntC::V3 => ArbitraryEnumIntC::V3,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ArbitraryEnumIntC {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArbitraryEnumIntC {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArbitraryEnumIntC {
            #[inline]
            fn eq(&self, other: &ArbitraryEnumIntC) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ArbitraryEnumIntC {
            #[inline]
            fn cmp(&self, other: &ArbitraryEnumIntC) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArbitraryEnumIntC {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArbitraryEnumIntC,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            const RECURSIVE_COUNT_ArbitraryEnumIntC: ::std::thread::LocalKey<std::cell::Cell<u32>> = {
                #[inline]
                fn __init() -> std::cell::Cell<u32> {
                    std::cell::Cell::new(0)
                }
                unsafe {
                    ::std::thread::LocalKey::new(
                        const {
                            if ::std::mem::needs_drop::<std::cell::Cell<u32>>() {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        (),
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            } else {
                                |init| {
                                    #[thread_local]
                                    static VAL: ::std::thread::local_impl::LazyStorage<
                                        std::cell::Cell<u32>,
                                        !,
                                    > = ::std::thread::local_impl::LazyStorage::new();
                                    VAL.get_or_init(init, __init)
                                }
                            }
                        },
                    )
                }
            };
            #[automatically_derived]
            impl<'arbitrary> arbitrary::Arbitrary<'arbitrary> for ArbitraryEnumIntC {
                fn arbitrary(
                    u: &mut arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntC.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(u)?) * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumIntC::V1,
                                1u64 => ArbitraryEnumIntC::V2,
                                2u64 => ArbitraryEnumIntC::V3,
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntC.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                fn arbitrary_take_rest(
                    mut u: arbitrary::Unstructured<'arbitrary>,
                ) -> arbitrary::Result<Self> {
                    let guard_against_recursion = u.is_empty();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntC.with(|count| {
                            if count.get() > 0 {
                                return Err(arbitrary::Error::NotEnoughData);
                            }
                            count.set(count.get() + 1);
                            Ok(())
                        })?;
                    }
                    let result = (|| {
                        Ok(
                            match (u64::from(<u32 as arbitrary::Arbitrary>::arbitrary(&mut u)?)
                                * 3u64)
                                >> 32
                            {
                                0u64 => ArbitraryEnumIntC::V1,
                                1u64 => ArbitraryEnumIntC::V2,
                                2u64 => ArbitraryEnumIntC::V3,
                                _ => ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                ),
                            },
                        )
                    })();
                    if guard_against_recursion {
                        RECURSIVE_COUNT_ArbitraryEnumIntC.with(|count| {
                            count.set(count.get() - 1);
                        });
                    }
                    result
                }
                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    arbitrary::size_hint::and(
                        <u32 as arbitrary::Arbitrary>::size_hint(depth),
                        arbitrary::size_hint::recursion_guard(depth, |depth| {
                            arbitrary::size_hint::or_all(&[
                                arbitrary::size_hint::and_all(&[]),
                                arbitrary::size_hint::and_all(&[]),
                                arbitrary::size_hint::and_all(&[]),
                            ])
                        }),
                    )
                }
            }
        };
        impl soroban_sdk::testutils::arbitrary::SorobanArbitrary for EnumIntC {
            type Prototype = ArbitraryEnumIntC;
        }
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, ArbitraryEnumIntC> for EnumIntC {
            type Error = soroban_sdk::ConversionError;
            fn try_from_val(
                env: &soroban_sdk::Env,
                v: &ArbitraryEnumIntC,
            ) -> std::result::Result<Self, Self::Error> {
                Ok(match v {
                    ArbitraryEnumIntC::V1 => EnumIntC::V1,
                    ArbitraryEnumIntC::V2 => EnumIntC::V2,
                    ArbitraryEnumIntC::V3 => EnumIntC::V3,
                })
            }
        }
    };
    pub enum ErrorA {
        E1 = 1,
        E2 = 2,
        E3 = 3,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ErrorA::E1 => "E1",
                    ErrorA::E2 => "E2",
                    ErrorA::E3 => "E3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorA {}
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorA {
        #[inline]
        fn clone(&self) -> ErrorA {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ErrorA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ErrorA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ErrorA {
        #[inline]
        fn eq(&self, other: &ErrorA) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ErrorA {
        #[inline]
        fn cmp(&self, other: &ErrorA) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ErrorA {
        #[inline]
        fn partial_cmp(&self, other: &ErrorA) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    pub static __SPEC_XDR_TYPE_ERRORA: [u8; 76usize] = ErrorA::spec_xdr();
    impl ErrorA {
        pub const fn spec_xdr() -> [u8; 76usize] {
            *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorA\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0\x01\0\0\0\0\0\0\0\x02E2\0\0\0\0\0\x02\0\0\0\0\0\0\0\x02E3\0\0\0\0\0\x03"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for ErrorA {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {}
    }
    impl TryFrom<soroban_sdk::Error> for ErrorA {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    1u32 => Self::E1,
                    2u32 => Self::E2,
                    3u32 => Self::E3,
                    _ => return Err(error),
                })
            } else {
                Err(error)
            }
        }
    }
    impl TryFrom<&soroban_sdk::Error> for ErrorA {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
        }
    }
    impl From<ErrorA> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: ErrorA) -> soroban_sdk::Error {
            <_ as From<&ErrorA>>::from(&val)
        }
    }
    impl From<&ErrorA> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: &ErrorA) -> soroban_sdk::Error {
            match val {
                ErrorA::E1 => soroban_sdk::Error::from_contract_error(1u32),
                ErrorA::E2 => soroban_sdk::Error::from_contract_error(2u32),
                ErrorA::E3 => soroban_sdk::Error::from_contract_error(3u32),
            }
        }
    }
    impl TryFrom<soroban_sdk::InvokeError> for ErrorA {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            match error {
                soroban_sdk::InvokeError::Abort => Err(error),
                soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                    1u32 => Self::E1,
                    2u32 => Self::E2,
                    3u32 => Self::E3,
                    _ => return Err(error),
                }),
            }
        }
    }
    impl TryFrom<&soroban_sdk::InvokeError> for ErrorA {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
        }
    }
    impl From<ErrorA> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: ErrorA) -> soroban_sdk::InvokeError {
            <_ as From<&ErrorA>>::from(&val)
        }
    }
    impl From<&ErrorA> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: &ErrorA) -> soroban_sdk::InvokeError {
            match val {
                ErrorA::E1 => soroban_sdk::InvokeError::Contract(1u32),
                ErrorA::E2 => soroban_sdk::InvokeError::Contract(2u32),
                ErrorA::E3 => soroban_sdk::InvokeError::Contract(3u32),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorA {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let error: soroban_sdk::Error = val.try_into_val(env)?;
            error.try_into().map_err(|_| soroban_sdk::ConversionError)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &ErrorA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            let error: soroban_sdk::Error = val.into();
            Ok(error.into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorA> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&ErrorA,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorA>>::try_from_val(env, *val)
        }
    }
    pub enum ErrorB {
        E1 = 10,
        E2 = 11,
        E3 = 12,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ErrorB::E1 => "E1",
                    ErrorB::E2 => "E2",
                    ErrorB::E3 => "E3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorB {}
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorB {
        #[inline]
        fn clone(&self) -> ErrorB {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ErrorB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ErrorB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ErrorB {
        #[inline]
        fn eq(&self, other: &ErrorB) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ErrorB {
        #[inline]
        fn cmp(&self, other: &ErrorB) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ErrorB {
        #[inline]
        fn partial_cmp(&self, other: &ErrorB) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    pub static __SPEC_XDR_TYPE_ERRORB: [u8; 76usize] = ErrorB::spec_xdr();
    impl ErrorB {
        pub const fn spec_xdr() -> [u8; 76usize] {
            *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorB\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0\n\0\0\0\0\0\0\0\x02E2\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02E3\0\0\0\0\0\x0c"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for ErrorB {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {}
    }
    impl TryFrom<soroban_sdk::Error> for ErrorB {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    10u32 => Self::E1,
                    11u32 => Self::E2,
                    12u32 => Self::E3,
                    _ => return Err(error),
                })
            } else {
                Err(error)
            }
        }
    }
    impl TryFrom<&soroban_sdk::Error> for ErrorB {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
        }
    }
    impl From<ErrorB> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: ErrorB) -> soroban_sdk::Error {
            <_ as From<&ErrorB>>::from(&val)
        }
    }
    impl From<&ErrorB> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: &ErrorB) -> soroban_sdk::Error {
            match val {
                ErrorB::E1 => soroban_sdk::Error::from_contract_error(10u32),
                ErrorB::E2 => soroban_sdk::Error::from_contract_error(11u32),
                ErrorB::E3 => soroban_sdk::Error::from_contract_error(12u32),
            }
        }
    }
    impl TryFrom<soroban_sdk::InvokeError> for ErrorB {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            match error {
                soroban_sdk::InvokeError::Abort => Err(error),
                soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                    10u32 => Self::E1,
                    11u32 => Self::E2,
                    12u32 => Self::E3,
                    _ => return Err(error),
                }),
            }
        }
    }
    impl TryFrom<&soroban_sdk::InvokeError> for ErrorB {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
        }
    }
    impl From<ErrorB> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: ErrorB) -> soroban_sdk::InvokeError {
            <_ as From<&ErrorB>>::from(&val)
        }
    }
    impl From<&ErrorB> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: &ErrorB) -> soroban_sdk::InvokeError {
            match val {
                ErrorB::E1 => soroban_sdk::InvokeError::Contract(10u32),
                ErrorB::E2 => soroban_sdk::InvokeError::Contract(11u32),
                ErrorB::E3 => soroban_sdk::InvokeError::Contract(12u32),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorB {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let error: soroban_sdk::Error = val.try_into_val(env)?;
            error.try_into().map_err(|_| soroban_sdk::ConversionError)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &ErrorB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            let error: soroban_sdk::Error = val.into();
            Ok(error.into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorB> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&ErrorB,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorB>>::try_from_val(env, *val)
        }
    }
    pub enum ErrorC {
        E1 = 100,
        E2 = 101,
        E3 = 102,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ErrorC::E1 => "E1",
                    ErrorC::E2 => "E2",
                    ErrorC::E3 => "E3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorC {}
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorC {
        #[inline]
        fn clone(&self) -> ErrorC {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ErrorC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ErrorC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ErrorC {
        #[inline]
        fn eq(&self, other: &ErrorC) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for ErrorC {
        #[inline]
        fn cmp(&self, other: &ErrorC) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for ErrorC {
        #[inline]
        fn partial_cmp(&self, other: &ErrorC) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    pub static __SPEC_XDR_TYPE_ERRORC: [u8; 76usize] = ErrorC::spec_xdr();
    impl ErrorC {
        pub const fn spec_xdr() -> [u8; 76usize] {
            *b"\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\x06ErrorC\0\0\0\0\0\x03\0\0\0\0\0\0\0\x02E1\0\0\0\0\0d\0\0\0\0\0\0\0\x02E2\0\0\0\0\0e\0\0\0\0\0\0\0\x02E3\0\0\0\0\0f"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for ErrorC {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {}
    }
    impl TryFrom<soroban_sdk::Error> for ErrorC {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            if error.is_type(soroban_sdk::xdr::ScErrorType::Contract) {
                let discriminant = error.get_code();
                Ok(match discriminant {
                    100u32 => Self::E1,
                    101u32 => Self::E2,
                    102u32 => Self::E3,
                    _ => return Err(error),
                })
            } else {
                Err(error)
            }
        }
    }
    impl TryFrom<&soroban_sdk::Error> for ErrorC {
        type Error = soroban_sdk::Error;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::Error) -> Result<Self, soroban_sdk::Error> {
            <_ as TryFrom<soroban_sdk::Error>>::try_from(*error)
        }
    }
    impl From<ErrorC> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: ErrorC) -> soroban_sdk::Error {
            <_ as From<&ErrorC>>::from(&val)
        }
    }
    impl From<&ErrorC> for soroban_sdk::Error {
        #[inline(always)]
        fn from(val: &ErrorC) -> soroban_sdk::Error {
            match val {
                ErrorC::E1 => soroban_sdk::Error::from_contract_error(100u32),
                ErrorC::E2 => soroban_sdk::Error::from_contract_error(101u32),
                ErrorC::E3 => soroban_sdk::Error::from_contract_error(102u32),
            }
        }
    }
    impl TryFrom<soroban_sdk::InvokeError> for ErrorC {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            match error {
                soroban_sdk::InvokeError::Abort => Err(error),
                soroban_sdk::InvokeError::Contract(code) => Ok(match code {
                    100u32 => Self::E1,
                    101u32 => Self::E2,
                    102u32 => Self::E3,
                    _ => return Err(error),
                }),
            }
        }
    }
    impl TryFrom<&soroban_sdk::InvokeError> for ErrorC {
        type Error = soroban_sdk::InvokeError;
        #[inline(always)]
        fn try_from(error: &soroban_sdk::InvokeError) -> Result<Self, soroban_sdk::InvokeError> {
            <_ as TryFrom<soroban_sdk::InvokeError>>::try_from(*error)
        }
    }
    impl From<ErrorC> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: ErrorC) -> soroban_sdk::InvokeError {
            <_ as From<&ErrorC>>::from(&val)
        }
    }
    impl From<&ErrorC> for soroban_sdk::InvokeError {
        #[inline(always)]
        fn from(val: &ErrorC) -> soroban_sdk::InvokeError {
            match val {
                ErrorC::E1 => soroban_sdk::InvokeError::Contract(100u32),
                ErrorC::E2 => soroban_sdk::InvokeError::Contract(101u32),
                ErrorC::E3 => soroban_sdk::InvokeError::Contract(102u32),
            }
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val> for ErrorC {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &soroban_sdk::Val,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            use soroban_sdk::TryIntoVal;
            let error: soroban_sdk::Error = val.try_into_val(env)?;
            error.try_into().map_err(|_| soroban_sdk::ConversionError)
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &ErrorC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            let error: soroban_sdk::Error = val.into();
            Ok(error.into())
        }
    }
    impl soroban_sdk::TryFromVal<soroban_sdk::Env, &ErrorC> for soroban_sdk::Val {
        type Error = soroban_sdk::ConversionError;
        #[inline(always)]
        fn try_from_val(
            env: &soroban_sdk::Env,
            val: &&ErrorC,
        ) -> Result<Self, soroban_sdk::ConversionError> {
            <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, ErrorC>>::try_from_val(env, *val)
        }
    }
    pub struct EventA {
        pub f1: soroban_sdk::Address,
        pub f2: soroban_sdk::String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EventA {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f, "EventA", "f1", &self.f1, "f2", &&self.f2,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EventA {
        #[inline]
        fn clone(&self) -> EventA {
            EventA {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EventA {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::String>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EventA {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EventA {
        #[inline]
        fn eq(&self, other: &EventA) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EventA {
        #[inline]
        fn cmp(&self, other: &EventA) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f2, &other.f2),
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EventA {
        #[inline]
        fn partial_cmp(&self, other: &EventA) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2)
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_EVENT_EVENTA: [u8; 88usize] = EventA::spec_xdr();
    impl EventA {
        pub const fn spec_xdr() -> [u8; 88usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventA\0\0\0\0\0\x01\0\0\0\x07event_a\0\0\0\0\x02\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0\0\0\0\x02"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EventA {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <soroban_sdk::Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <soroban_sdk::String as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::Event for EventA {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_a");
                    SYMBOL
                },
                {
                    let v: soroban_sdk::Val = self.f1.into_val(env);
                    v
                },
            )
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
            const KEYS: [&'static str; 1usize] = ["f2"];
            let vals: [soroban_sdk::Val; 1usize] = [self.f2.into_val(env)];
            env.map_new_from_slices(&KEYS, &vals)
                .unwrap_infallible()
                .into()
        }
    }
    impl EventA {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <Self as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
    pub struct EventB {
        pub f1: soroban_sdk::Address,
        pub f2: soroban_sdk::Address,
        pub f3: i128,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EventB {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f, "EventB", "f1", &self.f1, "f2", &self.f2, "f3", &&self.f3,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EventB {
        #[inline]
        fn clone(&self) -> EventB {
            EventB {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
                f3: ::core::clone::Clone::clone(&self.f3),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EventB {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Address>;
            let _: ::core::cmp::AssertParamIsEq<i128>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EventB {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EventB {
        #[inline]
        fn eq(&self, other: &EventB) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2 && self.f3 == other.f3
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EventB {
        #[inline]
        fn cmp(&self, other: &EventB) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.f2, &other.f2) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f3, &other.f3),
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EventB {
        #[inline]
        fn partial_cmp(&self, other: &EventB) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.f3, &other.f3)
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_EVENT_EVENTB: [u8; 108usize] = EventB::spec_xdr();
    impl EventB {
        pub const fn spec_xdr() -> [u8; 108usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventB\0\0\0\0\0\x01\0\0\0\x07event_b\0\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x13\0\0\0\x01\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x0b\0\0\0\0\0\0\0\x02"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EventB {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <soroban_sdk::Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <soroban_sdk::Address as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <i128 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::Event for EventB {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_b");
                    SYMBOL
                },
                {
                    let v: soroban_sdk::Val = self.f1.into_val(env);
                    v
                },
                {
                    let v: soroban_sdk::Val = self.f2.into_val(env);
                    v
                },
            )
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
            const KEYS: [&'static str; 1usize] = ["f3"];
            let vals: [soroban_sdk::Val; 1usize] = [self.f3.into_val(env)];
            env.map_new_from_slices(&KEYS, &vals)
                .unwrap_infallible()
                .into()
        }
    }
    impl EventB {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <Self as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
    pub struct EventC {
        pub f1: soroban_sdk::Symbol,
        pub f2: i64,
        pub f3: i64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EventC {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f, "EventC", "f1", &self.f1, "f2", &self.f2, "f3", &&self.f3,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EventC {
        #[inline]
        fn clone(&self) -> EventC {
            EventC {
                f1: ::core::clone::Clone::clone(&self.f1),
                f2: ::core::clone::Clone::clone(&self.f2),
                f3: ::core::clone::Clone::clone(&self.f3),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EventC {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<soroban_sdk::Symbol>;
            let _: ::core::cmp::AssertParamIsEq<i64>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EventC {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EventC {
        #[inline]
        fn eq(&self, other: &EventC) -> bool {
            self.f1 == other.f1 && self.f2 == other.f2 && self.f3 == other.f3
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for EventC {
        #[inline]
        fn cmp(&self, other: &EventC) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.f1, &other.f1) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.f2, &other.f2) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.f3, &other.f3),
                    cmp => cmp,
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for EventC {
        #[inline]
        fn partial_cmp(&self, other: &EventC) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.f1, &other.f1) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.f2, &other.f2) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::cmp::PartialOrd::partial_cmp(&self.f3, &other.f3)
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    pub static __SPEC_XDR_EVENT_EVENTC: [u8; 108usize] = EventC::spec_xdr();
    impl EventC {
        pub const fn spec_xdr() -> [u8; 108usize] {
            *b"\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x06EventC\0\0\0\0\0\x01\0\0\0\x07event_c\0\0\0\0\x03\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x11\0\0\0\x01\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\0\0\0\x02f3\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02"
        }
    }
    impl soroban_sdk::IncludeSpecMarker for EventC {
        #[doc(hidden)]
        #[inline(always)]
        fn include_spec_marker() {
            <soroban_sdk::Symbol as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <i64 as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
        }
    }
    impl soroban_sdk::Event for EventC {
        fn topics(&self, env: &soroban_sdk::Env) -> soroban_sdk::Vec<soroban_sdk::Val> {
            use soroban_sdk::IntoVal;
            (
                &{
                    #[allow(deprecated)]
                    const SYMBOL: soroban_sdk::Symbol = soroban_sdk::Symbol::short("event_c");
                    SYMBOL
                },
                {
                    let v: soroban_sdk::Val = self.f1.into_val(env);
                    v
                },
            )
                .into_val(env)
        }
        fn data(&self, env: &soroban_sdk::Env) -> soroban_sdk::Val {
            use soroban_sdk::{unwrap::UnwrapInfallible, EnvBase, IntoVal};
            const KEYS: [&'static str; 2usize] = ["f2", "f3"];
            let vals: [soroban_sdk::Val; 2usize] = [self.f2.into_val(env), self.f3.into_val(env)];
            env.map_new_from_slices(&KEYS, &vals)
                .unwrap_infallible()
                .into()
        }
    }
    impl EventC {
        pub fn publish(&self, env: &soroban_sdk::Env) {
            <Self as soroban_sdk::IncludeSpecMarker>::include_spec_marker();
            <_ as soroban_sdk::Event>::publish(self, env);
        }
    }
}
pub use imported::{EnumA, EnumIntA, ErrorA, EventA, StructA, StructTupleA};
pub struct Contract;
///ContractArgs is a type for building arg lists for functions defined in "Contract".
pub struct ContractArgs;
///ContractClient is a client for calling the contract defined in "Contract".
pub struct ContractClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    set_auths: Option<&'a [soroban_sdk::xdr::SorobanAuthorizationEntry]>,
    #[doc(hidden)]
    mock_auths: Option<&'a [soroban_sdk::testutils::MockAuth<'a>]>,
    #[doc(hidden)]
    mock_all_auths: bool,
    #[doc(hidden)]
    allow_non_root_auth: bool,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Set authorizations in the environment which will be consumed by
    /// contracts when they invoke `Address::require_auth` or
    /// `Address::require_auth_for_args` functions.
    ///
    /// Requires valid signatures for the authorization to be successful.
    /// To mock auth without requiring valid signatures, use `mock_auths`.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn set_auths(&self, auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: Some(auths),
            mock_auths: self.mock_auths.clone(),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock authorizations in the environment which will cause matching invokes
    /// of `Address::require_auth` and `Address::require_auth_for_args` to
    /// pass.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn mock_auths(&self, mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: self.set_auths.clone(),
            mock_auths: Some(mock_auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock all calls to the `Address::require_auth` and
    /// `Address::require_auth_for_args` functions in invoked contracts,
    /// having them succeed as if authorization was provided.
    ///
    /// See `soroban_sdk::Env::mock_all_auths` for more details and
    /// examples.
    pub fn mock_all_auths(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: false,
        }
    }
    /// A version of `mock_all_auths` that allows authorizations that
    /// are not present in the root invocation.
    ///
    /// Refer to `mock_all_auths` documentation for details and
    /// prefer using `mock_all_auths` unless non-root authorization is
    /// required.
    ///
    /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
    /// for more details and examples.
    pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: true,
        }
    }
}
mod __contract_fn_set_registry {
    use super::*;
    extern crate std;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    pub type F = soroban_sdk::testutils::ContractFunctionF;
    static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());
    pub fn register(name: &'static str, func: &'static F) {
        FUNCS.lock().unwrap().insert(name, func);
    }
    pub fn call(
        name: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        let fopt: Option<&'static F> = FUNCS.lock().unwrap().get(name).map(|f| f.clone());
        fopt.map(|f| f(env, args))
    }
}
impl soroban_sdk::testutils::ContractFunctionRegister for Contract {
    fn register(name: &'static str, func: &'static __contract_fn_set_registry::F) {
        __contract_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for Contract {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contract_fn_set_registry::call(func, env, args)
    }
}
impl Contract {
    pub fn wrap_struct_a(env: Env, contract_id: Address, f1: u32, f2: bool) -> StructA {
        imported::Client::new(&env, &contract_id).create_struct_a(&f1, &f2)
    }
    pub fn wrap_struct_tuple_a(env: Env, contract_id: Address, f1: i64, f2: i64) -> StructTupleA {
        imported::Client::new(&env, &contract_id).create_struct_tuple_a(&f1, &f2)
    }
    pub fn wrap_enum_a(env: Env, contract_id: Address) -> EnumA {
        imported::Client::new(&env, &contract_id).get_enum_a()
    }
    pub fn wrap_enum_int_a(env: Env, contract_id: Address) -> EnumIntA {
        imported::Client::new(&env, &contract_id).get_enum_int_a()
    }
    pub fn wrap_check_a(env: Env, contract_id: Address, input: u32) -> u32 {
        imported::Client::new(&env, &contract_id).check_a(&input)
    }
    pub fn wrap_emit_event_a(env: Env, contract_id: Address, f1: Address, f2: String) {
        imported::Client::new(&env, &contract_id).emit_event_a(&f1, &f2);
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_struct_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_WRAP_STRUCT_A: [u8; 108usize] =
        super::Contract::spec_xdr_wrap_struct_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrap_struct_a() -> [u8; 108usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\rwrap_struct_a\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x04\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x01\0\0\0\x01\0\0\x07\xd0\0\0\0\x07StructA\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_struct_tuple_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_WRAP_STRUCT_TUPLE_A: [u8; 116usize] =
        super::Contract::spec_xdr_wrap_struct_tuple_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrap_struct_tuple_a() -> [u8; 116usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x13wrap_struct_tuple_a\0\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x07\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x07\0\0\0\x01\0\0\x07\xd0\0\0\0\x0cStructTupleA"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_enum_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_WRAP_ENUM_A: [u8; 72usize] = super::Contract::spec_xdr_wrap_enum_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrap_enum_a() -> [u8; 72usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0bwrap_enum_a\0\0\0\0\x01\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\x01\0\0\x07\xd0\0\0\0\x05EnumA\0\0\0"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_enum_int_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_WRAP_ENUM_INT_A: [u8; 76usize] =
        super::Contract::spec_xdr_wrap_enum_int_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrap_enum_int_a() -> [u8; 76usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0fwrap_enum_int_a\0\0\0\0\x01\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\x01\0\0\x07\xd0\0\0\0\x08EnumIntA"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_check_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_WRAP_CHECK_A: [u8; 80usize] = super::Contract::spec_xdr_wrap_check_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrap_check_a() -> [u8; 80usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x0cwrap_check_a\0\0\0\x02\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x05input\0\0\0\0\0\0\x04\0\0\0\x01\0\0\0\x04"
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_emit_event_a__spec {
    #[doc(hidden)]
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub static __SPEC_XDR_FN_WRAP_EMIT_EVENT_A: [u8; 96usize] =
        super::Contract::spec_xdr_wrap_emit_event_a();
}
impl Contract {
    #[allow(non_snake_case)]
    pub const fn spec_xdr_wrap_emit_event_a() -> [u8; 96usize] {
        *b"\0\0\0\0\0\0\0\0\0\0\0\x11wrap_emit_event_a\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\x0bcontract_id\0\0\0\0\x13\0\0\0\0\0\0\0\x02f1\0\0\0\0\0\x13\0\0\0\0\0\0\0\x02f2\0\0\0\0\0\x10\0\0\0\0"
    }
}
impl<'a> ContractClient<'a> {
    pub fn wrap_struct_a(&self, contract_id: &Address, f1: &u32, f2: &bool) -> StructA {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_wrap_struct_a(
        &self,
        contract_id: &Address,
        f1: &u32,
        f2: &bool,
    ) -> Result<
        Result<
            StructA,
            <StructA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_struct_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn wrap_struct_tuple_a(&self, contract_id: &Address, f1: &i64, f2: &i64) -> StructTupleA {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_wrap_struct_tuple_a(
        &self,
        contract_id: &Address,
        f1: &i64,
        f2: &i64,
    ) -> Result<
        Result<
            StructTupleA,
            <StructTupleA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_struct_tuple_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn wrap_enum_a(&self, contract_id: &Address) -> EnumA {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_enum_a") },
            ::soroban_sdk::Vec::from_array(&self.env, [contract_id.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_wrap_enum_a(
        &self,
        contract_id: &Address,
    ) -> Result<
        Result<
            EnumA,
            <EnumA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_enum_a") },
            ::soroban_sdk::Vec::from_array(&self.env, [contract_id.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn wrap_enum_int_a(&self, contract_id: &Address) -> EnumIntA {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_enum_int_a") },
            ::soroban_sdk::Vec::from_array(&self.env, [contract_id.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_wrap_enum_int_a(
        &self,
        contract_id: &Address,
    ) -> Result<
        Result<
            EnumIntA,
            <EnumIntA as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error,
        >,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_enum_int_a") },
            ::soroban_sdk::Vec::from_array(&self.env, [contract_id.into_val(&self.env)]),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn wrap_check_a(&self, contract_id: &Address, input: &u32) -> u32 {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_check_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [contract_id.into_val(&self.env), input.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_wrap_check_a(
        &self,
        contract_id: &Address,
        input: &u32,
    ) -> Result<
        Result<u32, <u32 as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_check_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [contract_id.into_val(&self.env), input.into_val(&self.env)],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn wrap_emit_event_a(&self, contract_id: &Address, f1: &Address, f2: &String) -> () {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                if self.allow_non_root_auth {
                    self.env.mock_all_auths_allowing_non_root_auth();
                } else {
                    self.env.mock_all_auths();
                }
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_emit_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
    pub fn try_wrap_emit_event_a(
        &self,
        contract_id: &Address,
        f1: &Address,
        f2: &String,
    ) -> Result<
        Result<(), <() as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::Val>>::Error>,
        Result<soroban_sdk::Error, soroban_sdk::InvokeError>,
    > {
        use core::ops::Not;
        let old_auth_manager = self
            .env
            .in_contract()
            .not()
            .then(|| self.env.host().snapshot_auth_manager().unwrap());
        {
            if let Some(set_auths) = self.set_auths {
                self.env.set_auths(set_auths);
            }
            if let Some(mock_auths) = self.mock_auths {
                self.env.mock_auths(mock_auths);
            }
            if self.mock_all_auths {
                self.env.mock_all_auths();
            }
        }
        use soroban_sdk::{FromVal, IntoVal};
        let res = self.env.try_invoke_contract(
            &self.address,
            &{ soroban_sdk::Symbol::new(&self.env, "wrap_emit_event_a") },
            ::soroban_sdk::Vec::from_array(
                &self.env,
                [
                    contract_id.into_val(&self.env),
                    f1.into_val(&self.env),
                    f2.into_val(&self.env),
                ],
            ),
        );
        if let Some(old_auth_manager) = old_auth_manager {
            self.env.host().set_auth_manager(old_auth_manager).unwrap();
        }
        res
    }
}
impl ContractArgs {
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrap_struct_a<'i>(
        contract_id: &'i Address,
        f1: &'i u32,
        f2: &'i bool,
    ) -> (&'i Address, &'i u32, &'i bool) {
        (contract_id, f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrap_struct_tuple_a<'i>(
        contract_id: &'i Address,
        f1: &'i i64,
        f2: &'i i64,
    ) -> (&'i Address, &'i i64, &'i i64) {
        (contract_id, f1, f2)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrap_enum_a<'i>(contract_id: &'i Address) -> (&'i Address,) {
        (contract_id,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrap_enum_int_a<'i>(contract_id: &'i Address) -> (&'i Address,) {
        (contract_id,)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrap_check_a<'i>(contract_id: &'i Address, input: &'i u32) -> (&'i Address, &'i u32) {
        (contract_id, input)
    }
    #[inline(always)]
    #[allow(clippy::unused_unit)]
    pub fn wrap_emit_event_a<'i>(
        contract_id: &'i Address,
        f1: &'i Address,
        f2: &'i String,
    ) -> (&'i Address, &'i Address, &'i String) {
        (contract_id, f1, f2)
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_struct_a {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_struct_a` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::wrap_struct_a(
                env.clone(),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_1),
                ),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_2),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_struct_a` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 3usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    3usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env, args[0usize], args[1usize], args[2usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_struct_a` instead")]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_struct_tuple_a {
    use super::*;
    #[deprecated(
        note = "use `ContractClient::new(&env, &contract_id).wrap_struct_tuple_a` instead"
    )]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::wrap_struct_tuple_a(
                env.clone(),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_1),
                ),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_2),
                ),
            ),
            &env,
        )
    }
    #[deprecated(
        note = "use `ContractClient::new(&env, &contract_id).wrap_struct_tuple_a` instead"
    )]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 3usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    3usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env, args[0usize], args[1usize], args[2usize])
    }
    #[deprecated(
        note = "use `ContractClient::new(&env, &contract_id).wrap_struct_tuple_a` instead"
    )]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_enum_a {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_enum_a` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::wrap_enum_a(
                env.clone(),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_enum_a` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 1usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    1usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env, args[0usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_enum_a` instead")]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_enum_int_a {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_enum_int_a` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(env: soroban_sdk::Env, arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::wrap_enum_int_a(
                env.clone(),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_enum_int_a` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 1usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    1usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env, args[0usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_enum_int_a` instead")]
    pub extern "C" fn invoke_raw_extern(arg_0: soroban_sdk::Val) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_check_a {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_check_a` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::wrap_check_a(
                env.clone(),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_1),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_check_a` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 2usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    2usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env, args[0usize], args[1usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_check_a` instead")]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub mod __Contract__wrap_emit_event_a {
    use super::*;
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_emit_event_a` instead")]
    #[allow(deprecated)]
    pub fn invoke_raw(
        env: soroban_sdk::Env,
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        soroban_sdk::IntoValForContractFn::into_val_for_contract_fn(
            <super::Contract>::wrap_emit_event_a(
                env.clone(),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_0),
                ),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_1),
                ),
                <_ as soroban_sdk::unwrap::UnwrapOptimized>::unwrap_optimized(
                    <_ as soroban_sdk::TryFromValForContractFn<
                        soroban_sdk::Env,
                        soroban_sdk::Val,
                    >>::try_from_val_for_contract_fn(&env, &arg_2),
                ),
            ),
            &env,
        )
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_emit_event_a` instead")]
    pub fn invoke_raw_slice(env: soroban_sdk::Env, args: &[soroban_sdk::Val]) -> soroban_sdk::Val {
        if args.len() != 3usize {
            {
                ::core::panicking::panic_fmt(format_args!(
                    "invalid number of input arguments: {0} expected, got {1}",
                    3usize,
                    args.len(),
                ));
            };
        }
        #[allow(deprecated)]
        invoke_raw(env, args[0usize], args[1usize], args[2usize])
    }
    #[deprecated(note = "use `ContractClient::new(&env, &contract_id).wrap_emit_event_a` instead")]
    pub extern "C" fn invoke_raw_extern(
        arg_0: soroban_sdk::Val,
        arg_1: soroban_sdk::Val,
        arg_2: soroban_sdk::Val,
    ) -> soroban_sdk::Val {
        #[allow(deprecated)]
        invoke_raw(soroban_sdk::Env::default(), arg_0, arg_1, arg_2)
    }
    use super::*;
}
#[doc(hidden)]
#[allow(non_snake_case)]
#[allow(unused)]
fn __Contract__ce8e2bdbcdf6c60182dd70f69509d7ca88e28358cf52c984ae04155681c56c34_ctor() {
    #[allow(unsafe_code)]
    {
        #[link_section = ".init_array"]
        #[used]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[doc(hidden)]
        static f: extern "C" fn() -> ::ctor::__support::CtorRetType = {
            #[link_section = ".text.startup"]
            #[allow(non_snake_case)]
            extern "C" fn f() -> ::ctor::__support::CtorRetType {
                unsafe {
                    __Contract__ce8e2bdbcdf6c60182dd70f69509d7ca88e28358cf52c984ae04155681c56c34_ctor();
                };
                core::default::Default::default()
            }
            f
        };
    }
    {
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "wrap_struct_a",
            #[allow(deprecated)]
            &__Contract__wrap_struct_a::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "wrap_struct_tuple_a",
            #[allow(deprecated)]
            &__Contract__wrap_struct_tuple_a::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "wrap_enum_a",
            #[allow(deprecated)]
            &__Contract__wrap_enum_a::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "wrap_enum_int_a",
            #[allow(deprecated)]
            &__Contract__wrap_enum_int_a::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "wrap_check_a",
            #[allow(deprecated)]
            &__Contract__wrap_check_a::invoke_raw_slice,
        );
        <Contract as soroban_sdk::testutils::ContractFunctionRegister>::register(
            "wrap_emit_event_a",
            #[allow(deprecated)]
            &__Contract__wrap_emit_event_a::invoke_raw_slice,
        );
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_wrap_struct_a"]
    #[doc(hidden)]
    pub const test_wrap_struct_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wrap_struct_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_wasm/src/lib.rs",
            start_line: 60usize,
            start_col: 8usize,
            end_line: 60usize,
            end_col: 26usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wrap_struct_a()),
        ),
    };
    fn test_wrap_struct_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let s = client.wrap_struct_a(&imported_contract_id, &10, &true);
        match (&s.f1, &10) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&s.f2, &true) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_wrap_struct_tuple_a"]
    #[doc(hidden)]
    pub const test_wrap_struct_tuple_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wrap_struct_tuple_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_wasm/src/lib.rs",
            start_line: 72usize,
            start_col: 8usize,
            end_line: 72usize,
            end_col: 32usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wrap_struct_tuple_a()),
        ),
    };
    fn test_wrap_struct_tuple_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let s = client.wrap_struct_tuple_a(&imported_contract_id, &5, &10);
        match (&s.0, &5) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&s.1, &10) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_wrap_enum_a"]
    #[doc(hidden)]
    pub const test_wrap_enum_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wrap_enum_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_wasm/src/lib.rs",
            start_line: 84usize,
            start_col: 8usize,
            end_line: 84usize,
            end_col: 24usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wrap_enum_a()),
        ),
    };
    fn test_wrap_enum_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let v = client.wrap_enum_a(&imported_contract_id);
        match (&v, &EnumA::V2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_wrap_enum_int_a"]
    #[doc(hidden)]
    pub const test_wrap_enum_int_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wrap_enum_int_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_wasm/src/lib.rs",
            start_line: 95usize,
            start_col: 8usize,
            end_line: 95usize,
            end_col: 28usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wrap_enum_int_a()),
        ),
    };
    fn test_wrap_enum_int_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let v = client.wrap_enum_int_a(&imported_contract_id);
        match (&v, &EnumIntA::V3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_wrap_check_a"]
    #[doc(hidden)]
    pub const test_wrap_check_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wrap_check_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_wasm/src/lib.rs",
            start_line: 106usize,
            start_col: 8usize,
            end_line: 106usize,
            end_col: 25usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wrap_check_a()),
        ),
    };
    fn test_wrap_check_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let result = client.wrap_check_a(&imported_contract_id, &10);
        match (&result, &10) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "test::test_wrap_emit_event_a"]
    #[doc(hidden)]
    pub const test_wrap_emit_event_a: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("test::test_wrap_emit_event_a"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/spec_contract_import_wasm/src/lib.rs",
            start_line: 117usize,
            start_col: 8usize,
            end_line: 117usize,
            end_col: 30usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test_wrap_emit_event_a()),
        ),
    };
    fn test_wrap_emit_event_a() {
        let e = Env::default();
        let imported_contract_id = e.register(imported::WASM, ());
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);
        let f1 = Address::generate(&e);
        let f2 = String::from_str(&e, "test");
        client.wrap_emit_event_a(&imported_contract_id, &f1, &f2);
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &test_wrap_check_a,
        &test_wrap_emit_event_a,
        &test_wrap_enum_a,
        &test_wrap_enum_int_a,
        &test_wrap_struct_a,
        &test_wrap_struct_tuple_a,
    ])
}
