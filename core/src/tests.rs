use super::cpu::Instruction;

fn cache_all() {
    //todo!();
}

fn test_instruction(instr: Instruction) -> bool {
    let opcodes = instr.opcodes();
    println!("{:?}", opcodes);
    //todo!();
    true
}

#[test]
fn test_adc() {
    cache_all();
    assert!(test_instruction(Instruction::ADC));
}

#[test]
fn test_and() {
    cache_all();
    assert!(test_instruction(Instruction::AND));
}

#[test]
fn test_asl() {
    cache_all();
    assert!(test_instruction(Instruction::ASL));
}

#[test]
fn test_bcc() {
    cache_all();
    assert!(test_instruction(Instruction::BCC));
}

#[test]
fn test_bcs() {
    cache_all();
    assert!(test_instruction(Instruction::BCS));
}

#[test]
fn test_beq() {
    cache_all();
    assert!(test_instruction(Instruction::BEQ));
}

#[test]
fn test_bit() {
    cache_all();
    assert!(test_instruction(Instruction::BIT));
}

#[test]
fn test_bmi() {
    cache_all();
    assert!(test_instruction(Instruction::BMI));
}

#[test]
fn test_bne() {
    cache_all();
    assert!(test_instruction(Instruction::BNE));
}

#[test]
fn test_bpl() {
    cache_all();
    assert!(test_instruction(Instruction::BPL));
}

#[test]
fn test_bra() {
    cache_all();
    assert!(test_instruction(Instruction::BRA));
}

#[test]
fn test_brk() {
    cache_all();
    assert!(test_instruction(Instruction::BRK));
}

#[test]
fn test_brl() {
    cache_all();
    assert!(test_instruction(Instruction::BRL));
}

#[test]
fn test_bvc() {
    cache_all();
    assert!(test_instruction(Instruction::BVC));
}

#[test]
fn test_bvs() {
    cache_all();
    assert!(test_instruction(Instruction::BVS));
}

#[test]
fn test_clc() {
    cache_all();
    assert!(test_instruction(Instruction::CLC));
}

#[test]
fn test_cld() {
    cache_all();
    assert!(test_instruction(Instruction::CLD));
}

#[test]
fn test_cli() {
    cache_all();
    assert!(test_instruction(Instruction::CLI));
}

#[test]
fn test_clv() {
    cache_all();
    assert!(test_instruction(Instruction::CLV));
}

#[test]
fn test_cmp() {
    cache_all();
    assert!(test_instruction(Instruction::CMP));
}

#[test]
fn test_cop() {
    cache_all();
    assert!(test_instruction(Instruction::COP));
}

#[test]
fn test_cpx() {
    cache_all();
    assert!(test_instruction(Instruction::CPX));
}

#[test]
fn test_cpy() {
    cache_all();
    assert!(test_instruction(Instruction::CPY));
}

#[test]
fn test_dec() {
    cache_all();
    assert!(test_instruction(Instruction::DEC));
}

#[test]
fn test_dex() {
    cache_all();
    assert!(test_instruction(Instruction::DEX));
}

#[test]
fn test_dey() {
    cache_all();
    assert!(test_instruction(Instruction::DEY));
}

#[test]
fn test_eor() {
    cache_all();
    assert!(test_instruction(Instruction::EOR));
}

#[test]
fn test_inc() {
    cache_all();
    assert!(test_instruction(Instruction::INC));
}

#[test]
fn test_inx() {
    cache_all();
    assert!(test_instruction(Instruction::INX));
}

#[test]
fn test_iny() {
    cache_all();
    assert!(test_instruction(Instruction::INY));
}

#[test]
fn test_jml() {
    cache_all();
    assert!(test_instruction(Instruction::JML));
}

#[test]
fn test_jmp() {
    cache_all();
    assert!(test_instruction(Instruction::JMP));
}

#[test]
fn test_jsl() {
    cache_all();
    assert!(test_instruction(Instruction::JSL));
}

#[test]
fn test_jsr() {
    cache_all();
    assert!(test_instruction(Instruction::JSR));
}

#[test]
fn test_lda() {
    cache_all();
    assert!(test_instruction(Instruction::LDA));
}

#[test]
fn test_ldx() {
    cache_all();
    assert!(test_instruction(Instruction::LDX));
}

#[test]
fn test_ldy() {
    cache_all();
    assert!(test_instruction(Instruction::LDY));
}

#[test]
fn test_lsr() {
    cache_all();
    assert!(test_instruction(Instruction::LSR));
}

#[test]
fn test_mvn() {
    cache_all();
    assert!(test_instruction(Instruction::MVN));
}

#[test]
fn test_mvp() {
    cache_all();
    assert!(test_instruction(Instruction::MVP));
}

#[test]
fn test_nop() {
    cache_all();
    assert!(test_instruction(Instruction::NOP));
}

#[test]
fn test_ora() {
    cache_all();
    assert!(test_instruction(Instruction::ORA));
}

#[test]
fn test_pea() {
    cache_all();
    assert!(test_instruction(Instruction::PEA));
}

#[test]
fn test_pei() {
    cache_all();
    assert!(test_instruction(Instruction::PEI));
}

#[test]
fn test_per() {
    cache_all();
    assert!(test_instruction(Instruction::PER));
}

#[test]
fn test_pha() {
    cache_all();
    assert!(test_instruction(Instruction::PHA));
}

#[test]
fn test_phb() {
    cache_all();
    assert!(test_instruction(Instruction::PHB));
}

#[test]
fn test_phd() {
    cache_all();
    assert!(test_instruction(Instruction::PHD));
}

#[test]
fn test_phk() {
    cache_all();
    assert!(test_instruction(Instruction::PHK));
}

#[test]
fn test_php() {
    cache_all();
    assert!(test_instruction(Instruction::PHP));
}

#[test]
fn test_phx() {
    cache_all();
    assert!(test_instruction(Instruction::PHX));
}

#[test]
fn test_phy() {
    cache_all();
    assert!(test_instruction(Instruction::PHY));
}

#[test]
fn test_pla() {
    cache_all();
    assert!(test_instruction(Instruction::PLA));
}

#[test]
fn test_plb() {
    cache_all();
    assert!(test_instruction(Instruction::PLB));
}

#[test]
fn test_pld() {
    cache_all();
    assert!(test_instruction(Instruction::PLD));
}

#[test]
fn test_plp() {
    cache_all();
    assert!(test_instruction(Instruction::PLP));
}

#[test]
fn test_plx() {
    cache_all();
    assert!(test_instruction(Instruction::PLX));
}

#[test]
fn test_ply() {
    cache_all();
    assert!(test_instruction(Instruction::PLY));
}

#[test]
fn test_rep() {
    cache_all();
    assert!(test_instruction(Instruction::REP));
}

#[test]
fn test_rol() {
    cache_all();
    assert!(test_instruction(Instruction::ROL));
}

#[test]
fn test_ror() {
    cache_all();
    assert!(test_instruction(Instruction::ROR));
}

#[test]
fn test_rti() {
    cache_all();
    assert!(test_instruction(Instruction::RTI));
}

#[test]
fn test_rtl() {
    cache_all();
    assert!(test_instruction(Instruction::RTL));
}

#[test]
fn test_rts() {
    cache_all();
    assert!(test_instruction(Instruction::RTS));
}

#[test]
fn test_sbc() {
    cache_all();
    assert!(test_instruction(Instruction::SBC));
}

#[test]
fn test_sep() {
    cache_all();
    assert!(test_instruction(Instruction::SEP));
}

#[test]
fn test_sec() {
    cache_all();
    assert!(test_instruction(Instruction::SEC));
}

#[test]
fn test_sed() {
    cache_all();
    assert!(test_instruction(Instruction::SED));
}

#[test]
fn test_sei() {
    cache_all();
    assert!(test_instruction(Instruction::SEI));
}

#[test]
fn test_sta() {
    cache_all();
    assert!(test_instruction(Instruction::STA));
}

#[test]
fn test_stp() {
    cache_all();
    assert!(test_instruction(Instruction::STP));
}

#[test]
fn test_stx() {
    cache_all();
    assert!(test_instruction(Instruction::STX));
}

#[test]
fn test_sty() {
    cache_all();
    assert!(test_instruction(Instruction::STY));
}

#[test]
fn test_stz() {
    cache_all();
    assert!(test_instruction(Instruction::STZ));
}

#[test]
fn test_tax() {
    cache_all();
    assert!(test_instruction(Instruction::TAX));
}

#[test]
fn test_tay() {
    cache_all();
    assert!(test_instruction(Instruction::TAY));
}

#[test]
fn test_txy() {
    cache_all();
    assert!(test_instruction(Instruction::TXY));
}

#[test]
fn test_tyx() {
    cache_all();
    assert!(test_instruction(Instruction::TYX));
}

#[test]
fn test_tcd() {
    cache_all();
    assert!(test_instruction(Instruction::TCD));
}

#[test]
fn test_tcs() {
    cache_all();
    assert!(test_instruction(Instruction::TCS));
}

#[test]
fn test_tdc() {
    cache_all();
    assert!(test_instruction(Instruction::TDC));
}

#[test]
fn test_trb() {
    cache_all();
    assert!(test_instruction(Instruction::TRB));
}

#[test]
fn test_tsb() {
    cache_all();
    assert!(test_instruction(Instruction::TSB));
}

#[test]
fn test_tsc() {
    cache_all();
    assert!(test_instruction(Instruction::TSC));
}

#[test]
fn test_tsx() {
    cache_all();
    assert!(test_instruction(Instruction::TSX));
}

#[test]
fn test_txa() {
    cache_all();
    assert!(test_instruction(Instruction::TXA));
}

#[test]
fn test_txs() {
    cache_all();
    assert!(test_instruction(Instruction::TXS));
}

#[test]
fn test_tya() {
    cache_all();
    assert!(test_instruction(Instruction::TYA));
}

#[test]
fn test_wai() {
    cache_all();
    assert!(test_instruction(Instruction::WAI));
}

#[test]
fn test_wdm() {
    cache_all();
    assert!(test_instruction(Instruction::WDM));
}

#[test]
fn test_xba() {
    cache_all();
    assert!(test_instruction(Instruction::XBA));
}

#[test]
fn test_xce() {
    cache_all();
    assert!(test_instruction(Instruction::XCE));
}

