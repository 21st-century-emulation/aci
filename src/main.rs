extern crate warp;
extern crate reqwest;

use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct CpuFlags {
    sign: bool,
    zero: bool,
    #[serde(rename = "auxCarry")] aux_carry: bool,
    parity: bool,
    carry: bool
}

#[derive(Deserialize, Serialize)]
struct CpuState {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    #[serde(rename = "stackPointer")] stack_pointer: u16,
    #[serde(rename = "programCounter")] program_counter: u16,
    cycles: u64,
    flags: CpuFlags
}

#[derive(Deserialize, Serialize)]
struct Cpu {
    state: CpuState,
    id: String,
    opcode: u8
}

async fn execute(query_string: HashMap<String, String>, mut cpu: Cpu) -> Result<impl warp::Reply, warp::Rejection> {
    cpu.state.cycles += 7;

    let operand = match query_string.get("operand1") {
        Some(x) => match x.parse::<u8>() {
            Ok(y) => y,
            _ => panic!("Invalid operand, not u8")
        },
        None => panic!("Missing operand")
    };

    let c = u8::from(cpu.state.flags.carry);
    let result = cpu.state.a.wrapping_add(operand).wrapping_add(c);
    cpu.state.flags.sign = (result & 0b1000_0000) == 0b1000_0000;
    cpu.state.flags.zero = result == 0;
    cpu.state.flags.aux_carry = (cpu.state.a & 0x0f) + (operand & 0x0f) + c > 0x0f;
    cpu.state.flags.parity = (result.count_ones() & 0b1) == 0;
    cpu.state.flags.carry = (u16::from(cpu.state.a) + u16::from(operand) + u16::from(c)) > 0xFF;
    cpu.state.a = result;

    Ok(warp::reply::json(&cpu))
}

#[tokio::main]
async fn main() {
    let status = warp::get()
        .and(warp::path!("status"))
        .map(|| {
            "Healthy"
        });

    let execute = warp::post()
        .and(warp::path!("api"/"v1"/"execute"))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(execute);

    warp::serve(execute.or(status)).run(([0, 0, 0, 0], 8080)).await
}