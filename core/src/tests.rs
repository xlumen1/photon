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
    assert!(test_instruction(Instruction::Adc));
}

#[test]
fn test_and() {
    cache_all();
    assert!(test_instruction(Instruction::And));
}

#[test]
fn test_asl() {
    cache_all();
    assert!(test_instruction(Instruction::Asl));
}

#[test]
fn test_bcc() {
    cache_all();
    assert!(test_instruction(Instruction::Bcc));
}

#[test]
fn test_bcs() {
    cache_all();
    assert!(test_instruction(Instruction::Bcs));
}

#[test]
fn test_beq() {
    cache_all();
    assert!(test_instruction(Instruction::Beq));
}

#[test]
fn test_bit() {
    cache_all();
    assert!(test_instruction(Instruction::Bit));
}

#[test]
fn test_bmi() {
    cache_all();
    assert!(test_instruction(Instruction::Bmi));
}

#[test]
fn test_bne() {
    cache_all();
    assert!(test_instruction(Instruction::Bne));
}

#[test]
fn test_bpl() {
    cache_all();
    assert!(test_instruction(Instruction::Bpl));
}

#[test]
fn test_bra() {
    cache_all();
    assert!(test_instruction(Instruction::Bra));
}

#[test]
fn test_brk() {
    cache_all();
    assert!(test_instruction(Instruction::Brk));
}

#[test]
fn test_brl() {
    cache_all();
    assert!(test_instruction(Instruction::Brl));
}

#[test]
fn test_bvc() {
    cache_all();
    assert!(test_instruction(Instruction::Bvc));
}

#[test]
fn test_bvs() {
    cache_all();
    assert!(test_instruction(Instruction::Bvs));
}

#[test]
fn test_clc() {
    cache_all();
    assert!(test_instruction(Instruction::Clc));
}

#[test]
fn test_cld() {
    cache_all();
    assert!(test_instruction(Instruction::Cld));
}

#[test]
fn test_cli() {
    cache_all();
    assert!(test_instruction(Instruction::Cli));
}

#[test]
fn test_clv() {
    cache_all();
    assert!(test_instruction(Instruction::Clv));
}

#[test]
fn test_cmp() {
    cache_all();
    assert!(test_instruction(Instruction::Cmp));
}

#[test]
fn test_cop() {
    cache_all();
    assert!(test_instruction(Instruction::Cop));
}

#[test]
fn test_cpx() {
    cache_all();
    assert!(test_instruction(Instruction::Cpx));
}

#[test]
fn test_cpy() {
    cache_all();
    assert!(test_instruction(Instruction::Cpy));
}

#[test]
fn test_dec() {
    cache_all();
    assert!(test_instruction(Instruction::Dec));
}

#[test]
fn test_dex() {
    cache_all();
    assert!(test_instruction(Instruction::Dex));
}

#[test]
fn test_dey() {
    cache_all();
    assert!(test_instruction(Instruction::Dey));
}

#[test]
fn test_eor() {
    cache_all();
    assert!(test_instruction(Instruction::Eor));
}

#[test]
fn test_inc() {
    cache_all();
    assert!(test_instruction(Instruction::Inc));
}

#[test]
fn test_inx() {
    cache_all();
    assert!(test_instruction(Instruction::Inx));
}

#[test]
fn test_iny() {
    cache_all();
    assert!(test_instruction(Instruction::Iny));
}

#[test]
fn test_jml() {
    cache_all();
    assert!(test_instruction(Instruction::Jml));
}

#[test]
fn test_jmp() {
    cache_all();
    assert!(test_instruction(Instruction::Jmp));
}

#[test]
fn test_jsl() {
    cache_all();
    assert!(test_instruction(Instruction::Jsl));
}

#[test]
fn test_jsr() {
    cache_all();
    assert!(test_instruction(Instruction::Jsr));
}

#[test]
fn test_lda() {
    cache_all();
    assert!(test_instruction(Instruction::Lda));
}

#[test]
fn test_ldx() {
    cache_all();
    assert!(test_instruction(Instruction::Ldx));
}

#[test]
fn test_ldy() {
    cache_all();
    assert!(test_instruction(Instruction::Ldy));
}

#[test]
fn test_lsr() {
    cache_all();
    assert!(test_instruction(Instruction::Lsr));
}

#[test]
fn test_mvn() {
    cache_all();
    assert!(test_instruction(Instruction::Mvn));
}

#[test]
fn test_mvp() {
    cache_all();
    assert!(test_instruction(Instruction::Mvp));
}

#[test]
fn test_nop() {
    cache_all();
    assert!(test_instruction(Instruction::Nop));
}

#[test]
fn test_ora() {
    cache_all();
    assert!(test_instruction(Instruction::Ora));
}

#[test]
fn test_pea() {
    cache_all();
    assert!(test_instruction(Instruction::Pea));
}

#[test]
fn test_pei() {
    cache_all();
    assert!(test_instruction(Instruction::Pei));
}

#[test]
fn test_per() {
    cache_all();
    assert!(test_instruction(Instruction::Per));
}

#[test]
fn test_pha() {
    cache_all();
    assert!(test_instruction(Instruction::Pha));
}

#[test]
fn test_phb() {
    cache_all();
    assert!(test_instruction(Instruction::Phb));
}

#[test]
fn test_phd() {
    cache_all();
    assert!(test_instruction(Instruction::Phd));
}

#[test]
fn test_phk() {
    cache_all();
    assert!(test_instruction(Instruction::Phk));
}

#[test]
fn test_php() {
    cache_all();
    assert!(test_instruction(Instruction::Php));
}

#[test]
fn test_phx() {
    cache_all();
    assert!(test_instruction(Instruction::Phx));
}

#[test]
fn test_phy() {
    cache_all();
    assert!(test_instruction(Instruction::Phy));
}

#[test]
fn test_pla() {
    cache_all();
    assert!(test_instruction(Instruction::Pla));
}

#[test]
fn test_plb() {
    cache_all();
    assert!(test_instruction(Instruction::Plb));
}

#[test]
fn test_pld() {
    cache_all();
    assert!(test_instruction(Instruction::Pld));
}

#[test]
fn test_plp() {
    cache_all();
    assert!(test_instruction(Instruction::Plp));
}

#[test]
fn test_plx() {
    cache_all();
    assert!(test_instruction(Instruction::Plx));
}

#[test]
fn test_ply() {
    cache_all();
    assert!(test_instruction(Instruction::Ply));
}

#[test]
fn test_rep() {
    cache_all();
    assert!(test_instruction(Instruction::Rep));
}

#[test]
fn test_rol() {
    cache_all();
    assert!(test_instruction(Instruction::Rol));
}

#[test]
fn test_ror() {
    cache_all();
    assert!(test_instruction(Instruction::Ror));
}

#[test]
fn test_rti() {
    cache_all();
    assert!(test_instruction(Instruction::Rti));
}

#[test]
fn test_rtl() {
    cache_all();
    assert!(test_instruction(Instruction::Rtl));
}

#[test]
fn test_rts() {
    cache_all();
    assert!(test_instruction(Instruction::Rts));
}

#[test]
fn test_sbc() {
    cache_all();
    assert!(test_instruction(Instruction::Sbc));
}

#[test]
fn test_sep() {
    cache_all();
    assert!(test_instruction(Instruction::Sep));
}

#[test]
fn test_sec() {
    cache_all();
    assert!(test_instruction(Instruction::Sec));
}

#[test]
fn test_sed() {
    cache_all();
    assert!(test_instruction(Instruction::Sed));
}

#[test]
fn test_sei() {
    cache_all();
    assert!(test_instruction(Instruction::Sei));
}

#[test]
fn test_sta() {
    cache_all();
    assert!(test_instruction(Instruction::Sta));
}

#[test]
fn test_stp() {
    cache_all();
    assert!(test_instruction(Instruction::Stp));
}

#[test]
fn test_stx() {
    cache_all();
    assert!(test_instruction(Instruction::Stx));
}

#[test]
fn test_sty() {
    cache_all();
    assert!(test_instruction(Instruction::Sty));
}

#[test]
fn test_stz() {
    cache_all();
    assert!(test_instruction(Instruction::Stz));
}

#[test]
fn test_tax() {
    cache_all();
    assert!(test_instruction(Instruction::Tax));
}

#[test]
fn test_tay() {
    cache_all();
    assert!(test_instruction(Instruction::Tay));
}

#[test]
fn test_txy() {
    cache_all();
    assert!(test_instruction(Instruction::Txy));
}

#[test]
fn test_tyx() {
    cache_all();
    assert!(test_instruction(Instruction::Tyx));
}

#[test]
fn test_tcd() {
    cache_all();
    assert!(test_instruction(Instruction::Tcd));
}

#[test]
fn test_tcs() {
    cache_all();
    assert!(test_instruction(Instruction::Tcs));
}

#[test]
fn test_tdc() {
    cache_all();
    assert!(test_instruction(Instruction::Tdc));
}

#[test]
fn test_trb() {
    cache_all();
    assert!(test_instruction(Instruction::Trb));
}

#[test]
fn test_tsb() {
    cache_all();
    assert!(test_instruction(Instruction::Tsb));
}

#[test]
fn test_tsc() {
    cache_all();
    assert!(test_instruction(Instruction::Tsc));
}

#[test]
fn test_tsx() {
    cache_all();
    assert!(test_instruction(Instruction::Tsx));
}

#[test]
fn test_txa() {
    cache_all();
    assert!(test_instruction(Instruction::Txa));
}

#[test]
fn test_txs() {
    cache_all();
    assert!(test_instruction(Instruction::Txs));
}

#[test]
fn test_tya() {
    cache_all();
    assert!(test_instruction(Instruction::Tya));
}

#[test]
fn test_wai() {
    cache_all();
    assert!(test_instruction(Instruction::Wai));
}

#[test]
fn test_wdm() {
    cache_all();
    assert!(test_instruction(Instruction::Wdm));
}

#[test]
fn test_xba() {
    cache_all();
    assert!(test_instruction(Instruction::Xba));
}

#[test]
fn test_xce() {
    cache_all();
    assert!(test_instruction(Instruction::Xce));
}

