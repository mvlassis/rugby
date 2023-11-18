use std::process;

use super::CPU;
use crate::mmu::MMU;

impl CPU {

	// Populate the lookup tables with the correct function pointers
	pub fn build_lookup_tables(&mut self) {
		let mut lookup_table: [Option<fn(&mut CPU, &mut MMU)>; 256] = [None; 256];
		lookup_table[0x00] = Some(CPU::opcode_nop);
		lookup_table[0x01] = Some(CPU::opcode_ld_bc_nn);
		lookup_table[0x02] = Some(CPU::opcode_ld_bc_a);
		lookup_table[0x03] = Some(CPU::opcode_inc_bc);
		lookup_table[0x04] = Some(CPU::opcode_inc_b);
		lookup_table[0x05] = Some(CPU::opcode_dec_b);
		lookup_table[0x06] = Some(CPU::opcode_ld_b_n);
		lookup_table[0x07] = Some(CPU::opcode_rlca);
		lookup_table[0x08] = Some(CPU::opcode_ld_nn_sp);
		lookup_table[0x09] = Some(CPU::opcode_add_hl_bc);
		lookup_table[0x0B] = Some(CPU::opcode_dec_bc);
		lookup_table[0x0A] = Some(CPU::opcode_ld_a_bc);
		lookup_table[0x0C] = Some(CPU::opcode_inc_c);
		lookup_table[0x0D] = Some(CPU::opcode_dec_c);
		lookup_table[0x0E] = Some(CPU::opcode_ld_c_n);
		lookup_table[0x0F] = Some(CPU::opcode_rrca);

		lookup_table[0x11] = Some(CPU::opcode_ld_de_nn);
		lookup_table[0x12] = Some(CPU::opcode_ld_de_a);
		lookup_table[0x13] = Some(CPU::opcode_inc_de);
		lookup_table[0x14] = Some(CPU::opcode_inc_d);
		lookup_table[0x15] = Some(CPU::opcode_dec_d);
		lookup_table[0x16] = Some(CPU::opcode_ld_d_n);
		lookup_table[0x17] = Some(CPU::opcode_rla);
		lookup_table[0x18] = Some(CPU::opcode_jr_dd);
		lookup_table[0x19] = Some(CPU::opcode_add_hl_de);
		lookup_table[0x1A] = Some(CPU::opcode_ld_a_de);
		lookup_table[0x1B] = Some(CPU::opcode_dec_de);
		lookup_table[0x1C] = Some(CPU::opcode_inc_e);
		lookup_table[0x1D] = Some(CPU::opcode_dec_e);
		lookup_table[0x1E] = Some(CPU::opcode_ld_e_n);
		lookup_table[0x1F] = Some(CPU::opcode_rra);

		lookup_table[0x20] = Some(CPU::opcode_jr_nz_dd);
		lookup_table[0x22] = Some(CPU::opcode_ldi_hl_a);
		lookup_table[0x21] = Some(CPU::opcode_ld_hl_nn);
		lookup_table[0x23] = Some(CPU::opcode_inc_hl);
		lookup_table[0x24] = Some(CPU::opcode_inc_h);
		lookup_table[0x25] = Some(CPU::opcode_dec_h);
		lookup_table[0x26] = Some(CPU::opcode_ld_h_n);
		lookup_table[0x27] = Some(CPU::opcode_daa);
		lookup_table[0x28] = Some(CPU::opcode_jr_z_dd);
		lookup_table[0x29] = Some(CPU::opcode_add_hl_hl);
		lookup_table[0x2A] = Some(CPU::opcode_ldi_a_hl);
		lookup_table[0x2B] = Some(CPU::opcode_dec_hl);
		lookup_table[0x2C] = Some(CPU::opcode_inc_l);
		lookup_table[0x2D] = Some(CPU::opcode_dec_l);
		lookup_table[0x2E] = Some(CPU::opcode_ld_l_n);
		lookup_table[0x2F] = Some(CPU::opcode_cpl_a);

		lookup_table[0x30] = Some(CPU::opcode_jr_nc_dd);
		lookup_table[0x31] = Some(CPU::opcode_ld_sp_nn);
		lookup_table[0x32] = Some(CPU::opcode_ldd_hl_a);
		lookup_table[0x33] = Some(CPU::opcode_inc_sp);
		lookup_table[0x34] = Some(CPU::opcode_inc_m_hl);
		lookup_table[0x35] = Some(CPU::opcode_dec_m_hl);
		lookup_table[0x36] = Some(CPU::opcode_ld_hl_n);
		lookup_table[0x37] = Some(CPU::opcode_scf);
		lookup_table[0x38] = Some(CPU::opcode_jr_c_dd);
		lookup_table[0x39] = Some(CPU::opcode_add_hl_sp);
		lookup_table[0x3B] = Some(CPU::opcode_dec_sp);
		lookup_table[0x3A] = Some(CPU::opcode_ldd_a_hl);
		lookup_table[0x3C] = Some(CPU::opcode_inc_a);
		lookup_table[0x3D] = Some(CPU::opcode_dec_a);
		lookup_table[0x3E] = Some(CPU::opcode_ld_a_n);
		lookup_table[0x3F] = Some(CPU::opcode_ccf);
		
		lookup_table[0x40] = Some(CPU::opcode_ld_b_b);
		lookup_table[0x41] = Some(CPU::opcode_ld_b_c);
		lookup_table[0x42] = Some(CPU::opcode_ld_b_d);
		lookup_table[0x43] = Some(CPU::opcode_ld_b_e);
		lookup_table[0x44] = Some(CPU::opcode_ld_b_h);
		lookup_table[0x45] = Some(CPU::opcode_ld_b_l);
		lookup_table[0x46] = Some(CPU::opcode_ld_b_hl);
		lookup_table[0x47] = Some(CPU::opcode_ld_b_a);
		lookup_table[0x48] = Some(CPU::opcode_ld_c_b);
		lookup_table[0x49] = Some(CPU::opcode_ld_c_c);
		lookup_table[0x4A] = Some(CPU::opcode_ld_c_d);
		lookup_table[0x4B] = Some(CPU::opcode_ld_c_e);
		lookup_table[0x4C] = Some(CPU::opcode_ld_c_h);
		lookup_table[0x4D] = Some(CPU::opcode_ld_c_l);
		lookup_table[0x4E] = Some(CPU::opcode_ld_c_hl);
		lookup_table[0x4F] = Some(CPU::opcode_ld_c_a);

		lookup_table[0x50] = Some(CPU::opcode_ld_d_b);
		lookup_table[0x51] = Some(CPU::opcode_ld_d_c);
		lookup_table[0x52] = Some(CPU::opcode_ld_d_d);
		lookup_table[0x53] = Some(CPU::opcode_ld_d_e);
		lookup_table[0x54] = Some(CPU::opcode_ld_d_h);
		lookup_table[0x55] = Some(CPU::opcode_ld_d_l);
		lookup_table[0x56] = Some(CPU::opcode_ld_d_hl);
		lookup_table[0x57] = Some(CPU::opcode_ld_d_a);
		lookup_table[0x58] = Some(CPU::opcode_ld_e_b);
		lookup_table[0x59] = Some(CPU::opcode_ld_e_c);
		lookup_table[0x5A] = Some(CPU::opcode_ld_e_d);
		lookup_table[0x5B] = Some(CPU::opcode_ld_e_e);
		lookup_table[0x5C] = Some(CPU::opcode_ld_e_h);
		lookup_table[0x5D] = Some(CPU::opcode_ld_e_l);
		lookup_table[0x5E] = Some(CPU::opcode_ld_e_hl);
		lookup_table[0x5F] = Some(CPU::opcode_ld_e_a);

		lookup_table[0x60] = Some(CPU::opcode_ld_h_b);
		lookup_table[0x61] = Some(CPU::opcode_ld_h_c);
		lookup_table[0x62] = Some(CPU::opcode_ld_h_d);
		lookup_table[0x63] = Some(CPU::opcode_ld_h_e);
		lookup_table[0x64] = Some(CPU::opcode_ld_h_h);
		lookup_table[0x65] = Some(CPU::opcode_ld_h_l);
		lookup_table[0x66] = Some(CPU::opcode_ld_h_hl);
		lookup_table[0x67] = Some(CPU::opcode_ld_h_a);
		lookup_table[0x68] = Some(CPU::opcode_ld_l_b);
		lookup_table[0x69] = Some(CPU::opcode_ld_l_c);
		lookup_table[0x6A] = Some(CPU::opcode_ld_l_d);
		lookup_table[0x6B] = Some(CPU::opcode_ld_l_e);
		lookup_table[0x6C] = Some(CPU::opcode_ld_l_h);
		lookup_table[0x6D] = Some(CPU::opcode_ld_l_l);
		lookup_table[0x6E] = Some(CPU::opcode_ld_l_hl);
		lookup_table[0x6F] = Some(CPU::opcode_ld_l_a);

		lookup_table[0x70] = Some(CPU::opcode_ld_hl_b);
		lookup_table[0x71] = Some(CPU::opcode_ld_hl_c);
		lookup_table[0x72] = Some(CPU::opcode_ld_hl_d);
		lookup_table[0x73] = Some(CPU::opcode_ld_hl_e);
		lookup_table[0x74] = Some(CPU::opcode_ld_hl_h);
		lookup_table[0x75] = Some(CPU::opcode_ld_hl_l);
		lookup_table[0x77] = Some(CPU::opcode_ld_hl_a);
		lookup_table[0x78] = Some(CPU::opcode_ld_a_b);
		lookup_table[0x79] = Some(CPU::opcode_ld_a_c);
		lookup_table[0x7A] = Some(CPU::opcode_ld_a_d);
		lookup_table[0x7B] = Some(CPU::opcode_ld_a_e);
		lookup_table[0x7C] = Some(CPU::opcode_ld_a_h);
		lookup_table[0x7D] = Some(CPU::opcode_ld_a_l);
		lookup_table[0x7E] = Some(CPU::opcode_ld_a_hl);
		lookup_table[0x7F] = Some(CPU::opcode_ld_a_a);

		lookup_table[0x80] = Some(CPU::opcode_add_a_b);
		lookup_table[0x81] = Some(CPU::opcode_add_a_c);
		lookup_table[0x82] = Some(CPU::opcode_add_a_d);
		lookup_table[0x83] = Some(CPU::opcode_add_a_e);
		lookup_table[0x84] = Some(CPU::opcode_add_a_h);
		lookup_table[0x85] = Some(CPU::opcode_add_a_l);
		lookup_table[0x86] = Some(CPU::opcode_add_a_hl);
		lookup_table[0x87] = Some(CPU::opcode_add_a_a);
		lookup_table[0x88] = Some(CPU::opcode_adc_a_b);
		lookup_table[0x89] = Some(CPU::opcode_adc_a_c);
		lookup_table[0x8A] = Some(CPU::opcode_adc_a_d);
		lookup_table[0x8B] = Some(CPU::opcode_adc_a_e);
		lookup_table[0x8C] = Some(CPU::opcode_adc_a_h);
		lookup_table[0x8D] = Some(CPU::opcode_adc_a_l);
		lookup_table[0x8E] = Some(CPU::opcode_adc_a_hl);
		lookup_table[0x8F] = Some(CPU::opcode_adc_a_a);

		lookup_table[0x90] = Some(CPU::opcode_sub_a_b);
		lookup_table[0x91] = Some(CPU::opcode_sub_a_c);
		lookup_table[0x92] = Some(CPU::opcode_sub_a_d);
		lookup_table[0x93] = Some(CPU::opcode_sub_a_e);
		lookup_table[0x94] = Some(CPU::opcode_sub_a_h);
		lookup_table[0x95] = Some(CPU::opcode_sub_a_l);
		lookup_table[0x96] = Some(CPU::opcode_sub_a_hl);
		lookup_table[0x97] = Some(CPU::opcode_sub_a_a);
		lookup_table[0x98] = Some(CPU::opcode_sbc_a_b);
		lookup_table[0x99] = Some(CPU::opcode_sbc_a_c);
		lookup_table[0x9A] = Some(CPU::opcode_sbc_a_d);
		lookup_table[0x9B] = Some(CPU::opcode_sbc_a_e);
		lookup_table[0x9C] = Some(CPU::opcode_sbc_a_h);
		lookup_table[0x9D] = Some(CPU::opcode_sbc_a_l);
		lookup_table[0x9E] = Some(CPU::opcode_sbc_a_hl);
		lookup_table[0x9F] = Some(CPU::opcode_sbc_a_a);

		lookup_table[0xA0] = Some(CPU::opcode_and_a_b);
		lookup_table[0xA1] = Some(CPU::opcode_and_a_c);
		lookup_table[0xA2] = Some(CPU::opcode_and_a_d);
		lookup_table[0xA3] = Some(CPU::opcode_and_a_e);
		lookup_table[0xA4] = Some(CPU::opcode_and_a_h);
		lookup_table[0xA5] = Some(CPU::opcode_and_a_l);
		lookup_table[0xA6] = Some(CPU::opcode_and_a_hl);
		lookup_table[0xA7] = Some(CPU::opcode_and_a_a);
		lookup_table[0xA8] = Some(CPU::opcode_xor_a_b);
		lookup_table[0xA9] = Some(CPU::opcode_xor_a_c);
		lookup_table[0xAA] = Some(CPU::opcode_xor_a_d);
		lookup_table[0xAB] = Some(CPU::opcode_xor_a_e);
		lookup_table[0xAC] = Some(CPU::opcode_xor_a_h);
		lookup_table[0xAD] = Some(CPU::opcode_xor_a_l);
		lookup_table[0xAE] = Some(CPU::opcode_xor_a_hl);
		lookup_table[0xAF] = Some(CPU::opcode_xor_a_a);

		lookup_table[0xB0] = Some(CPU::opcode_or_a_b);
		lookup_table[0xB1] = Some(CPU::opcode_or_a_c);
		lookup_table[0xB2] = Some(CPU::opcode_or_a_d);
		lookup_table[0xB3] = Some(CPU::opcode_or_a_e);
		lookup_table[0xB4] = Some(CPU::opcode_or_a_h);
		lookup_table[0xB5] = Some(CPU::opcode_or_a_l);
		lookup_table[0xB6] = Some(CPU::opcode_or_a_hl);
		lookup_table[0xB7] = Some(CPU::opcode_or_a_a);
		lookup_table[0xB8] = Some(CPU::opcode_cp_a_b);
		lookup_table[0xB9] = Some(CPU::opcode_cp_a_c);
		lookup_table[0xBA] = Some(CPU::opcode_cp_a_d);
		lookup_table[0xBB] = Some(CPU::opcode_cp_a_e);
		lookup_table[0xBC] = Some(CPU::opcode_cp_a_h);
		lookup_table[0xBD] = Some(CPU::opcode_cp_a_l);
		lookup_table[0xBE] = Some(CPU::opcode_cp_a_hl);
		lookup_table[0xBF] = Some(CPU::opcode_cp_a_a);

		lookup_table[0xC0] = Some(CPU::opcode_ret_nz);
		lookup_table[0xC1] = Some(CPU::opcode_pop_bc);
		lookup_table[0xC2] = Some(CPU::opcode_jp_nz_nn);
		lookup_table[0xC3] = Some(CPU::opcode_jp_nn);
		lookup_table[0xC4] = Some(CPU::opcode_call_nz_nn);
		lookup_table[0xCE] = Some(CPU::opcode_adc_a_n);
		lookup_table[0xC5] = Some(CPU::opcode_push_bc);
		lookup_table[0xC6] = Some(CPU::opcode_add_a_n);
		lookup_table[0xC7] = Some(CPU::opcode_rst_0);
		lookup_table[0xC8] = Some(CPU::opcode_ret_z);
		lookup_table[0xC9] = Some(CPU::opcode_ret);
		lookup_table[0xCA] = Some(CPU::opcode_jp_z_nn);
		lookup_table[0xCC] = Some(CPU::opcode_call_z_nn);
		lookup_table[0xCD] = Some(CPU::opcode_call_nn);
		lookup_table[0xCF] = Some(CPU::opcode_rst_1);

		lookup_table[0xD0] = Some(CPU::opcode_ret_nc);
		lookup_table[0xD1] = Some(CPU::opcode_pop_de);
		lookup_table[0xD2] = Some(CPU::opcode_jp_nc_nn);
		lookup_table[0xD4] = Some(CPU::opcode_call_nc_nn);
		lookup_table[0xD5] = Some(CPU::opcode_push_de);
		lookup_table[0xD6] = Some(CPU::opcode_sub_a_n);
		lookup_table[0xD7] = Some(CPU::opcode_rst_2);
		lookup_table[0xD8] = Some(CPU::opcode_ret_c);
		lookup_table[0xD9] = Some(CPU::opcode_reti);
		lookup_table[0xDA] = Some(CPU::opcode_jp_c_nn);
		lookup_table[0xDC] = Some(CPU::opcode_call_c_nn);
		lookup_table[0xDE] = Some(CPU::opcode_sbc_a_n);
		lookup_table[0xDF] = Some(CPU::opcode_rst_3);
		
		lookup_table[0xE0] = Some(CPU::opcode_ldh_n_a);
		lookup_table[0xE1] = Some(CPU::opcode_pop_hl);
		lookup_table[0xE2] = Some(CPU::opcode_ldh_c_a);
		lookup_table[0xE5] = Some(CPU::opcode_push_hl);
		lookup_table[0xE6] = Some(CPU::opcode_and_a_n);
		lookup_table[0xE7] = Some(CPU::opcode_rst_4);
		lookup_table[0xE9] = Some(CPU::opcode_jp_hl);
		lookup_table[0xE8] = Some(CPU::opcode_add_sp_dd);
		lookup_table[0xEA] = Some(CPU::opcode_ld_nn_a);
		lookup_table[0xEE] = Some(CPU::opcode_xor_a_n);
		lookup_table[0xEF] = Some(CPU::opcode_rst_5);
		
		lookup_table[0xF0] = Some(CPU::opcode_ldh_a_n);
		lookup_table[0xF1] = Some(CPU::opcode_pop_af);
		lookup_table[0xF2] = Some(CPU::opcode_ldh_a_c);
		lookup_table[0xF3] = Some(CPU::opcode_di);
		lookup_table[0xF5] = Some(CPU::opcode_push_af);
		lookup_table[0xF6] = Some(CPU::opcode_or_a_n);
		lookup_table[0xF7] = Some(CPU::opcode_rst_6);
		lookup_table[0xF8] = Some(CPU::opcode_lds_hl_sp);
		lookup_table[0xF9] = Some(CPU::opcode_ld_sp_hl);
		lookup_table[0xFA] = Some(CPU::opcode_ld_a_nn);
		lookup_table[0xFB] = Some(CPU::opcode_ei);
		lookup_table[0xFE] = Some(CPU::opcode_cp_a_n);
		lookup_table[0xFF] = Some(CPU::opcode_rst_7);
		self.lookup_table = lookup_table;

		let mut lookup_table2: [Option<fn(&mut CPU, &mut MMU)>; 256] = [None; 256];
		lookup_table2[0x00] = Some(CPU::opcode_rlc_b);
		lookup_table2[0x01] = Some(CPU::opcode_rlc_c);
		lookup_table2[0x02] = Some(CPU::opcode_rlc_d);
		lookup_table2[0x03] = Some(CPU::opcode_rlc_e);
		lookup_table2[0x04] = Some(CPU::opcode_rlc_h);
		lookup_table2[0x05] = Some(CPU::opcode_rlc_l);
		lookup_table2[0x06] = Some(CPU::opcode_rlc_m_hl);
		lookup_table2[0x07] = Some(CPU::opcode_rlc_a);
		lookup_table2[0x08] = Some(CPU::opcode_rrc_b);
		lookup_table2[0x09] = Some(CPU::opcode_rrc_c);
		lookup_table2[0x0A] = Some(CPU::opcode_rrc_d);
		lookup_table2[0x0B] = Some(CPU::opcode_rrc_e);
		lookup_table2[0x0C] = Some(CPU::opcode_rrc_h);
		lookup_table2[0x0D] = Some(CPU::opcode_rrc_l);
		lookup_table2[0x0E] = Some(CPU::opcode_rrc_hl);
		lookup_table2[0x0F] = Some(CPU::opcode_rrc_a);

		lookup_table2[0x10] = Some(CPU::opcode_rl_b);
		lookup_table2[0x11] = Some(CPU::opcode_rl_c);
		lookup_table2[0x12] = Some(CPU::opcode_rl_d);
		lookup_table2[0x13] = Some(CPU::opcode_rl_e);
		lookup_table2[0x14] = Some(CPU::opcode_rl_h);
		lookup_table2[0x15] = Some(CPU::opcode_rl_l);
		lookup_table2[0x16] = Some(CPU::opcode_rl_hl);
		lookup_table2[0x17] = Some(CPU::opcode_rl_a);
		lookup_table2[0x18] = Some(CPU::opcode_rr_b);
		lookup_table2[0x19] = Some(CPU::opcode_rr_c);
		lookup_table2[0x1A] = Some(CPU::opcode_rr_d);
		lookup_table2[0x1B] = Some(CPU::opcode_rr_e);
		lookup_table2[0x1C] = Some(CPU::opcode_rr_h);
		lookup_table2[0x1D] = Some(CPU::opcode_rr_l);
		lookup_table2[0x1E] = Some(CPU::opcode_rr_hl);
		lookup_table2[0x1F] = Some(CPU::opcode_rr_a);

		lookup_table2[0x20] = Some(CPU::opcode_sla_b);
		lookup_table2[0x21] = Some(CPU::opcode_sla_c);
		lookup_table2[0x22] = Some(CPU::opcode_sla_d);
		lookup_table2[0x23] = Some(CPU::opcode_sla_e);
		lookup_table2[0x24] = Some(CPU::opcode_sla_h);
		lookup_table2[0x25] = Some(CPU::opcode_sla_l);
		lookup_table2[0x26] = Some(CPU::opcode_sla_hl);
		lookup_table2[0x27] = Some(CPU::opcode_sla_a);
		lookup_table2[0x28] = Some(CPU::opcode_sra_b);
		lookup_table2[0x29] = Some(CPU::opcode_sra_c);
		lookup_table2[0x2A] = Some(CPU::opcode_sra_d);
		lookup_table2[0x2B] = Some(CPU::opcode_sra_e);
		lookup_table2[0x2C] = Some(CPU::opcode_sra_h);
		lookup_table2[0x2D] = Some(CPU::opcode_sra_l);
		lookup_table2[0x2E] = Some(CPU::opcode_sra_hl);
		lookup_table2[0x2F] = Some(CPU::opcode_sra_a);

		lookup_table2[0x30] = Some(CPU::opcode_swap_b);
		lookup_table2[0x31] = Some(CPU::opcode_swap_c);
		lookup_table2[0x32] = Some(CPU::opcode_swap_d);
		lookup_table2[0x33] = Some(CPU::opcode_swap_e);
		lookup_table2[0x34] = Some(CPU::opcode_swap_h);
		lookup_table2[0x35] = Some(CPU::opcode_swap_l);
		lookup_table2[0x36] = Some(CPU::opcode_swap_hl);
		lookup_table2[0x37] = Some(CPU::opcode_swap_a);
		lookup_table2[0x38] = Some(CPU::opcode_srl_b);
		lookup_table2[0x39] = Some(CPU::opcode_srl_c);
		lookup_table2[0x3A] = Some(CPU::opcode_srl_d);
		lookup_table2[0x3B] = Some(CPU::opcode_srl_e);
		lookup_table2[0x3C] = Some(CPU::opcode_srl_h);
		lookup_table2[0x3D] = Some(CPU::opcode_srl_l);
		lookup_table2[0x3E] = Some(CPU::opcode_srl_hl);
		lookup_table2[0x3F] = Some(CPU::opcode_srl_a);

		lookup_table2[0x40] = Some(CPU::opcode_bit_0_b);
		lookup_table2[0x41] = Some(CPU::opcode_bit_0_c);
		lookup_table2[0x42] = Some(CPU::opcode_bit_0_d);
		lookup_table2[0x43] = Some(CPU::opcode_bit_0_e);
		lookup_table2[0x44] = Some(CPU::opcode_bit_0_h);
		lookup_table2[0x45] = Some(CPU::opcode_bit_0_l);
		lookup_table2[0x46] = Some(CPU::opcode_bit_0_hl);
		lookup_table2[0x47] = Some(CPU::opcode_bit_0_a);
		lookup_table2[0x48] = Some(CPU::opcode_bit_1_b);
		lookup_table2[0x49] = Some(CPU::opcode_bit_1_c);
		lookup_table2[0x4A] = Some(CPU::opcode_bit_1_d);
		lookup_table2[0x4B] = Some(CPU::opcode_bit_1_e);
		lookup_table2[0x4C] = Some(CPU::opcode_bit_1_h);
		lookup_table2[0x4D] = Some(CPU::opcode_bit_1_l);
		lookup_table2[0x4E] = Some(CPU::opcode_bit_1_hl);
		lookup_table2[0x4F] = Some(CPU::opcode_bit_1_a);

		lookup_table2[0x50] = Some(CPU::opcode_bit_2_b);
		lookup_table2[0x51] = Some(CPU::opcode_bit_2_c);
		lookup_table2[0x52] = Some(CPU::opcode_bit_2_d);
		lookup_table2[0x53] = Some(CPU::opcode_bit_2_e);
		lookup_table2[0x54] = Some(CPU::opcode_bit_2_h);
		lookup_table2[0x55] = Some(CPU::opcode_bit_2_l);
		lookup_table2[0x56] = Some(CPU::opcode_bit_2_hl);
		lookup_table2[0x57] = Some(CPU::opcode_bit_2_a);
		lookup_table2[0x58] = Some(CPU::opcode_bit_3_b);
		lookup_table2[0x59] = Some(CPU::opcode_bit_3_c);
		lookup_table2[0x5A] = Some(CPU::opcode_bit_3_d);
		lookup_table2[0x5B] = Some(CPU::opcode_bit_3_e);
		lookup_table2[0x5C] = Some(CPU::opcode_bit_3_h);
		lookup_table2[0x5D] = Some(CPU::opcode_bit_3_l);
		lookup_table2[0x5E] = Some(CPU::opcode_bit_3_hl);
		lookup_table2[0x5F] = Some(CPU::opcode_bit_3_a);

		lookup_table2[0x60] = Some(CPU::opcode_bit_4_b);
		lookup_table2[0x61] = Some(CPU::opcode_bit_4_c);
		lookup_table2[0x62] = Some(CPU::opcode_bit_4_d);
		lookup_table2[0x63] = Some(CPU::opcode_bit_4_e);
		lookup_table2[0x64] = Some(CPU::opcode_bit_4_h);
		lookup_table2[0x65] = Some(CPU::opcode_bit_4_l);
		lookup_table2[0x66] = Some(CPU::opcode_bit_4_hl);
		lookup_table2[0x67] = Some(CPU::opcode_bit_4_a);
		lookup_table2[0x68] = Some(CPU::opcode_bit_5_b);
		lookup_table2[0x69] = Some(CPU::opcode_bit_5_c);
		lookup_table2[0x6A] = Some(CPU::opcode_bit_5_d);
		lookup_table2[0x6B] = Some(CPU::opcode_bit_5_e);
		lookup_table2[0x6C] = Some(CPU::opcode_bit_5_h);
		lookup_table2[0x6D] = Some(CPU::opcode_bit_5_l);
		lookup_table2[0x6E] = Some(CPU::opcode_bit_5_hl);
		lookup_table2[0x6F] = Some(CPU::opcode_bit_5_a);

		lookup_table2[0x70] = Some(CPU::opcode_bit_6_b);
		lookup_table2[0x71] = Some(CPU::opcode_bit_6_c);
		lookup_table2[0x72] = Some(CPU::opcode_bit_6_d);
		lookup_table2[0x73] = Some(CPU::opcode_bit_6_e);
		lookup_table2[0x74] = Some(CPU::opcode_bit_6_h);
		lookup_table2[0x75] = Some(CPU::opcode_bit_6_l);
		lookup_table2[0x76] = Some(CPU::opcode_bit_6_hl);
		lookup_table2[0x77] = Some(CPU::opcode_bit_6_a);
		lookup_table2[0x78] = Some(CPU::opcode_bit_7_b);
		lookup_table2[0x79] = Some(CPU::opcode_bit_7_c);
		lookup_table2[0x7A] = Some(CPU::opcode_bit_7_d);
		lookup_table2[0x7B] = Some(CPU::opcode_bit_7_e);
		lookup_table2[0x7C] = Some(CPU::opcode_bit_7_h);
		lookup_table2[0x7D] = Some(CPU::opcode_bit_7_l);
		lookup_table2[0x7E] = Some(CPU::opcode_bit_7_hl);
		lookup_table2[0x7F] = Some(CPU::opcode_bit_7_a);
			
		lookup_table2[0x80] = Some(CPU::opcode_res_0_b);
		lookup_table2[0x81] = Some(CPU::opcode_res_0_c);
		lookup_table2[0x82] = Some(CPU::opcode_res_0_d);
		lookup_table2[0x83] = Some(CPU::opcode_res_0_e);
		lookup_table2[0x84] = Some(CPU::opcode_res_0_h);
		lookup_table2[0x85] = Some(CPU::opcode_res_0_l);
		lookup_table2[0x86] = Some(CPU::opcode_res_0_hl);
		lookup_table2[0x87] = Some(CPU::opcode_res_0_a);
		lookup_table2[0x88] = Some(CPU::opcode_res_1_b);
		lookup_table2[0x89] = Some(CPU::opcode_res_1_c);
		lookup_table2[0x8A] = Some(CPU::opcode_res_1_d);
		lookup_table2[0x8B] = Some(CPU::opcode_res_1_e);
		lookup_table2[0x8C] = Some(CPU::opcode_res_1_h);
		lookup_table2[0x8D] = Some(CPU::opcode_res_1_l);
		lookup_table2[0x8E] = Some(CPU::opcode_res_1_hl);
		lookup_table2[0x8F] = Some(CPU::opcode_res_1_a);

		lookup_table2[0x90] = Some(CPU::opcode_res_2_b);
		lookup_table2[0x91] = Some(CPU::opcode_res_2_c);
		lookup_table2[0x92] = Some(CPU::opcode_res_2_d);
		lookup_table2[0x93] = Some(CPU::opcode_res_2_e);
		lookup_table2[0x94] = Some(CPU::opcode_res_2_h);
		lookup_table2[0x95] = Some(CPU::opcode_res_2_l);
		lookup_table2[0x96] = Some(CPU::opcode_res_2_hl);
		lookup_table2[0x97] = Some(CPU::opcode_res_2_a);
		lookup_table2[0x98] = Some(CPU::opcode_res_3_b);
		lookup_table2[0x99] = Some(CPU::opcode_res_3_c);
		lookup_table2[0x9A] = Some(CPU::opcode_res_3_d);
		lookup_table2[0x9B] = Some(CPU::opcode_res_3_e);
		lookup_table2[0x9C] = Some(CPU::opcode_res_3_h);
		lookup_table2[0x9D] = Some(CPU::opcode_res_3_l);
		lookup_table2[0x9E] = Some(CPU::opcode_res_3_hl);
		lookup_table2[0x9F] = Some(CPU::opcode_res_3_a);

		lookup_table2[0xA0] = Some(CPU::opcode_res_4_b);
		lookup_table2[0xA1] = Some(CPU::opcode_res_4_c);
		lookup_table2[0xA2] = Some(CPU::opcode_res_4_d);
		lookup_table2[0xA3] = Some(CPU::opcode_res_4_e);
		lookup_table2[0xA4] = Some(CPU::opcode_res_4_h);
		lookup_table2[0xA5] = Some(CPU::opcode_res_4_l);
		lookup_table2[0xA6] = Some(CPU::opcode_res_4_hl);
		lookup_table2[0xA7] = Some(CPU::opcode_res_4_a);
		lookup_table2[0xA8] = Some(CPU::opcode_res_5_b);
		lookup_table2[0xA9] = Some(CPU::opcode_res_5_c);
		lookup_table2[0xAA] = Some(CPU::opcode_res_5_d);
		lookup_table2[0xAB] = Some(CPU::opcode_res_5_e);
		lookup_table2[0xAC] = Some(CPU::opcode_res_5_h);
		lookup_table2[0xAD] = Some(CPU::opcode_res_5_l);
		lookup_table2[0xAE] = Some(CPU::opcode_res_5_hl);
		lookup_table2[0xAF] = Some(CPU::opcode_res_5_a);

		lookup_table2[0xB0] = Some(CPU::opcode_res_6_b);
		lookup_table2[0xB1] = Some(CPU::opcode_res_6_c);
		lookup_table2[0xB2] = Some(CPU::opcode_res_6_d);
		lookup_table2[0xB3] = Some(CPU::opcode_res_6_e);
		lookup_table2[0xB4] = Some(CPU::opcode_res_6_h);
		lookup_table2[0xB5] = Some(CPU::opcode_res_6_l);
		lookup_table2[0xB6] = Some(CPU::opcode_res_6_hl);
		lookup_table2[0xB7] = Some(CPU::opcode_res_6_a);
		lookup_table2[0xB8] = Some(CPU::opcode_res_7_b);
		lookup_table2[0xB9] = Some(CPU::opcode_res_7_c);
		lookup_table2[0xBA] = Some(CPU::opcode_res_7_d);
		lookup_table2[0xBB] = Some(CPU::opcode_res_7_e);
		lookup_table2[0xBC] = Some(CPU::opcode_res_7_h);
		lookup_table2[0xBD] = Some(CPU::opcode_res_7_l);
		lookup_table2[0xBE] = Some(CPU::opcode_res_7_hl);
		lookup_table2[0xBF] = Some(CPU::opcode_res_7_a);

		lookup_table2[0xC0] = Some(CPU::opcode_set_0_b);
		lookup_table2[0xC1] = Some(CPU::opcode_set_0_c);
		lookup_table2[0xC2] = Some(CPU::opcode_set_0_d);
		lookup_table2[0xC3] = Some(CPU::opcode_set_0_e);
		lookup_table2[0xC4] = Some(CPU::opcode_set_0_h);
		lookup_table2[0xC5] = Some(CPU::opcode_set_0_l);
		lookup_table2[0xC6] = Some(CPU::opcode_set_0_hl);
		lookup_table2[0xC7] = Some(CPU::opcode_set_0_a);
		lookup_table2[0xC8] = Some(CPU::opcode_set_1_b);
		lookup_table2[0xC9] = Some(CPU::opcode_set_1_c);
		lookup_table2[0xCA] = Some(CPU::opcode_set_1_d);
		lookup_table2[0xCB] = Some(CPU::opcode_set_1_e);
		lookup_table2[0xCC] = Some(CPU::opcode_set_1_h);
		lookup_table2[0xCD] = Some(CPU::opcode_set_1_l);
		lookup_table2[0xCE] = Some(CPU::opcode_set_1_hl);
		lookup_table2[0xCF] = Some(CPU::opcode_set_1_a);

		lookup_table2[0xD0] = Some(CPU::opcode_set_2_b);
		lookup_table2[0xD1] = Some(CPU::opcode_set_2_c);
		lookup_table2[0xD2] = Some(CPU::opcode_set_2_d);
		lookup_table2[0xD3] = Some(CPU::opcode_set_2_e);
		lookup_table2[0xD4] = Some(CPU::opcode_set_2_h);
		lookup_table2[0xD5] = Some(CPU::opcode_set_2_l);
		lookup_table2[0xD6] = Some(CPU::opcode_set_2_hl);
		lookup_table2[0xD7] = Some(CPU::opcode_set_2_a);
		lookup_table2[0xD8] = Some(CPU::opcode_set_3_b);
		lookup_table2[0xD9] = Some(CPU::opcode_set_3_c);
		lookup_table2[0xDA] = Some(CPU::opcode_set_3_d);
		lookup_table2[0xDB] = Some(CPU::opcode_set_3_e);
		lookup_table2[0xDC] = Some(CPU::opcode_set_3_h);
		lookup_table2[0xDD] = Some(CPU::opcode_set_3_l);
		lookup_table2[0xDE] = Some(CPU::opcode_set_3_hl);
		lookup_table2[0xDF] = Some(CPU::opcode_set_3_a);

		lookup_table2[0xE0] = Some(CPU::opcode_set_4_b);
		lookup_table2[0xE1] = Some(CPU::opcode_set_4_c);
		lookup_table2[0xE2] = Some(CPU::opcode_set_4_d);
		lookup_table2[0xE3] = Some(CPU::opcode_set_4_e);
		lookup_table2[0xE4] = Some(CPU::opcode_set_4_h);
		lookup_table2[0xE5] = Some(CPU::opcode_set_4_l);
		lookup_table2[0xE6] = Some(CPU::opcode_set_4_hl);
		lookup_table2[0xE7] = Some(CPU::opcode_set_4_a);
		lookup_table2[0xE8] = Some(CPU::opcode_set_5_b);
		lookup_table2[0xE9] = Some(CPU::opcode_set_5_c);
		lookup_table2[0xEA] = Some(CPU::opcode_set_5_d);
		lookup_table2[0xEB] = Some(CPU::opcode_set_5_e);
		lookup_table2[0xEC] = Some(CPU::opcode_set_5_h);
		lookup_table2[0xED] = Some(CPU::opcode_set_5_l);
		lookup_table2[0xEE] = Some(CPU::opcode_set_5_hl);
		lookup_table2[0xEF] = Some(CPU::opcode_set_5_a);

		lookup_table2[0xF0] = Some(CPU::opcode_set_6_b);
		lookup_table2[0xF1] = Some(CPU::opcode_set_6_c);
		lookup_table2[0xF2] = Some(CPU::opcode_set_6_d);
		lookup_table2[0xF3] = Some(CPU::opcode_set_6_e);
		lookup_table2[0xF4] = Some(CPU::opcode_set_6_h);
		lookup_table2[0xF5] = Some(CPU::opcode_set_6_l);
		lookup_table2[0xF6] = Some(CPU::opcode_set_6_hl);
		lookup_table2[0xF7] = Some(CPU::opcode_set_6_a);
		lookup_table2[0xF8] = Some(CPU::opcode_set_7_b);
		lookup_table2[0xF9] = Some(CPU::opcode_set_7_c);
		lookup_table2[0xFA] = Some(CPU::opcode_set_7_d);
		lookup_table2[0xFB] = Some(CPU::opcode_set_7_e);
		lookup_table2[0xFC] = Some(CPU::opcode_set_7_h);
		lookup_table2[0xFD] = Some(CPU::opcode_set_7_l);
		lookup_table2[0xFE] = Some(CPU::opcode_set_7_hl);
		lookup_table2[0xFF] = Some(CPU::opcode_set_7_a);

		self.lookup_table2 = lookup_table2;
	}
	
	// LD r, r': Load register (register)
	fn opcode_ld_r_r(&mut self, _mmu: &mut MMU, dest: char, src: char) {
		let dest_register = self.r_index(dest);
		let src_register = self.r_index(src);		
		self.cpu_registers[dest_register] = self.cpu_registers[src_register];
		self.mcycles = 1;
	}
	// LD A, A
	fn opcode_ld_a_a(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'A', 'A');}
	// LD A, B
	fn opcode_ld_a_b(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'A', 'B');}
	// LD A, C
	fn opcode_ld_a_c(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'A', 'C');}
	// LD A, D
	fn opcode_ld_a_d(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'A', 'D');}
	// LD A, E
	fn opcode_ld_a_e(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'A', 'E');}
	// LD A, H
	fn opcode_ld_a_h(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'A', 'H');}
	// LD A, L
	fn opcode_ld_a_l(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'A', 'L');}
	
	// LD B, A
	fn opcode_ld_b_a(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'B', 'A');}
	// LD B, B
	fn opcode_ld_b_b(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'B', 'B');}
	// LD B, C
	fn opcode_ld_b_c(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'B', 'C');}
	// LD B, D
	fn opcode_ld_b_d(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'B', 'D');}
	// LD B, E
	fn opcode_ld_b_e(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'B', 'E');}
	// LD B, H
	fn opcode_ld_b_h(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'B', 'H');}
	// LD B, L
	fn opcode_ld_b_l(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'B', 'L');}
	
	// LD C, A
	fn opcode_ld_c_a(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'C', 'A');}
	// LD C, B
	fn opcode_ld_c_b(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'C', 'B');}
	// LD C, C
	fn opcode_ld_c_c(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'C', 'C');}
	// LD C, D
	fn opcode_ld_c_d(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'C', 'D');}
	// LD C, E
	fn opcode_ld_c_e(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'C', 'E');}
	// LD C, H
	fn opcode_ld_c_h(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'C', 'H');}
	// LD C, L
	fn opcode_ld_c_l(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'C', 'L');}

	// LD D, A
	fn opcode_ld_d_a(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'D', 'A');}
	// LD D, B
	fn opcode_ld_d_b(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'D', 'B');}
	// LD D, C
	fn opcode_ld_d_c(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'D', 'C');}
	// LD D, D
	fn opcode_ld_d_d(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'D', 'D');}
	// LD D, E
	fn opcode_ld_d_e(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'D', 'E');}
	// LD D, H
	fn opcode_ld_d_h(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'D', 'H');}
	// LD D, L
	fn opcode_ld_d_l(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'D', 'L');}

	// LD E, A
	fn opcode_ld_e_a(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'E', 'A');}
	// LD E, B
	fn opcode_ld_e_b(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'E', 'B');}
	// LD E, C
	fn opcode_ld_e_c(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'E', 'C');}
	// LD E, D
	fn opcode_ld_e_d(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'E', 'D');}
	// LD E, E
	fn opcode_ld_e_e(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'E', 'E');}
	// LD E, H
	fn opcode_ld_e_h(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'E', 'H');}
	// LD E, L
	fn opcode_ld_e_l(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'E', 'L');}

	// LD H, A
	fn opcode_ld_h_a(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'H', 'A');}
	// LD H, B
	fn opcode_ld_h_b(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'H', 'B');}
	// LD H, C
	fn opcode_ld_h_c(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'H', 'C');}
	// LD H, D
	fn opcode_ld_h_d(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'H', 'D');}
	// LD H, E
	fn opcode_ld_h_e(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'H', 'E');}
	// LD H, H
	fn opcode_ld_h_h(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'H', 'H');}
	// LD H, L
	fn opcode_ld_h_l(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'H', 'L');}
	
	// LD L, A
	fn opcode_ld_l_a(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'L', 'A');}
	// LD L, B
	fn opcode_ld_l_b(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'L', 'B');}
	// LD L, C
	fn opcode_ld_l_c(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'L', 'C');}
	// LD L, D
	fn opcode_ld_l_d(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'L', 'D');}
	// LD L, E
	fn opcode_ld_l_e(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'L', 'E');}
	// LD L, H
	fn opcode_ld_l_h(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'L', 'H');}
	// LD L, L
	fn opcode_ld_l_l(&mut self, mmu: &mut MMU) {self.opcode_ld_r_r(mmu, 'L', 'L');}


	// LD r, n: Load register (immediate)
	fn opcode_ld_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let dest_register = self.r_index(r);
		self.cpu_registers[dest_register] = n;
		self.mcycles = 2;
	}
	// LD A, n
	fn opcode_ld_a_n(&mut self, mmu: &mut MMU) {self.opcode_ld_r_n(mmu, 'A');}
	// LD B, n
	fn opcode_ld_b_n(&mut self, mmu: &mut MMU) {self.opcode_ld_r_n(mmu, 'B');}
	// LD C, n
	fn opcode_ld_c_n(&mut self, mmu: &mut MMU) {self.opcode_ld_r_n(mmu, 'C');}
	// LD D, n
	fn opcode_ld_d_n(&mut self, mmu: &mut MMU) {self.opcode_ld_r_n(mmu, 'D');}
	// LD E, n
	fn opcode_ld_e_n(&mut self, mmu: &mut MMU) {self.opcode_ld_r_n(mmu, 'E');}
	// LD H, n
	fn opcode_ld_h_n(&mut self, mmu: &mut MMU) {self.opcode_ld_r_n(mmu, 'H');}
	// LD L, n
	fn opcode_ld_l_n(&mut self, mmu: &mut MMU) {self.opcode_ld_r_n(mmu, 'L');}

	// LD r, m: Load register from memory pointed to by double register
	fn opcode_ld_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.cpu_registers[r_reg] = mmu.get_byte(mem);
		self.mcycles = 2;
	}
	// LD A, (BC)
	fn opcode_ld_a_bc(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'A', "BC");}
	// LD A, (DE)
	fn opcode_ld_a_de(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'A', "DE");}
	// LD A, (HL)
	fn opcode_ld_a_hl(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'A', "HL");}
	// LD B, (HL)
	fn opcode_ld_b_hl(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'B', "HL");}
	// LD C, (HL)
	fn opcode_ld_c_hl(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'C', "HL");}
	// LD D, (HL)
	fn opcode_ld_d_hl(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'D', "HL");}
	// LD E, (HL)
	fn opcode_ld_e_hl(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'E', "HL");}
	// LD H, (HL)
	fn opcode_ld_h_hl(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'H', "HL");}
	// LD L, (HL)
	fn opcode_ld_l_hl(&mut self, mmu: &mut MMU) {self.opcode_ld_r_m(mmu, 'L', "HL");}

	// LD m, r: Load memory pointed by double register from register
	fn opcode_ld_m_r(&mut self, mmu: &mut MMU, double_reg: &str, r: char) {
		let src_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		mmu.set_byte(mem as u16, self.cpu_registers[src_register]);
		self.mcycles = 2;
	}
	// LD (BC), A
	fn opcode_ld_bc_a(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "BC", 'A');}
	// LD (DE), A
	fn opcode_ld_de_a(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "DE", 'A');}
	// LD (HL), A
	fn opcode_ld_hl_a(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "HL", 'A');}
	// LD (HL), B
	fn opcode_ld_hl_b(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "HL", 'B');}
	// LD (HL), C
	fn opcode_ld_hl_c(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "HL", 'C');}
	// LD (HL), D
	fn opcode_ld_hl_d(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "HL", 'D');}
	// LD (HL), E
	fn opcode_ld_hl_e(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "HL", 'E');}
	// LD (HL), H
	fn opcode_ld_hl_h(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "HL", 'H');}
	// LD (HL), L
	fn opcode_ld_hl_l(&mut self, mmu: &mut MMU) {self.opcode_ld_m_r(mmu, "HL", 'L');}

	// LD m, n: Load memory pointed by double register from immediate
	fn opcode_ld_m_n(&mut self, mmu: &mut MMU, double_reg: &str) {
		let n = self.fetch_byte(mmu);
		let mem = self.double_register_value(double_reg);
		mmu.set_byte(mem, n);
		self.mcycles = 3;
	}
	// LD (HL), n
	fn opcode_ld_hl_n(&mut self, mmu: &mut MMU) {self.opcode_ld_m_n(mmu, "HL");}

	// Load r, nn: Load register from memory pointed by 16-bit immediate
	fn opcode_ld_r_nn(&mut self, mmu: &mut MMU, r: char) {
		let nn = self.fetch_word(mmu);
		let dest_register = self.r_index(r);
		self.cpu_registers[dest_register] = mmu.get_byte(nn);
		self.mcycles = 4;
	}
	// LD A, nn
	fn opcode_ld_a_nn(&mut self, mmu: &mut MMU) {
		self.opcode_ld_r_nn(mmu, 'A');
	}

	// Load nn, r: Load memory pointed by 16-bit immediate from register
	fn opcode_ld_nn_r(&mut self, mmu: &mut MMU, r: char) {
		let nn = self.fetch_word(mmu);
		let src_register = self.r_index(r);
		mmu.set_byte(nn, self.cpu_registers[src_register]);
		self.mcycles = 4;
	}
	// LD nn, A
	fn opcode_ld_nn_a(&mut self, mmu: &mut MMU) {self.opcode_ld_nn_r(mmu, 'A');}

	// LoadHigh r, n: Load register from memory pointed by 0xFF00 + n
	fn opcode_ldh_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let mem = 0xFF00 | (n as u16);
		let dest_register = self.r_index(r);
		let value = mmu.get_byte(mem);
		self.cpu_registers[dest_register] = value;
		if mem == 0xFF44 {
			self.cpu_registers[dest_register] = 0x90;
		}
		self.mcycles = 3;
	}
	// LDH A, (FF00+n)
	fn opcode_ldh_a_n(&mut self, mmu: &mut MMU) {
		self.opcode_ldh_r_n(mmu, 'A');
		// TODO change it back
		// self.cpu_registers[0] = 0x90;
	}

	// LoadHigh n, r: Load memory pointed by 0xFF00 + n from register
	// TODO: Change back
	fn opcode_ldh_n_r(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let mem = 0xFF00 | (n as u16);
		let src_register = self.r_index(r);
		mmu.set_byte(mem, self.cpu_registers[src_register]);
		self.mcycles = 3;
	}
	// LDH (FF00+n), A
	fn opcode_ldh_n_a(&mut self, mmu: &mut MMU) {
		self.opcode_ldh_n_r(mmu, 'A');
	}

	// LoadHigh r, m: Load register from memory pointed by 0xFF00+register
	fn opcode_ldh_r_m(&mut self, mmu: &mut MMU, r1: char, r2: char) {
		let dest_register = self.r_index(r1);
		let src_register = self.r_index(r2);
		let mem = 0xFF00 | (self.cpu_registers[src_register] as u16);
		self.cpu_registers[dest_register] = mmu.get_byte(mem);
		self.mcycles = 2;
	}
	// LDH A, (C)
	fn opcode_ldh_a_c(&mut self, mmu: &mut MMU) {
		self.opcode_ldh_r_m(mmu, 'A', 'C');
	}

	// LoadHigh m, r: Load memory pointed by 0xFF00 + register from register
	fn opcode_ldh_m_r(&mut self, mmu: &mut MMU, r1: char, r2: char) {
		let r1_idx = self.r_index(r1);
		let r2_idx = self.r_index(r2);
		let mem = 0xFF00 | (self.cpu_registers[r1_idx] as u16);
		mmu.set_byte(mem, self.cpu_registers[r2_idx]);
		self.mcycles = 2;
	}
	// LDH (C), A
	fn opcode_ldh_c_a(&mut self, mmu: &mut MMU) {
		self.opcode_ldh_m_r(mmu, 'C', 'A');
	}

	// LoadIncrement m, r: Load memory pointed by double register from register,
	// then increment double register
	fn opcode_ldi_m_r(&mut self, mmu: &mut MMU, double_reg: &str, r: char) {
		let src_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		mmu.set_byte(mem, self.cpu_registers[src_register]);
		self.set_double_register(double_reg, (mem+1) as u16);
		self.mcycles = 2;
	}
	//	LDI HL, A
	fn opcode_ldi_hl_a(&mut self, mmu: &mut MMU) {self.opcode_ldi_m_r(mmu, "HL", 'A');}

	// LoadIncrement r, m: Load register from memory pointed by double register,
	// then increment double register
	fn opcode_ldi_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let dest_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.cpu_registers[dest_register] = mmu.get_byte(mem);
		self.set_double_register(double_reg, (mem+1) as u16);
		self.mcycles = 2;
	}
	// LDI A, HL
	fn opcode_ldi_a_hl(&mut self, mmu: &mut MMU) {self.opcode_ldi_r_m(mmu, 'A', "HL");}

	// LoadDecrement m, r: Load memory pointed by double register from register,
	// then decrement double register
	fn opcode_ldd_m_r(&mut self, mmu: &mut MMU, double_reg: &str, r: char) {
		let src_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		mmu.set_byte(mem, self.cpu_registers[src_register]);
		self.set_double_register(double_reg, (mem-1) as u16);
		self.mcycles = 2;
	}
	// LDD HL, A
	fn opcode_ldd_hl_a(&mut self, mmu: &mut MMU) {self.opcode_ldd_m_r(mmu, "HL", 'A');}

	// LoadDecrement r, m: Load register from memory pointed by double register,
	// then decrement double register
	fn opcode_ldd_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let dest_register = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		self.cpu_registers[dest_register] = mmu.get_byte(mem);
		self.set_double_register(double_reg, (mem-1) as u16);
		self.mcycles = 2;
	}
	// LDD A, HL
	fn opcode_ldd_a_hl(&mut self, mmu: &mut MMU) {self.opcode_ldd_r_m(mmu, 'A', "HL")}

	// LD rr, nn: Load double register from 16-bit immediate
	fn opcode_ld_rr_nn(&mut self, mmu: &mut MMU, double_reg: &str) {
		let nn = self.fetch_word(mmu);
		self.set_double_register(double_reg, nn);
		self.mcycles = 3;
	}
	// LD BC, nn 
	fn opcode_ld_bc_nn(&mut self, mmu: &mut MMU) {self.opcode_ld_rr_nn(mmu, "BC");}
	// LD DE, nn 
	fn opcode_ld_de_nn(&mut self, mmu: &mut MMU) {self.opcode_ld_rr_nn(mmu, "DE");}
	// LD HL, nn
	fn opcode_ld_hl_nn(&mut self, mmu: &mut MMU) {self.opcode_ld_rr_nn(mmu, "HL");}
	// LD SP, nn 
	fn opcode_ld_sp_nn(&mut self, mmu: &mut MMU) {self.opcode_ld_rr_nn(mmu, "SP");}

	// Load nn, rr: Load double to memory pointed from double register
	fn opcode_ld_nn_rr(&mut self, mmu: &mut MMU, double_reg: &str) {
		let nn = self.fetch_word(mmu);
		let value = self.double_register_value(double_reg);
		mmu.set_byte(nn, (value & 0xFF) as u8);
		mmu.set_byte(nn + 1, ((value & 0xFF00) >> 8) as u8);
		self.mcycles = 5;
	}
	// LD (a16), SP
	fn opcode_ld_nn_sp(&mut self, mmu: &mut MMU) {self.opcode_ld_nn_rr(mmu, "SP");}

	// Load rr, rr': Load double register from double register
	fn opcode_ld_rr_rr(&mut self, _mmu: &mut MMU, double_reg1: &str, double_reg2: &str) {
		let value = self.double_register_value(double_reg2);
		self.set_double_register(double_reg1, value as u16);
		self.mcycles = 2;
	}
	// LD SP, HL
	fn opcode_ld_sp_hl(&mut self, mmu: &mut MMU) {self.opcode_ld_rr_rr(mmu, "SP", "HL");}

	// Push rr: Push the value of a double register to the stack
	fn opcode_push_rr(&mut self, mmu: &mut MMU, double_reg: &str) {
		let value = self.double_register_value(double_reg);
		self.push_stack(mmu, value);
		self.mcycles = 4;
	}
	// PUSH AF
	fn opcode_push_af(&mut self, mmu: &mut MMU) {self.opcode_push_rr(mmu, "AF");}
	// PUSH BC
	fn opcode_push_bc(&mut self, mmu: &mut MMU) {self.opcode_push_rr(mmu, "BC");}
	// PUSH DE
	fn opcode_push_de(&mut self, mmu: &mut MMU) {self.opcode_push_rr(mmu, "DE");}
	// PUSH HL
	fn opcode_push_hl(&mut self, mmu: &mut MMU) {self.opcode_push_rr(mmu, "HL");}

	// Pop rr: Pop the stack and store its value in a double register
	fn opcode_pop_rr(&mut self, mmu: &mut MMU, double_reg: &str) {
		let mut value = self.pop_stack(mmu);
		// The first 4 bits of F must be set to 0 
		if double_reg == "AF" {
			value &= 0xFFF0;
		}
		self.set_double_register(double_reg, value);
		self.mcycles = 3;
	}
	// POP AF
	fn opcode_pop_af(&mut self, mmu: &mut MMU) {self.opcode_pop_rr(mmu, "AF");}
	// POP BC
	fn opcode_pop_bc(&mut self, mmu: &mut MMU) {self.opcode_pop_rr(mmu, "BC");}
	// POP DE
	fn opcode_pop_de(&mut self, mmu: &mut MMU) {self.opcode_pop_rr(mmu, "DE");}
	// POP HL
	fn opcode_pop_hl(&mut self, mmu: &mut MMU) {self.opcode_pop_rr(mmu, "HL");}

	// Add r, r: Add register to register
	fn opcode_add_r_r(&mut self, _mmu: &mut MMU, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let (result, overflow) = a.overflowing_add(b);
		let hc = (((a & 0xF) + (b & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r1_reg] = result;

		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 1;
	}
	// ADD A, A
	fn opcode_add_a_a(&mut self, mmu: &mut MMU) {self.opcode_add_r_r(mmu, 'A', 'A');}
	// ADD A, B
	fn opcode_add_a_b(&mut self, mmu: &mut MMU) {self.opcode_add_r_r(mmu, 'A', 'B');}
	// ADD A, C
	fn opcode_add_a_c(&mut self, mmu: &mut MMU) {self.opcode_add_r_r(mmu, 'A', 'C');}
	// ADD A, D
	fn opcode_add_a_d(&mut self, mmu: &mut MMU) {self.opcode_add_r_r(mmu, 'A', 'D');}
	// ADD A, E
	fn opcode_add_a_e(&mut self, mmu: &mut MMU) {self.opcode_add_r_r(mmu, 'A', 'E');}
	// ADD A, H
	fn opcode_add_a_h(&mut self, mmu: &mut MMU) {self.opcode_add_r_r(mmu, 'A', 'H');}
	// ADD A, L
	fn opcode_add_a_l(&mut self, mmu: &mut MMU) {self.opcode_add_r_r(mmu, 'A', 'L');}

	// Add r, n: Add immediate to register
	fn opcode_add_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let r_reg = self.r_index(r);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_add(n);
		let hc = (((self.cpu_registers[r_reg] & 0xF) + (n & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r_reg] = result;

		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	fn opcode_add_a_n(&mut self, mmu: &mut MMU) {self.opcode_add_r_n(mmu, 'A');}

	// Add r, m : Add from memory pointed by dobule register to register
	fn opcode_add_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let a = self.cpu_registers[r_reg];
		let b = mmu.get_byte(mem);
		let (result, overflow) = a.overflowing_add(b);
		let hc = (((a & 0xF) + (b & 0xF)) & 0x10) >> 4;
		
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// ADD A, (HL)
	fn opcode_add_a_hl(&mut self, mmu: &mut MMU) {self.opcode_add_r_m(mmu, 'A', "HL");}

	// Adc r, r: Add with carry from register to register
	fn opcode_adc_r_r(&mut self, _mmu: &mut MMU, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_add(b);
		let (result2, overflow2) = result1.overflowing_add(carry);
		let hc = (((a & 0xF) + (b & 0xF) + (carry & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r1_reg] = result2;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 1;
	}
	// ADC A, A
	fn opcode_adc_a_a(&mut self, mmu: &mut MMU) {self.opcode_adc_r_r(mmu, 'A', 'A');}
	// ADC A, B
	fn opcode_adc_a_b(&mut self, mmu: &mut MMU) {self.opcode_adc_r_r(mmu, 'A', 'B');}
	// ADC A, C
	fn opcode_adc_a_c(&mut self, mmu: &mut MMU) {self.opcode_adc_r_r(mmu, 'A', 'C');}
	// ADC A, D
	fn opcode_adc_a_d(&mut self, mmu: &mut MMU) {self.opcode_adc_r_r(mmu, 'A', 'D');}
	// ADC A, E
	fn opcode_adc_a_e(&mut self, mmu: &mut MMU) {self.opcode_adc_r_r(mmu, 'A', 'E');}
	// ADC A, H
	fn opcode_adc_a_h(&mut self, mmu: &mut MMU) {self.opcode_adc_r_r(mmu, 'A', 'H');}
	// ADC A, L
	fn opcode_adc_a_l(&mut self, mmu: &mut MMU) {self.opcode_adc_r_r(mmu, 'A', 'L');}

	// Adc r, n: Add with carry from immediate to register
	fn opcode_adc_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let r_reg = self.r_index(r);
		let a = self.cpu_registers[r_reg];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_add(n);
		let (result2, overflow2) = result1.overflowing_add(carry);
		let hc = (((a & 0xF) + (n & 0xF) + (carry & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r_reg] = result2;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// ADC a, n
	fn opcode_adc_a_n(&mut self, mmu: &mut MMU) {self.opcode_adc_r_n(mmu, 'A');}
	

	// Adc r, m: Add with carry from memory to register
	fn opcode_adc_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let a = self.cpu_registers[r_reg];
		let b = mmu.get_byte(mem);
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_add(b);
		let (result2, overflow2) = result1.overflowing_add(carry);
		let hc = (((a & 0xF) + (b & 0xF) + (carry & 0xF)) & 0x10) >> 4;

		self.cpu_registers[r_reg] = result2;
		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// ADC A, (HL)
	fn opcode_adc_a_hl(&mut self, mmu: &mut MMU) {self.opcode_adc_r_m(mmu, 'A', "HL");}

	// SUB r, r: Subtract register from register
	fn opcode_sub_r_r(&mut self, _mmu: &mut MMU, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let (result, overflow) = a.overflowing_sub(b);
		let hc = (((a & 0xF).wrapping_sub(b & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r1_reg] = result;

		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 1;
	}
	// SUB A, A
	fn opcode_sub_a_a(&mut self, mmu: &mut MMU) {self.opcode_sub_r_r(mmu, 'A', 'A');}
	// SUB A, B
	fn opcode_sub_a_b(&mut self, mmu: &mut MMU) {self.opcode_sub_r_r(mmu, 'A', 'B');}
	// SUB A, C
	fn opcode_sub_a_c(&mut self, mmu: &mut MMU) {self.opcode_sub_r_r(mmu, 'A', 'C');}
	// SUB A, D
	fn opcode_sub_a_d(&mut self, mmu: &mut MMU) {self.opcode_sub_r_r(mmu, 'A', 'D');}
	// SUB A, E
	fn opcode_sub_a_e(&mut self, mmu: &mut MMU) {self.opcode_sub_r_r(mmu, 'A', 'E');}
	// SUB A, H
	fn opcode_sub_a_h(&mut self, mmu: &mut MMU) {self.opcode_sub_r_r(mmu, 'A', 'H');}
	// SUB A, L
	fn opcode_sub_a_l(&mut self, mmu: &mut MMU) {self.opcode_sub_r_r(mmu, 'A', 'L');}

	// SUB r, n: Subtract immediate from register
	fn opcode_sub_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let r_reg = self.r_index(r);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_sub(n);
		let hc = (((self.cpu_registers[r_reg] & 0xF).wrapping_sub(n & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r_reg] = result;

		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// SUB A, n
	fn opcode_sub_a_n(&mut self, mmu: &mut MMU) {self.opcode_sub_r_n(mmu, 'A');}

	// SUB r, m : Subtract value pointed by dobule register from register
	fn opcode_sub_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let a = self.cpu_registers[r_reg];
		let b = mmu.get_byte(mem);
		let (result, overflow) = a.overflowing_sub(b);
		let hc = (((a & 0xF).wrapping_sub(b & 0xF)) & 0x10) >> 4;
				  
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// SUB A, (HL)
	fn opcode_sub_a_hl(&mut self, mmu: &mut MMU) {self.opcode_sub_r_m(mmu, 'A', "HL");}

	// SBC r, r: Subtract with carry, register from register
	fn opcode_sbc_r_r(&mut self, _mmu: &mut MMU, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_sub(b);
		let (result2, overflow2) = result1.overflowing_sub(carry);
		let hc = (((a & 0xF).wrapping_sub(b & 0xF).wrapping_sub(carry & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r1_reg] = result2;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 1
	}
	// SBC A, A
	fn opcode_sbc_a_a(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_r(mmu, 'A', 'A');}
	// SBC A, B
	fn opcode_sbc_a_b(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_r(mmu, 'A', 'B');}
	// SBC A, C
	fn opcode_sbc_a_c(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_r(mmu, 'A', 'C');}
	// SBC A, D
	fn opcode_sbc_a_d(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_r(mmu, 'A', 'D');}
	// SBC A, E
	fn opcode_sbc_a_e(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_r(mmu, 'A', 'E');}
	// SBC A, H
	fn opcode_sbc_a_h(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_r(mmu, 'A', 'H');}
	// SBC A, L
	fn opcode_sbc_a_l(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_r(mmu, 'A', 'L');}

	// SBC r, n: Subtract with carry immediate from register
	fn opcode_sbc_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let r_reg = self.r_index(r);
		let a = self.cpu_registers[r_reg];
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_sub(n);
		let (result2, overflow2) = result1.overflowing_sub(carry);
		let hc = (((a & 0xF).wrapping_sub(n & 0xF).wrapping_sub(carry & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r_reg] = result2;

		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// SBC A, n
	fn opcode_sbc_a_n(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_n(mmu, 'A');}

	// SBC r, m: Subtract with carry, memory from register
	fn opcode_sbc_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let a = self.cpu_registers[r_reg];
		let b = mmu.get_byte(mem);
		let carry = self.get_flag('c');
		let (result1, overflow1) = a.overflowing_sub(b);
		let (result2, overflow2) = result1.overflowing_sub(carry);
		let hc = (((a & 0xF).wrapping_sub(b & 0xF).wrapping_sub(carry & 0xF)) & 0x10) >> 4;

		self.cpu_registers[r_reg] = result2;
		if result2 == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow1 == true || overflow2 == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// SBC A, (HL)
	fn opcode_sbc_a_hl(&mut self, mmu: &mut MMU) {self.opcode_sbc_r_m(mmu, 'A', "HL");}


	// AND r, r: AND register with register
	fn opcode_and_r_r(&mut self, _mmu: &mut MMU, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let result = self.cpu_registers[r1_reg] & self.cpu_registers[r2_reg];
		self.cpu_registers[r1_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
		self.set_flag('c', 0);
		self.mcycles = 1;
	}
	// AND A, A
	fn opcode_and_a_a(&mut self, mmu: &mut MMU) {self.opcode_and_r_r(mmu, 'A', 'A');}
	// AND A, B
	fn opcode_and_a_b(&mut self, mmu: &mut MMU) {self.opcode_and_r_r(mmu, 'A', 'B');}
	// AND A, C
	fn opcode_and_a_c(&mut self, mmu: &mut MMU) {self.opcode_and_r_r(mmu, 'A', 'C');}
	// AND A, D
	fn opcode_and_a_d(&mut self, mmu: &mut MMU) {self.opcode_and_r_r(mmu, 'A', 'D');}
	// AND A, E
	fn opcode_and_a_e(&mut self, mmu: &mut MMU) {self.opcode_and_r_r(mmu, 'A', 'E');}
	// AND A, H
	fn opcode_and_a_h(&mut self, mmu: &mut MMU) {self.opcode_and_r_r(mmu, 'A', 'H');}
	// AND A, L
	fn opcode_and_a_l(&mut self, mmu: &mut MMU) {self.opcode_and_r_r(mmu, 'A', 'L');}

	// AND r, n: AND register with immediate
	fn opcode_and_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let r_reg = self.r_index(r);
		let result = self.cpu_registers[r_reg] & n;
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
		self.set_flag('c', 0);
		self.mcycles = 2;
	}
	// AND A, n
	fn opcode_and_a_n(&mut self, mmu: &mut MMU) {self.opcode_and_r_n(mmu, 'A');}

	// AND r, m: AND register with memory pointed by double register
	fn opcode_and_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let result = self.cpu_registers[r_reg] & mmu.get_byte(mem);
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
		self.set_flag('c', 0);
	}
	// AND A, (HL)
	fn opcode_and_a_hl(&mut self, mmu: &mut MMU) {self.opcode_and_r_m(mmu, 'A', "HL");}

	// XOR r, r: XOR register with register
	fn opcode_xor_r_r(&mut self, _mmu: &mut MMU, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let result = self.cpu_registers[r1_reg] ^ self.cpu_registers[r2_reg];
		self.cpu_registers[r1_reg] = result;
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
		self.mcycles = 1;
	}
	// XOR A, A
	fn opcode_xor_a_a(&mut self, mmu: &mut MMU) {self.opcode_xor_r_r(mmu, 'A', 'A');}
	// XOR A, B
	fn opcode_xor_a_b(&mut self, mmu: &mut MMU) {self.opcode_xor_r_r(mmu, 'A', 'B');}
	// XOR A, C
	fn opcode_xor_a_c(&mut self, mmu: &mut MMU) {self.opcode_xor_r_r(mmu, 'A', 'C');}
	// XOR A, D
	fn opcode_xor_a_d(&mut self, mmu: &mut MMU) {self.opcode_xor_r_r(mmu, 'A', 'D');}
	// XOR A, E
	fn opcode_xor_a_e(&mut self, mmu: &mut MMU) {self.opcode_xor_r_r(mmu, 'A', 'E');}
	// XOR A, H
	fn opcode_xor_a_h(&mut self, mmu: &mut MMU) {self.opcode_xor_r_r(mmu, 'A', 'H');}
	// XOR A, L
	fn opcode_xor_a_l(&mut self, mmu: &mut MMU) {self.opcode_xor_r_r(mmu, 'A', 'L');}

	// XOR r, n: XOR register with immediate
	fn opcode_xor_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let r_reg = self.r_index(r);
		let result = self.cpu_registers[r_reg] ^ n;
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
		self.mcycles = 2;
	}
	// XOR A, n
	fn opcode_xor_a_n(&mut self, mmu: &mut MMU) {self.opcode_xor_r_n(mmu, 'A');}

	// XOR r, m: XOR register with memory pointed by double register
	fn opcode_xor_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let result = self.cpu_registers[r_reg] ^ mmu.get_byte(mem);
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
		self.mcycles = 2;
	}
	// XOR A, HL
	fn opcode_xor_a_hl(&mut self, mmu: &mut MMU) {self.opcode_xor_r_m(mmu, 'A', "HL");}

	// OR r, r: OR register with register
	fn opcode_or_r_r(&mut self, _mmu: &mut MMU, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let result = self.cpu_registers[r1_reg] | self.cpu_registers[r2_reg];
		self.cpu_registers[r1_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
		self.mcycles = 1;
	}
	// OR A, A
	fn opcode_or_a_a(&mut self, mmu: &mut MMU) {self.opcode_or_r_r(mmu, 'A', 'A');}
	// OR A, B
	fn opcode_or_a_b(&mut self, mmu: &mut MMU) {self.opcode_or_r_r(mmu, 'A', 'B');}
	// OR A, C
	fn opcode_or_a_c(&mut self, mmu: &mut MMU) {self.opcode_or_r_r(mmu, 'A', 'C');}
	// OR A, D
	fn opcode_or_a_d(&mut self, mmu: &mut MMU) {self.opcode_or_r_r(mmu, 'A', 'D');}
	// OR A, E
	fn opcode_or_a_e(&mut self, mmu: &mut MMU) {self.opcode_or_r_r(mmu, 'A', 'E');}
	// OR A, H
	fn opcode_or_a_h(&mut self, mmu: &mut MMU) {self.opcode_or_r_r(mmu, 'A', 'H');}
	// OR A, L
	fn opcode_or_a_l(&mut self, mmu: &mut MMU) {self.opcode_or_r_r(mmu, 'A', 'L');}

	// OR r, n: OR register with immediate
	fn opcode_or_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let r_reg = self.r_index(r);
		let result = self.cpu_registers[r_reg] | n;
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
		self.mcycles = 2;
	}
	// OR A, n
	fn opcode_or_a_n(&mut self, mmu: &mut MMU) {self.opcode_or_r_n(mmu, 'A');}

	// OR r, m: OR register with memory pointed by double register
	fn opcode_or_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let result = self.cpu_registers[r_reg] | mmu.get_byte(mem);
		self.cpu_registers[r_reg] = result;
		if result == 0 {
			self.set_flag('z', 1);
		} else {    
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
		self.mcycles = 2;
	}
	// OR A (HL)
	fn opcode_or_a_hl(&mut self, mmu: &mut MMU) {self.opcode_or_r_m(mmu, 'A', "HL");}

	// CP r, r: Compare register with register
	fn opcode_cp_r_r(&mut self, _mmu: &mut MMU, r1: char, r2: char) {
		let r1_reg = self.r_index(r1);
		let r2_reg = self.r_index(r2);
		let a = self.cpu_registers[r1_reg];
		let b = self.cpu_registers[r2_reg];
		let (result, overflow) = a.overflowing_sub(b);
		let hc = (((a & 0xF).wrapping_sub(b & 0xF)) & 0x10) >> 4;
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 1;
	}
	// CP A B
	fn opcode_cp_a_a(&mut self, mmu: &mut MMU) {self.opcode_cp_r_r(mmu, 'A', 'A');}
	// CP A B
	fn opcode_cp_a_b(&mut self, mmu: &mut MMU) {self.opcode_cp_r_r(mmu, 'A', 'B');}
	// CP A C
	fn opcode_cp_a_c(&mut self, mmu: &mut MMU) {self.opcode_cp_r_r(mmu, 'A', 'C');}
	// CP A D
	fn opcode_cp_a_d(&mut self, mmu: &mut MMU) {self.opcode_cp_r_r(mmu, 'A', 'D');}
	// CP A E
	fn opcode_cp_a_e(&mut self, mmu: &mut MMU) {self.opcode_cp_r_r(mmu, 'A', 'E');}
	// CP A H
	fn opcode_cp_a_h(&mut self, mmu: &mut MMU) {self.opcode_cp_r_r(mmu, 'A', 'H');}
	// CP A L
	fn opcode_cp_a_l(&mut self, mmu: &mut MMU) {self.opcode_cp_r_r(mmu, 'A', 'L');}

	// CP r, n: Compare register with immediate
	fn opcode_cp_r_n(&mut self, mmu: &mut MMU, r: char) {
		let n = self.fetch_byte(mmu);
		let r_reg = self.r_index(r);
		let (result, overflow) = self.cpu_registers[r_reg].overflowing_sub(n);

		let hc = (((self.cpu_registers[r_reg] & 0xF).wrapping_sub(n & 0xF)) & 0x10) >> 4;
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// CP A, n
	fn opcode_cp_a_n(&mut self, mmu: &mut MMU) {self.opcode_cp_r_n(mmu, 'A');}

	// CP r, m : Compare register with value pointed by double register
	fn opcode_cp_r_m(&mut self, mmu: &mut MMU, r: char, double_reg: &str) {
		let r_reg = self.r_index(r);
		let mem = self.double_register_value(double_reg);
		let a = self.cpu_registers[r_reg];
		let b = mmu.get_byte(mem);
		let (result, overflow) = a.overflowing_sub(b);
		let hc = (((a & 0xF).wrapping_sub(b & 0xF)) & 0x10) >> 4;
	
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
	self.mcycles = 2;
	}
	// CP A, (HL)
	fn opcode_cp_a_hl(&mut self, mmu: &mut MMU) {self.opcode_cp_r_m(mmu, 'A', "HL");}


	// INC r: Increment register
	fn opcode_inc_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_reg = self.r_index(r);
		let (result, _) = self.cpu_registers[r_reg].overflowing_add(1);
		let hc = (((self.cpu_registers[r_reg] & 0xF) + (1 & 0xF)) & 0x10) >> 4;
		self.cpu_registers[r_reg] = result;

		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		self.mcycles = 1;
	}
	fn opcode_inc_a(&mut self, mmu: &mut MMU) {self.opcode_inc_r(mmu, 'A');}
	fn opcode_inc_b(&mut self, mmu: &mut MMU) {self.opcode_inc_r(mmu, 'B');}
	fn opcode_inc_c(&mut self, mmu: &mut MMU) {self.opcode_inc_r(mmu, 'C');}
	fn opcode_inc_d(&mut self, mmu: &mut MMU) {self.opcode_inc_r(mmu, 'D');}
	fn opcode_inc_e(&mut self, mmu: &mut MMU) {self.opcode_inc_r(mmu, 'E');}
	fn opcode_inc_h(&mut self, mmu: &mut MMU) {self.opcode_inc_r(mmu, 'H');}
	fn opcode_inc_l(&mut self, mmu: &mut MMU) {self.opcode_inc_r(mmu, 'L');}
	

	// INC m: Increment memory pointed by double register
	fn opcode_inc_m_rr(&mut self, mmu: &mut MMU, double_reg: &str) {
		let mem = self.double_register_value(double_reg);
		let result = mmu.get_byte(mem).wrapping_add(1);
		let hc = (((mmu.get_byte(mem) & 0xF) + (1 & 0xF)) & 0x10) >> 4;
		
		mmu.set_byte(mem, result);
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', hc);
		self.mcycles = 3;
	}
	// INC (HL)
	fn opcode_inc_m_hl(&mut self, mmu: &mut MMU) {self.opcode_inc_m_rr(mmu, "HL");}

	// DEC r: Decrement register
	fn opcode_dec_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_reg = self.r_index(r);
		let (result, _) = self.cpu_registers[r_reg].overflowing_sub(1);
		let hc = ((self.cpu_registers[r_reg] & 0xF).wrapping_sub(1) & 0x10) >> 4;
		self.cpu_registers[r_reg] = result;

		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		self.mcycles = 1;
	}
	// DEC A
	fn opcode_dec_a(&mut self, mmu: &mut MMU) {self.opcode_dec_r(mmu, 'A');}
	// DEC B
	fn opcode_dec_b(&mut self, mmu: &mut MMU) {self.opcode_dec_r(mmu, 'B');}
	// DEC C
	fn opcode_dec_c(&mut self, mmu: &mut MMU) {self.opcode_dec_r(mmu, 'C');}
	// DEC D
	fn opcode_dec_d(&mut self, mmu: &mut MMU) {self.opcode_dec_r(mmu, 'D');}
	// DEC E
	fn opcode_dec_e(&mut self, mmu: &mut MMU) {self.opcode_dec_r(mmu, 'E');}
	// DEC H
	fn opcode_dec_h(&mut self, mmu: &mut MMU) {self.opcode_dec_r(mmu, 'H');}
	// DEC L
	fn opcode_dec_l(&mut self, mmu: &mut MMU) {self.opcode_dec_r(mmu, 'L');}

	// DEC m: Decrement memory pointed by double register
	fn opcode_dec_m(&mut self, mmu: &mut MMU, double_reg: &str) {
		let mem = self.double_register_value(double_reg);
		let (result, _) = mmu.get_byte(mem).overflowing_sub(1);
		let hc = (((mmu.get_byte(mem) & 0xF).wrapping_sub(1)) & 0x10) >> 4;
		mmu.set_byte(mem, result);
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 1);
		self.set_flag('h', hc);
		self.mcycles = 3;
	}
	// DEC (HL)
	fn opcode_dec_m_hl(&mut self, mmu: &mut MMU) {self.opcode_dec_m(mmu, "HL");}
		

	// DA r: Decimal adjust register
	fn opcode_da_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let mut adjusted_value = self.cpu_registers[r_idx];
		if self.get_flag('n') == 0 {
			if self.get_flag('c') == 1 || adjusted_value > 0x99 {
				adjusted_value = adjusted_value.wrapping_add(0x60);
				self.set_flag('c', 1);
			}
			if self.get_flag('h') == 1 || adjusted_value & 0xF > 9 {
				adjusted_value = adjusted_value.wrapping_add(0x06);
			}
		}
		else if self.get_flag('n') == 1 {
			if self.get_flag('c') == 1 {
				adjusted_value = adjusted_value.wrapping_sub(0x60);
			}
			if self.get_flag('h') == 1 {
				adjusted_value = adjusted_value.wrapping_sub(0x06);
			}
		}
		
		self.cpu_registers[r_idx] = adjusted_value;

		if adjusted_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('h', 0);
		self.mcycles = 1;
	}
	// DAA
	fn opcode_daa(&mut self, mmu: &mut MMU) {self.opcode_da_r(mmu, 'A');}

	// CPL r: Complement register
	fn opcode_cpl_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let flipped_value = !self.cpu_registers[r_idx];
		self.cpu_registers[r_idx] = flipped_value;

		self.set_flag('n', 1);
		self.set_flag('h', 1);
		self.mcycles = 1;
	}
	// CPL A
	fn opcode_cpl_a(&mut self, mmu: &mut MMU) {self.opcode_cpl_r(mmu, 'A');}
	

	// ADD rr, rr: Add double register to double register
	fn opcode_add_rr_rr(&mut self, _mmu: &mut MMU, dreg_str1: &str, dreg_str2: &str) {
		let dreg_val1 = self.double_register_value(dreg_str1);
		let dreg_val2 = self.double_register_value(dreg_str2);
		let (result, overflow) = dreg_val1.overflowing_add(dreg_val2);
		let hc = (((dreg_val1 & 0xFFF) + (dreg_val2 & 0xFFF)) & 0x1000) >> 12;
		self.set_double_register(dreg_str1, result as u16);
		
		self.set_flag('n', 0);
		self.set_flag('h', hc as u8);
		if overflow == true {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// ADD HL, BC
	fn opcode_add_hl_bc(&mut self, mmu: &mut MMU) {self.opcode_add_rr_rr(mmu, "HL", "BC");}
	// ADD HL, BC
	fn opcode_add_hl_de(&mut self, mmu: &mut MMU) {self.opcode_add_rr_rr(mmu, "HL", "DE");}
	// ADD HL, HL
	fn opcode_add_hl_hl(&mut self, mmu: &mut MMU) {self.opcode_add_rr_rr(mmu, "HL", "HL");}
	// ADD HL, SP
	fn opcode_add_hl_sp(&mut self, mmu: &mut MMU) {self.opcode_add_rr_rr(mmu, "HL", "SP");}

	// INC rr: Increment value of double register
	fn opcode_inc_rr(&mut self, _mmu: &mut MMU, dreg_str: &str) {
		let dreg_val = self.double_register_value(dreg_str);
		let result = dreg_val.wrapping_add(1);
		self.set_double_register(dreg_str, result as u16);
		self.mcycles = 2;
	}
	// INC BC
	fn opcode_inc_bc(&mut self, mmu: &mut MMU) {self.opcode_inc_rr(mmu, "BC");}
	// INC DE
	fn opcode_inc_de(&mut self, mmu: &mut MMU) {self.opcode_inc_rr(mmu, "DE");}
	// INC HL
	fn opcode_inc_hl(&mut self, mmu: &mut MMU) {self.opcode_inc_rr(mmu, "HL");}
	// INC SP
	fn opcode_inc_sp(&mut self, mmu: &mut MMU) {self.opcode_inc_rr(mmu, "SP");}

	// DEC rr: Decrement value of double register
	fn opcode_dec_rr(&mut self, _mmu: &mut MMU, dreg_str: &str) {
		let dreg_val = self.double_register_value(dreg_str);
		let result = dreg_val.wrapping_sub(1);
		self.set_double_register(dreg_str, result as u16);
		self.mcycles = 2;
	}
	// DEC BC
	fn opcode_dec_bc(&mut self, mmu: &mut MMU) {self.opcode_dec_rr(mmu, "BC");}
	// DEC DE
	fn opcode_dec_de(&mut self, mmu: &mut MMU) {self.opcode_dec_rr(mmu, "DE");}
	// DEC HL
	fn opcode_dec_hl(&mut self, mmu: &mut MMU) {self.opcode_dec_rr(mmu, "HL");}
	// DEC SP
	fn opcode_dec_sp(&mut self, mmu: &mut MMU) {self.opcode_dec_rr(mmu, "SP");}

	// ADD rr, dd: Add signed 8-bit to double register
	fn opcode_add_rr_dd(&mut self, mmu: &mut MMU, dreg_str: &str) {
		let dd = self.fetch_byte(mmu);
		let dreg_val = self.double_register_value(dreg_str);
		let signed_value = dd as i8;
		let result = dreg_val.wrapping_add_signed(signed_value as i16);
		self.set_double_register(dreg_str, result as u16);

		let hc = ((dreg_val & 0xF) + (dd as u16 & 0xF) & 0x10) >> 4;
		let (_, fc) = (dreg_val as u8).overflowing_add(dd);
		
		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', hc as u8);
		self.set_flag('c', fc as u8);
		self.mcycles = 4;
	}
	// ADD SP, dd
	fn opcode_add_sp_dd(&mut self, mmu: &mut MMU) {self.opcode_add_rr_dd(mmu, "SP");}
	
	// LDS RR, RR, dd: Add signed 8-bit to double register, then store the result in memory
	// pointed by double-register
	fn opcode_lds(&mut self, mmu: &mut MMU, dreg_str1: &str, dreg_str2: &str) {
		let dd = self.fetch_byte(mmu);
		let dreg_val = self.double_register_value(dreg_str2);
		let signed_value = dd as i8;
		let result = dreg_val.wrapping_add_signed(signed_value as i16);
		self.set_double_register(dreg_str1, result as u16);

		let hc = ((dreg_val & 0xF) + (dd as u16 & 0xF) & 0x10) >> 4;
		let (_, fc) = (dreg_val as u8).overflowing_add(dd);
		
		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', hc as u8);
		self.set_flag('c', fc as u8);
		self.mcycles = 4;
	}
	// LD HL, SP+dd
	fn opcode_lds_hl_sp(&mut self, mmu: &mut MMU) {self.opcode_lds(mmu, "HL", "SP");}

	// RLCA: Rotate A left
	fn opcode_rlca(&mut self, _mmu: &mut MMU) {
		let r_idx = self.r_index('A');
		let bit7 = (self.cpu_registers[r_idx] & 0x80) >> 7;
		let new_value = self.cpu_registers[r_idx].rotate_left(1);
		self.cpu_registers[r_idx] = new_value;
		self.set_bit('A', 0, bit7);
		
		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit7);
		self.mcycles = 1;
	}

	// RLA: Rotate A left through carry
	fn opcode_rla(&mut self, _mmu: &mut MMU) {
		let r_idx = self.r_index('A');
		let c_flag = self.get_flag('c');
		let bit7 = (self.cpu_registers[r_idx] & 0x80) >> 7;
		let new_value = self.cpu_registers[r_idx].rotate_left(1);
		self.cpu_registers[r_idx] = new_value;
		self.set_bit('A', 0, c_flag);
		
		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit7);
		self.mcycles = 1;
	}

	// RRCA: Rotate A right 
	fn opcode_rrca(&mut self, _mmu: &mut MMU) {
		let r_idx = self.r_index('A');
		let c = self.cpu_registers[r_idx] & 0x01;
		let new_value = self.cpu_registers[r_idx].rotate_right(1);
		self.cpu_registers[r_idx] = new_value;

		self.set_flag('z', 0);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', c);
		self.mcycles = 1;
	}

	// // RRA: Rotate A right through carry
	// fn opcode_rra(&mut self) {
	// 	let r_idx = self.r_index('A');
	// 	let c = self.cpu_registers[r_idx] & 0x01;
	// 	let mut new_value = self.cpu_registers[r_idx].rotate_right(1);
	// 	new_value |= c << 7;
	// 	self.cpu_registers[r_idx] = new_value;

	// 	self.set_flag('z', 0);
	// 	self.set_flag('n', 0);
	// 	self.set_flag('h', 0);
	// 	self.set_flag('c', c);
	// }

	// RLC r: Rotate r left
	fn opcode_rlc_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let bit7 = (self.cpu_registers[r_idx] & 0x80) >> 7;
		let new_value = self.cpu_registers[r_idx].rotate_left(1);
		self.cpu_registers[r_idx] = new_value;
		self.set_bit(r, 0, bit7);
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit7);
		self.mcycles = 2;
	}
	// RLC A
	fn opcode_rlc_a(&mut self, mmu: &mut MMU) {self.opcode_rlc_r(mmu, 'A');}
	// RLC B
	fn opcode_rlc_b(&mut self, mmu: &mut MMU) {self.opcode_rlc_r(mmu, 'B');}
	// RLC C
	fn opcode_rlc_c(&mut self, mmu: &mut MMU) {self.opcode_rlc_r(mmu, 'C');}
	// RLC D
	fn opcode_rlc_d(&mut self, mmu: &mut MMU) {self.opcode_rlc_r(mmu, 'D');}
	// RLC E
	fn opcode_rlc_e(&mut self, mmu: &mut MMU) {self.opcode_rlc_r(mmu, 'E');}
	// RLC H
	fn opcode_rlc_h(&mut self, mmu: &mut MMU) {self.opcode_rlc_r(mmu, 'H');}
	// RLC L
	fn opcode_rlc_l(&mut self, mmu: &mut MMU) {self.opcode_rlc_r(mmu, 'L');}

	// RLC m: Rotate m left
	fn opcode_rlc_m(&mut self, mmu: &mut MMU, dreg_str: &str) {
		let mem = self.double_register_value(dreg_str);
		let bit7 = (mmu.get_byte(mem) & 0x80) >> 7;
		let new_value = mmu.get_byte(mem).rotate_left(1);
		mmu.set_byte(mem, new_value);
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit7);
		self.mcycles = 4;
	}
	// RLC (HL)
	fn opcode_rlc_m_hl(&mut self, mmu: &mut MMU) {self.opcode_rlc_m(mmu, "HL");}

	// RL r: Rotate r left through carry
	fn opcode_rl_r(&mut self, _mmu: &mut MMU, r: char) {
		let c_flag = self.get_flag('c');
		let r_idx = self.r_index(r);
		let bit7 = (self.cpu_registers[r_idx] & 0x80) >> 7;
		let new_value = self.cpu_registers[r_idx].rotate_left(1);
		self.cpu_registers[r_idx] = new_value;
		self.set_bit(r, 0, c_flag);
		
		if self.cpu_registers[r_idx] == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit7);
		self.mcycles = 2;
	}
	// RL A
	fn opcode_rl_a(&mut self, mmu: &mut MMU) {self.opcode_rl_r(mmu, 'A');}
	// RL B
	fn opcode_rl_b(&mut self, mmu: &mut MMU) {self.opcode_rl_r(mmu, 'B');}
	// RL C
	fn opcode_rl_c(&mut self, mmu: &mut MMU) {self.opcode_rl_r(mmu, 'C');}
	// RL D
	fn opcode_rl_d(&mut self, mmu: &mut MMU) {self.opcode_rl_r(mmu, 'D');}
	// RL E
	fn opcode_rl_e(&mut self, mmu: &mut MMU) {self.opcode_rl_r(mmu, 'E');}
	// RL H
	fn opcode_rl_h(&mut self, mmu: &mut MMU) {self.opcode_rl_r(mmu, 'H');}
	// RL L
	fn opcode_rl_l(&mut self, mmu: &mut MMU) {self.opcode_rl_r(mmu, 'L');}
		

	// RL m: Rotate m left through carry
	fn opcode_rl_m(&mut self, mmu: &mut MMU, dreg_str: &str) {
		let c_flag = self.get_flag('c');
		let mem = self.double_register_value(dreg_str);
		let bit7 = (mmu.get_byte(mem) & 0x80) >> 7;
		let mut new_value = mmu.get_byte(mem).rotate_left(1);
		if c_flag == 0 {
			new_value &= 0b1111_1110;
		} else {
			new_value |= 1;
		}
		mmu.set_byte(mem, new_value);
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit7);
		self.mcycles = 4;
	}
	// RL (HL)
	fn opcode_rl_hl(&mut self, mmu: &mut MMU) {self.opcode_rl_m(mmu, "HL");}

	// RRC r: Rotate register right 
	fn opcode_rrc_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let bit0 = self.cpu_registers[r_idx] & 0x01;
		let new_value = self.cpu_registers[r_idx].rotate_right(1);
		self.cpu_registers[r_idx] = new_value;
		self.set_bit(r, 7, bit0);

		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit0);
		self.mcycles = 2;
	}
	// RRC A
	fn opcode_rrc_a(&mut self, mmu: &mut MMU) {self.opcode_rrc_r(mmu, 'A');}
	// RRC B
	fn opcode_rrc_b(&mut self, mmu: &mut MMU) {self.opcode_rrc_r(mmu, 'B');}
	// RRC C
	fn opcode_rrc_c(&mut self, mmu: &mut MMU) {self.opcode_rrc_r(mmu, 'C');}
	// RRC D
	fn opcode_rrc_d(&mut self, mmu: &mut MMU) {self.opcode_rrc_r(mmu, 'D');}
	// RRC E
	fn opcode_rrc_e(&mut self, mmu: &mut MMU) {self.opcode_rrc_r(mmu, 'E');}
	// RRC H
	fn opcode_rrc_h(&mut self, mmu: &mut MMU) {self.opcode_rrc_r(mmu, 'H');}
	// RRC L
	fn opcode_rrc_l(&mut self, mmu: &mut MMU) {self.opcode_rrc_r(mmu, 'L');}

	// RRC m: Rotate memory right
	fn opcode_rrc_m(&mut self, mmu: &mut MMU, dreg_str: &str) {
		let mem = self.double_register_value(dreg_str);
		let bit0 = mmu.get_byte(mem) & 0x01;
		let new_value = mmu.get_byte(mem).rotate_right(1);
		mmu.set_byte(mem, new_value);
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit0);
		self.mcycles = 4;
	}
	// RRC (HL)
	fn opcode_rrc_hl(&mut self, mmu: &mut MMU) {self.opcode_rrc_m(mmu, "HL");}

	// RR r: Rotate register right through carry
	fn opcode_rr_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let bit0 = self.cpu_registers[r_idx] & 0x01;
		let c_flag = self.get_flag('c');
		let new_value = self.cpu_registers[r_idx].rotate_right(1);
		self.cpu_registers[r_idx] = new_value;
		self.set_bit(r, 7, c_flag);

		if self.cpu_registers[r_idx] == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit0);
		self.mcycles = 2;
	}
	// RRA
	fn opcode_rra(&mut self, mmu: &mut MMU) {
		self.opcode_rr_r(mmu, 'A');
		self.set_flag('z', 0);
		self.mcycles = 1;
	}
	// RR A
	fn opcode_rr_a(&mut self, mmu: &mut MMU) {self.opcode_rr_r(mmu, 'A');}
	// RR B
	fn opcode_rr_b(&mut self, mmu: &mut MMU) {self.opcode_rr_r(mmu, 'B');}
	// RR C
	fn opcode_rr_c(&mut self, mmu: &mut MMU) {self.opcode_rr_r(mmu, 'C');}
	// RR D
	fn opcode_rr_d(&mut self, mmu: &mut MMU) {self.opcode_rr_r(mmu, 'D');}
	// RR E
	fn opcode_rr_e(&mut self, mmu: &mut MMU) {self.opcode_rr_r(mmu, 'E');}
	// RR H
	fn opcode_rr_h(&mut self, mmu: &mut MMU) {self.opcode_rr_r(mmu, 'H');}
	// RR L
	fn opcode_rr_l(&mut self, mmu: &mut MMU) {self.opcode_rr_r(mmu, 'L');}

	// RR m: Rotate memory right through carry
	fn opcode_rr_m(&mut self, mmu: &mut MMU, dreg_str: &str) {
		let c_flag = self.get_flag('c');
		let mem = self.double_register_value(dreg_str);
		let bit0 = mmu.get_byte(mem) & 0x01;
		let mut new_value = mmu.get_byte(mem).rotate_right(1);
		if c_flag == 0 {
			new_value &= !(1 << 7);
		} else {
			new_value |= 1 << 7;
		}
		mmu.set_byte(mem, new_value);
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit0);
		self.mcycles = 4;
	}
	// RR (HL)
	fn opcode_rr_hl(&mut self, mmu: &mut MMU) {self.opcode_rr_m(mmu, "HL");}

	// SLA r: Shift left register
	fn opcode_sla_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let bit7 = self.cpu_registers[r_idx] & 0x80;
		let result = self.cpu_registers[r_idx] << 1;
		self.cpu_registers[r_idx] = result;
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit7);
		self.mcycles = 2;
	}
	// SLA A
	fn opcode_sla_a(&mut self, mmu: &mut MMU) {self.opcode_sla_r(mmu, 'A');}
	// SLA B
	fn opcode_sla_b(&mut self, mmu: &mut MMU) {self.opcode_sla_r(mmu, 'B');}
	// SLA C
	fn opcode_sla_c(&mut self, mmu: &mut MMU) {self.opcode_sla_r(mmu, 'C');}
	// SLA D
	fn opcode_sla_d(&mut self, mmu: &mut MMU) {self.opcode_sla_r(mmu, 'D');}
	// SLA E
	fn opcode_sla_e(&mut self, mmu: &mut MMU) {self.opcode_sla_r(mmu, 'E');}
	// SLA H
	fn opcode_sla_h(&mut self, mmu: &mut MMU) {self.opcode_sla_r(mmu, 'H');}
	// SLA L
	fn opcode_sla_l(&mut self, mmu: &mut MMU) {self.opcode_sla_r(mmu, 'L');}

	// SLA m: Shift left memory pointed by double register
	fn opcode_sla_m(&mut self, mmu: &mut MMU, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let bit7 = (mmu.get_byte(mem) & 0x80) >> 7;
		let result = mmu.get_byte(mem) << 1;
		mmu.set_byte(mem, result);
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit7);
		self.mcycles = 4;
	}
	// SLA (HL)
	fn opcode_sla_hl(&mut self, mmu: &mut MMU) {self.opcode_sla_m(mmu, "HL");}

	// SWAP r: Swap nibbles of register
	fn opcode_swap_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let value = self.cpu_registers[r_idx];
		let low_nibble = value & 0x0F;
		let high_nibble = value & 0xF0;
		let new_value = (low_nibble << 4) | (high_nibble >> 4);
		self.cpu_registers[r_idx] = new_value;

		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
		self.mcycles = 2;
	}
	// SWAP A
	fn opcode_swap_a(&mut self, mmu: &mut MMU) {self.opcode_swap_r(mmu, 'A');}
	// SWAP B
	fn opcode_swap_b(&mut self, mmu: &mut MMU) {self.opcode_swap_r(mmu, 'B');}
	// SWAP C
	fn opcode_swap_c(&mut self, mmu: &mut MMU) {self.opcode_swap_r(mmu, 'C');}
	// SWAP D
	fn opcode_swap_d(&mut self, mmu: &mut MMU) {self.opcode_swap_r(mmu, 'D');}
	// SWAP E
	fn opcode_swap_e(&mut self, mmu: &mut MMU) {self.opcode_swap_r(mmu, 'E');}
	// SWAP H
	fn opcode_swap_h(&mut self, mmu: &mut MMU) {self.opcode_swap_r(mmu, 'H');}
	// SWAP L
	fn opcode_swap_l(&mut self, mmu: &mut MMU) {self.opcode_swap_r(mmu, 'L');}

	// SWAP m: Swap nibbles of memory pointed by double register
	fn opcode_swap_m(&mut self, mmu: &mut MMU, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let value = mmu.get_byte(mem);
		let low_nibble = value & 0xF;
		let high_nibble = value & 0xF0;
		let new_value = (low_nibble << 4) | (high_nibble >> 4);
		mmu.set_byte(mem, new_value);

		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', 0);
		self.mcycles = 4;
	}
	// SWAP (HL)
	fn opcode_swap_hl(&mut self, mmu: &mut MMU) {self.opcode_swap_m(mmu, "HL");}

	// SRA r: Shift right arithmetic register (b7 = b7)
	fn opcode_sra_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let bit7 = self.cpu_registers[r_idx] & 0x80;
		let bit0 = self.cpu_registers[r_idx] & 0x01;
		let result = self.cpu_registers[r_idx].wrapping_shr(1);
		self.cpu_registers[r_idx] = result;
		self.set_bit(r, 7, bit7);
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		if bit0 == 1 {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// SRA A
	fn opcode_sra_a(&mut self, mmu: &mut MMU) {self.opcode_sra_r(mmu, 'A');}
	// SRA B
	fn opcode_sra_b(&mut self, mmu: &mut MMU) {self.opcode_sra_r(mmu, 'B');}
	// SRA C
	fn opcode_sra_c(&mut self, mmu: &mut MMU) {self.opcode_sra_r(mmu, 'C');}
	// SRA D
	fn opcode_sra_d(&mut self, mmu: &mut MMU) {self.opcode_sra_r(mmu, 'D');}
	// SRA E
	fn opcode_sra_e(&mut self, mmu: &mut MMU) {self.opcode_sra_r(mmu, 'E');}
	// SRA H
	fn opcode_sra_h(&mut self, mmu: &mut MMU) {self.opcode_sra_r(mmu, 'H');}
	// SRA L
	fn opcode_sra_l(&mut self, mmu: &mut MMU) {self.opcode_sra_r(mmu, 'L');}

	// SRA m: Shift right arithmetic memory pointed by double register (b7 = b7)
	fn opcode_sra_m(&mut self, mmu: &mut MMU, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let bit0 = mmu.get_byte(mem) & 0x01;
		let bit7 = (mmu.get_byte(mem) & 0x80) >> 7;
		let mut new_value = mmu.get_byte(mem) >> 1;
		if bit7 == 0 {
			new_value &= !(1 << 7);
		} else {
			new_value |= 1 << 7;
		}
		mmu.set_byte(mem, new_value);
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit0);
		self.mcycles = 4;
	}
	// SRA (HL)
	fn opcode_sra_hl(&mut self, mmu: &mut MMU) {self.opcode_sra_m(mmu, "HL");}

	// SRL r: Shift right logical register (b7 = 0)
	fn opcode_srl_r(&mut self, _mmu: &mut MMU, r: char) {
		let r_idx = self.r_index(r);
		let bit0 = self.cpu_registers[r_idx] & 0x01;
		let result = self.cpu_registers[r_idx] >> 1;
		self.cpu_registers[r_idx] = result;
		
		if result == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		if bit0 == 1 {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		self.mcycles = 2;
	}
	// SRL A
	fn opcode_srl_a(&mut self, mmu: &mut MMU) {self.opcode_srl_r(mmu, 'A');}
	// SRL B
	fn opcode_srl_b(&mut self, mmu: &mut MMU) {self.opcode_srl_r(mmu, 'B');}
	// SRL C
	fn opcode_srl_c(&mut self, mmu: &mut MMU) {self.opcode_srl_r(mmu, 'C');}
	// SRL D
	fn opcode_srl_d(&mut self, mmu: &mut MMU) {self.opcode_srl_r(mmu, 'D');}
	// SRL E
	fn opcode_srl_e(&mut self, mmu: &mut MMU) {self.opcode_srl_r(mmu, 'E');}
	// SRL H
	fn opcode_srl_h(&mut self, mmu: &mut MMU) {self.opcode_srl_r(mmu, 'H');}
	// SRL L
	fn opcode_srl_l(&mut self, mmu: &mut MMU) {self.opcode_srl_r(mmu, 'L');}

	// SRL m: Shift right logical memory pointed by double register (b7 = 0)
	fn opcode_srl_m(&mut self, mmu: &mut MMU, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let bit0 = mmu.get_byte(mem) & 0x01;
		let new_value = mmu.get_byte(mem) >> 1;
		mmu.set_byte(mem, new_value);
		
		if new_value == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.set_flag('c', bit0);
		self.mcycles = 4;
	}
	// SRL (HL)
	fn opcode_srl_hl(&mut self, mmu: &mut MMU) {self.opcode_srl_m(mmu, "HL");}

	// BIT n, r: Test bit n in register r
	fn opcode_bit_n_r(&mut self, _mmu: &mut MMU, n: u8, r: char) {
		let r_idx = self.r_index(r);
		let bit = (self.cpu_registers[r_idx] >> n) & 0x1;

		if bit == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
		self.mcycles = 2;
	}
	// BIT 0, A
	fn opcode_bit_0_a(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 0, 'A');}
	// BIT 0, B
	fn opcode_bit_0_b(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 0, 'B');}
	// BIT 0, C
	fn opcode_bit_0_c(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 0, 'C');}
	// BIT 0, D
	fn opcode_bit_0_d(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 0, 'D');}
	// BIT 0, E
	fn opcode_bit_0_e(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 0, 'E');}
	// BIT 0, H
	fn opcode_bit_0_h(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 0, 'H');}
	// BIT 0, L
	fn opcode_bit_0_l(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 0, 'L');}

	// BIT 1, A
	fn opcode_bit_1_a(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 1, 'A');}
	// BIT 1, B
	fn opcode_bit_1_b(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 1, 'B');}
	// BIT 1, C
	fn opcode_bit_1_c(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 1, 'C');}
	// BIT 1, D
	fn opcode_bit_1_d(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 1, 'D');}
	// BIT 1, E
	fn opcode_bit_1_e(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 1, 'E');}
	// BIT 1, H
	fn opcode_bit_1_h(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 1, 'H');}
	// BIT 1, L
	fn opcode_bit_1_l(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 1, 'L');}

	// BIT 2, A
	fn opcode_bit_2_a(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 2, 'A');}
	// BIT 2, B
	fn opcode_bit_2_b(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 2, 'B');}
	// BIT 2, C
	fn opcode_bit_2_c(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 2, 'C');}
	// BIT 2, D
	fn opcode_bit_2_d(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 2, 'D');}
	// BIT 2, E
	fn opcode_bit_2_e(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 2, 'E');}
	// BIT 2, H
	fn opcode_bit_2_h(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 2, 'H');}
	// BIT 2, L
	fn opcode_bit_2_l(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 2, 'L');}

	// BIT 3, A
	fn opcode_bit_3_a(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 3, 'A');}
	// BIT 3, B
	fn opcode_bit_3_b(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 3, 'B');}
	// BIT 3, C
	fn opcode_bit_3_c(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 3, 'C');}
	// BIT 3, D
	fn opcode_bit_3_d(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 3, 'D');}
	// BIT 3, E
	fn opcode_bit_3_e(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 3, 'E');}
	// BIT 3, H
	fn opcode_bit_3_h(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 3, 'H');}
	// BIT 3, L
	fn opcode_bit_3_l(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 3, 'L');}

	// BIT 4, A
	fn opcode_bit_4_a(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 4, 'A');}
	// BIT 4, B
	fn opcode_bit_4_b(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 4, 'B');}
	// BIT 4, C
	fn opcode_bit_4_c(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 4, 'C');}
	// BIT 4, D
	fn opcode_bit_4_d(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 4, 'D');}
	// BIT 4, E
	fn opcode_bit_4_e(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 4, 'E');}
	// BIT 4, H
	fn opcode_bit_4_h(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 4, 'H');}
	// BIT 4, L
	fn opcode_bit_4_l(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 4, 'L');}

	// BIT 5, A
	fn opcode_bit_5_a(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 5, 'A');}
	// BIT 5, B
	fn opcode_bit_5_b(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 5, 'B');}
	// BIT 5, C
	fn opcode_bit_5_c(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 5, 'C');}
	// BIT 5, D
	fn opcode_bit_5_d(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 5, 'D');}
	// BIT 5, E
	fn opcode_bit_5_e(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 5, 'E');}
	// BIT 5, H
	fn opcode_bit_5_h(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 5, 'H');}
	// BIT 5, L
	fn opcode_bit_5_l(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 5, 'L');}

	// BIT 6, A
	fn opcode_bit_6_a(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 6, 'A');}
	// BIT 6, B
	fn opcode_bit_6_b(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 6, 'B');}
	// BIT 6, C
	fn opcode_bit_6_c(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 6, 'C');}
	// BIT 6, D
	fn opcode_bit_6_d(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 6, 'D');}
	// BIT 6, E
	fn opcode_bit_6_e(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 6, 'E');}
	// BIT 6, H
	fn opcode_bit_6_h(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 6, 'H');}
	// BIT 6, L
	fn opcode_bit_6_l(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 6, 'L');}

	// BIT 7, A
	fn opcode_bit_7_a(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 7, 'A');}
	// BIT 7, B
	fn opcode_bit_7_b(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 7, 'B');}
	// BIT 7, C
	fn opcode_bit_7_c(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 7, 'C');}
	// BIT 7, D
	fn opcode_bit_7_d(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 7, 'D');}
	// BIT 7, E
	fn opcode_bit_7_e(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 7, 'E');}
	// BIT 7, H
	fn opcode_bit_7_h(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 7, 'H');}
	// BIT 7, L
	fn opcode_bit_7_l(&mut self, mmu: &mut MMU) {self.opcode_bit_n_r(mmu, 7, 'L');}

	// BIT n, m: Test bit n in memory pointed by double register
	fn opcode_bit_n_m(&mut self, mmu: &mut MMU, n: u8, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let bit = (mmu.get_byte(mem) >> n) & 0x1;

		if bit == 0 {
			self.set_flag('z', 1);
		} else {
			self.set_flag('z', 0);
		}
		self.set_flag('n', 0);
		self.set_flag('h', 1);
		self.mcycles = 4;
	}
	// BIT 0, (HL)
	fn opcode_bit_0_hl(&mut self, mmu: &mut MMU) {self.opcode_bit_n_m(mmu, 0, "HL");}
	// BIT 1, (HL)
	fn opcode_bit_1_hl(&mut self, mmu: &mut MMU) {self.opcode_bit_n_m(mmu, 1, "HL");}
	// BIT 2, (HL)
	fn opcode_bit_2_hl(&mut self, mmu: &mut MMU) {self.opcode_bit_n_m(mmu, 2, "HL");}
	// BIT 3, (HL)
	fn opcode_bit_3_hl(&mut self, mmu: &mut MMU) {self.opcode_bit_n_m(mmu, 3, "HL");}
	// BIT 4, (HL)
	fn opcode_bit_4_hl(&mut self, mmu: &mut MMU) {self.opcode_bit_n_m(mmu, 4, "HL");}
	// BIT 5, (HL)
	fn opcode_bit_5_hl(&mut self, mmu: &mut MMU) {self.opcode_bit_n_m(mmu, 5, "HL");}
	// BIT 6, (HL)
	fn opcode_bit_6_hl(&mut self, mmu: &mut MMU) {self.opcode_bit_n_m(mmu, 6, "HL");}
	// BIT 7, (HL)
	fn opcode_bit_7_hl(&mut self, mmu: &mut MMU) {self.opcode_bit_n_m(mmu, 7, "HL");}

	// SET n, r: Set bit n in register
	fn opcode_set_n_r(&mut self, _mmu: &mut MMU, n: u8, r: char) {
		let r_idx = self.r_index(r);
		self.cpu_registers[r_idx] |= 1 << n;
		self.mcycles = 2;
	}
	// SET 0, A
	fn opcode_set_0_a(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 0, 'A');}
	// SET 0, B
	fn opcode_set_0_b(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 0, 'B');}
	// SET 0, C
	fn opcode_set_0_c(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 0, 'C');}
	// SET 0, D
	fn opcode_set_0_d(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 0, 'D');}
	// SET 0, E
	fn opcode_set_0_e(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 0, 'E');}
	// SET 0, H
	fn opcode_set_0_h(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 0, 'H');}
	// SET 0, L
	fn opcode_set_0_l(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 0, 'L');}

	// SET 1, A
	fn opcode_set_1_a(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 1, 'A');}
	// SET 1, B
	fn opcode_set_1_b(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 1, 'B');}
	// SET 1, C
	fn opcode_set_1_c(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 1, 'C');}
	// SET 1, D
	fn opcode_set_1_d(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 1, 'D');}
	// SET 1, E
	fn opcode_set_1_e(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 1, 'E');}
	// SET 1, H
	fn opcode_set_1_h(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 1, 'H');}
	// SET 1, L
	fn opcode_set_1_l(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 1, 'L');}

	// SET 2, A
	fn opcode_set_2_a(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 2, 'A');}
	// SET 2, B
	fn opcode_set_2_b(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 2, 'B');}
	// SET 2, C
	fn opcode_set_2_c(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 2, 'C');}
	// SET 2, D
	fn opcode_set_2_d(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 2, 'D');}
	// SET 2, E
	fn opcode_set_2_e(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 2, 'E');}
	// SET 2, H
	fn opcode_set_2_h(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 2, 'H');}
	// SET 2, L
	fn opcode_set_2_l(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 2, 'L');}

	// SET 3, A
	fn opcode_set_3_a(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 3, 'A');}
	// SET 3, B
	fn opcode_set_3_b(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 3, 'B');}
	// SET 3, C
	fn opcode_set_3_c(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 3, 'C');}
	// SET 3, D
	fn opcode_set_3_d(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 3, 'D');}
	// SET 3, E
	fn opcode_set_3_e(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 3, 'E');}
	// SET 3, H
	fn opcode_set_3_h(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 3, 'H');}
	// SET 3, L
	fn opcode_set_3_l(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 3, 'L');}

	// SET 4, A
	fn opcode_set_4_a(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 4, 'A');}
	// SET 4, B
	fn opcode_set_4_b(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 4, 'B');}
	// SET 4, C
	fn opcode_set_4_c(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 4, 'C');}
	// SET 4, D
	fn opcode_set_4_d(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 4, 'D');}
	// SET 4, E
	fn opcode_set_4_e(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 4, 'E');}
	// SET 4, H
	fn opcode_set_4_h(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 4, 'H');}
	// SET 4, L
	fn opcode_set_4_l(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 4, 'L');}

	// SET 5, A
	fn opcode_set_5_a(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 5, 'A');}
	// SET 5, B
	fn opcode_set_5_b(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 5, 'B');}
	// SET 5, C
	fn opcode_set_5_c(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 5, 'C');}
	// SET 5, D
	fn opcode_set_5_d(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 5, 'D');}
	// SET 5, E
	fn opcode_set_5_e(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 5, 'E');}
	// SET 5, H
	fn opcode_set_5_h(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 5, 'H');}
	// SET 5, L
	fn opcode_set_5_l(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 5, 'L');}

	// SET 6, A
	fn opcode_set_6_a(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 6, 'A');}
	// SET 6, B
	fn opcode_set_6_b(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 6, 'B');}
	// SET 6, C
	fn opcode_set_6_c(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 6, 'C');}
	// SET 6, D
	fn opcode_set_6_d(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 6, 'D');}
	// SET 6, E
	fn opcode_set_6_e(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 6, 'E');}
	// SET 6, H
	fn opcode_set_6_h(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 6, 'H');}
	// SET 6, L
	fn opcode_set_6_l(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 6, 'L');}

	// SET 7, A
	fn opcode_set_7_a(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 7, 'A');}
	// SET 7, B
	fn opcode_set_7_b(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 7, 'B');}
	// SET 7, C
	fn opcode_set_7_c(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 7, 'C');}
	// SET 7, D
	fn opcode_set_7_d(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 7, 'D');}
	// SET 7, E
	fn opcode_set_7_e(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 7, 'E');}
	// SET 7, H
	fn opcode_set_7_h(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 7, 'H');}
	// SET 7, L
	fn opcode_set_7_l(&mut self, mmu: &mut MMU) {self.opcode_set_n_r(mmu, 7, 'L');}


	// Set n, m: Set bit n in memory pointed by double register
	fn opcode_set_n_m(&mut self, mmu: &mut MMU, n: u8, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let new_value = mmu.get_byte(mem) | (1 << n);
		mmu.set_byte(mem, new_value);
	}
	// SET 0, (HL)
	fn opcode_set_0_hl(&mut self, mmu: &mut MMU) {self.opcode_set_n_m(mmu, 0, "HL");}
	// SET 1, (HL)
	fn opcode_set_1_hl(&mut self, mmu: &mut MMU) {self.opcode_set_n_m(mmu, 1, "HL");}
	// SET 2, (HL)
	fn opcode_set_2_hl(&mut self, mmu: &mut MMU) {self.opcode_set_n_m(mmu, 2, "HL");}
	// SET 3, (HL)
	fn opcode_set_3_hl(&mut self, mmu: &mut MMU) {self.opcode_set_n_m(mmu, 3, "HL");}
	// SET 4, (HL)
	fn opcode_set_4_hl(&mut self, mmu: &mut MMU) {self.opcode_set_n_m(mmu, 4, "HL");}
	// SET 5, (HL)
	fn opcode_set_5_hl(&mut self, mmu: &mut MMU) {self.opcode_set_n_m(mmu, 5, "HL");}
	// SET 6, (HL)
	fn opcode_set_6_hl(&mut self, mmu: &mut MMU) {self.opcode_set_n_m(mmu, 6, "HL");}
	// SET 7, (HL)
	fn opcode_set_7_hl(&mut self, mmu: &mut MMU) {self.opcode_set_n_m(mmu, 7, "HL");}

	// RES n, r: Reset bit n in register
	fn opcode_res_n_r(&mut self, _mmu: &mut MMU, n: u8, r: char) {
		let r_idx = self.r_index(r);
		self.cpu_registers[r_idx] &= !(1 << n);
		self.mcycles = 2;
	}
	// RES 0, A
	fn opcode_res_0_a(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 0, 'A');}
	// RES 0, B
	fn opcode_res_0_b(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 0, 'B');}
	// RES 0, C
	fn opcode_res_0_c(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 0, 'C');}
	// RES 0, D
	fn opcode_res_0_d(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 0, 'D');}
	// RES 0, E
	fn opcode_res_0_e(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 0, 'E');}
	// RES 0, H
	fn opcode_res_0_h(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 0, 'H');}
	// RES 0, L
	fn opcode_res_0_l(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 0, 'L');}

	// RES 1, A
	fn opcode_res_1_a(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 1, 'A');}
	// RES 1, B
	fn opcode_res_1_b(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 1, 'B');}
	// RES 1, C
	fn opcode_res_1_c(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 1, 'C');}
	// RES 1, D
	fn opcode_res_1_d(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 1, 'D');}
	// RES 1, E
	fn opcode_res_1_e(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 1, 'E');}
	// RES 1, H
	fn opcode_res_1_h(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 1, 'H');}
	// RES 1, L
	fn opcode_res_1_l(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 1, 'L');}

	// RES 2, A
	fn opcode_res_2_a(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 2, 'A');}
	// RES 2, B
	fn opcode_res_2_b(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 2, 'B');}
	// RES 2, C
	fn opcode_res_2_c(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 2, 'C');}
	// RES 2, D
	fn opcode_res_2_d(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 2, 'D');}
	// RES 2, E
	fn opcode_res_2_e(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 2, 'E');}
	// RES 2, H
	fn opcode_res_2_h(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 2, 'H');}
	// RES 2, L
	fn opcode_res_2_l(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 2, 'L');}

	// RES 3, A
	fn opcode_res_3_a(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 3, 'A');}
	// RES 3, B
	fn opcode_res_3_b(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 3, 'B');}
	// RES 3, C
	fn opcode_res_3_c(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 3, 'C');}
	// RES 3, D
	fn opcode_res_3_d(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 3, 'D');}
	// RES 3, E
	fn opcode_res_3_e(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 3, 'E');}
	// RES 3, H
	fn opcode_res_3_h(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 3, 'H');}
	// RES 3, L
	fn opcode_res_3_l(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 3, 'L');}

	// RES 4, A
	fn opcode_res_4_a(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 4, 'A');}
	// RES 4, B
	fn opcode_res_4_b(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 4, 'B');}
	// RES 4, C
	fn opcode_res_4_c(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 4, 'C');}
	// RES 4, D
	fn opcode_res_4_d(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 4, 'D');}
	// RES 4, E
	fn opcode_res_4_e(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 4, 'E');}
	// RES 4, H
	fn opcode_res_4_h(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 4, 'H');}
	// RES 4, L
	fn opcode_res_4_l(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 4, 'L');}

	// RES 5, A
	fn opcode_res_5_a(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 5, 'A');}
	// RES 5, B
	fn opcode_res_5_b(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 5, 'B');}
	// RES 5, C
	fn opcode_res_5_c(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 5, 'C');}
	// RES 5, D
	fn opcode_res_5_d(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 5, 'D');}
	// RES 5, E
	fn opcode_res_5_e(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 5, 'E');}
	// RES 5, H
	fn opcode_res_5_h(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 5, 'H');}
	// RES 5, L
	fn opcode_res_5_l(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 5, 'L');}

	// RES 6, A
	fn opcode_res_6_a(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 6, 'A');}
	// RES 6, B
	fn opcode_res_6_b(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 6, 'B');}
	// RES 6, C
	fn opcode_res_6_c(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 6, 'C');}
	// RES 6, D
	fn opcode_res_6_d(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 6, 'D');}
	// RES 6, E
	fn opcode_res_6_e(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 6, 'E');}
	// RES 6, H
	fn opcode_res_6_h(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 6, 'H');}
	// RES 6, L
	fn opcode_res_6_l(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 6, 'L');}

	// RES 7, A
	fn opcode_res_7_a(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 7, 'A');}
	// RES 7, B
	fn opcode_res_7_b(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 7, 'B');}
	// RES 7, C
	fn opcode_res_7_c(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 7, 'C');}
	// RES 7, D
	fn opcode_res_7_d(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 7, 'D');}
	// RES 7, E
	fn opcode_res_7_e(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 7, 'E');}
	// RES 7, H
	fn opcode_res_7_h(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 7, 'H');}
	// RES 7, L
	fn opcode_res_7_l(&mut self, mmu: &mut MMU) {self.opcode_res_n_r(mmu, 7, 'L');}
	
	// Res n, m: Reset bit n in memory pointed by double register
	fn opcode_res_n_m(&mut self, mmu: &mut MMU, n: u8, dreg: &str) {
		let mem = self.double_register_value(dreg);
		let new_value = mmu.get_byte(mem) & !(1 << n);
		mmu.set_byte(mem, new_value);
	}
	// RES 0, HL
	fn opcode_res_0_hl(&mut self, mmu: &mut MMU) {self.opcode_res_n_m(mmu, 0, "HL");}
	// RES 1, HL
	fn opcode_res_1_hl(&mut self, mmu: &mut MMU) {self.opcode_res_n_m(mmu, 1, "HL");}
	// RES 2, HL
	fn opcode_res_2_hl(&mut self, mmu: &mut MMU) {self.opcode_res_n_m(mmu, 2, "HL");}
	// RES 3, HL
	fn opcode_res_3_hl(&mut self, mmu: &mut MMU) {self.opcode_res_n_m(mmu, 3, "HL");}
	// RES 4, HL
	fn opcode_res_4_hl(&mut self, mmu: &mut MMU) {self.opcode_res_n_m(mmu, 4, "HL");}
	// RES 5, HL
	fn opcode_res_5_hl(&mut self, mmu: &mut MMU) {self.opcode_res_n_m(mmu, 5, "HL");}
	// RES 5, HL
	fn opcode_res_6_hl(&mut self, mmu: &mut MMU) {self.opcode_res_n_m(mmu, 6, "HL");}
	// RES 7, HL
	fn opcode_res_7_hl(&mut self, mmu: &mut MMU) {self.opcode_res_n_m(mmu, 7, "HL");}

	// CCF: Complement c flag, reset n and h flags
	fn opcode_ccf(&mut self, _mmu: &mut MMU) {
		let flag = self.get_flag('c');
		if flag == 0 {
			self.set_flag('c', 1);
		} else {
			self.set_flag('c', 0);
		}
		
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.mcycles = 1;
	}

	// SCF: Set c flag, reset n and h flags
	fn opcode_scf(&mut self, _mmu: &mut MMU) {
		self.set_flag('c', 1);
		self.set_flag('n', 0);
		self.set_flag('h', 0);
		self.mcycles = 1;
	}

	// NOP: No operation
	fn opcode_nop(&mut self, _mmu: &mut MMU) {
		self.mcycles = 1;
	}

	// // HALT: Halt in low pwer until interrupt occurs
	// // TODO
	// fn opcode_halt(&mut self) {
		
	// }

	// // STOP: Low power standby mode
	// // TODO
	// fn opcode_stop(&mut self) {
		
	// }

	// DI: Disable interrupts
	// TODO
	fn opcode_di(&mut self, _mmu: &mut MMU) {
		self.ime = 0;
		self.mcycles = 1;
	}

	// EI: Enable interrupts
	// TODO: It is delayed by one instruction, fix later
	fn opcode_ei(&mut self, __mmu: &mut MMU) {
		self.ime = 1;
		self.mcycles = 1;
	}

	// JP: Jump to nn
	fn opcode_jp_nn(&mut self, mmu: &mut MMU) {
		let nn = self.fetch_word(mmu);
		self.pc = nn;
		self.mcycles = 4;
	}

	// JP rr: Jump to memory pointed by double register
	fn opcode_jp_rr(&mut self, _mmu: &mut MMU, dreg: &str) {
		self.pc = (self.double_register_value(dreg)) as u16;
		self.mcycles = 1;
	}
	// JP HL
	fn opcode_jp_hl(&mut self, mmu: &mut MMU) {self.opcode_jp_rr(mmu, "HL");}

	// JP cc, nn: Jump conditional to nn
	fn opcode_jp_cc_nn(&mut self, mmu: &mut MMU, cc: &str) {
		let nn = self.fetch_word(mmu);
		let condition = match cc {
			"NZ" => self.get_flag('z') == 0,
			"Z"  => self.get_flag('z') == 1,
			"NC" => self.get_flag('c') == 0,
			"C"  => self.get_flag('c') == 1,
			_ => panic!("JP cc, nn"),
		};

		if condition == true {
			self.pc = nn;
			self.mcycles = 4;
		} else {
			self.mcycles = 3;
		}
	}
	// JP NZ, nn
	fn opcode_jp_nz_nn(&mut self, mmu: &mut MMU) {self.opcode_jp_cc_nn(mmu, "NZ");}
	// JP Z, nn
	fn opcode_jp_z_nn(&mut self, mmu: &mut MMU) {self.opcode_jp_cc_nn(mmu, "Z");}
	// JP NC, nn
	fn opcode_jp_nc_nn(&mut self, mmu: &mut MMU) {self.opcode_jp_cc_nn(mmu, "NC");}
	// JP C, nn
	fn opcode_jp_c_nn(&mut self, mmu: &mut MMU) {self.opcode_jp_cc_nn(mmu, "C");}

	// JR dd: Relative jump to dd (signed)
	fn opcode_jr_dd(&mut self, mmu: &mut MMU) {
		let dd = self.fetch_byte(mmu);
		let signed_value = dd as i8;
		self.pc = self.pc.wrapping_add_signed(signed_value as i16);
		#[cfg(debug_assertions)] // Detect and exit from infinite loop
		if signed_value == -2 {
			process::exit(0);
		}
		self.mcycles = 3;
	}

	// JR cc, dd: Relative jump to dd (signed) if condition is met
	fn opcode_jr_cc_dd(&mut self, mmu: &mut MMU, cc: &str) {
		let dd = self.fetch_byte(mmu);
		let signed_value = dd as i8;
		let condition = match cc {
			"NZ" => self.get_flag('z') == 0,
			"Z"  => self.get_flag('z') == 1,
			"NC" => self.get_flag('c') == 0,
			"C"  => self.get_flag('c') == 1,
			_ => panic!("JR cc, dd"),
		};
		if condition == true {
			self.pc = self.pc.wrapping_add_signed(signed_value as i16);
			self.mcycles = 3;
		}
		else {
			self.mcycles = 2;
		}
	}
	// JR NZ, dd
	fn opcode_jr_nz_dd(&mut self, mmu: &mut MMU) {self.opcode_jr_cc_dd(mmu, "NZ");}
	// JR Z, dd
	fn opcode_jr_z_dd(&mut self, mmu: &mut MMU) {self.opcode_jr_cc_dd(mmu, "Z");}
	// JR NC, dd
	fn opcode_jr_nc_dd(&mut self, mmu: &mut MMU) {self.opcode_jr_cc_dd(mmu, "NC");}
	// JR C, dd
	fn opcode_jr_c_dd(&mut self, mmu: &mut MMU) {self.opcode_jr_cc_dd(mmu, "C");}

	// CALL nn: Call subroutine at nn
	fn opcode_call_nn(&mut self, mmu: &mut MMU) {
		let nn = self.fetch_word(mmu);
		self.push_stack(mmu, self.pc);
		self.pc = nn;
		self.mcycles = 6;
	}

	// CALL cc, nn: Call subroutine at nn if condition is met
	fn opcode_call_cc_nn(&mut self, mmu: &mut MMU, cc: &str) {
		let nn = self.fetch_word(mmu);
		let condition = match cc {
			"NZ" => self.get_flag('z') == 0,
			"Z"  => self.get_flag('z') == 1,
			"NC" => self.get_flag('c') == 0,
			"C"  => self.get_flag('c') == 1,
			_ => panic!("CALL cc, nn"),
		};

		if condition == true {
			self.push_stack(mmu, self.pc);
			self.pc = nn;
			self.mcycles = 6;
		}
		else {
			self.mcycles = 3;
		}
	}
	// CALL NZ, nn
	fn opcode_call_nz_nn(&mut self, mmu: &mut MMU) {self.opcode_call_cc_nn(mmu, "NZ");}
	// CALL Z, nn
	fn opcode_call_z_nn(&mut self, mmu: &mut MMU) {self.opcode_call_cc_nn(mmu, "Z");}
	// CALL NC, nn
	fn opcode_call_nc_nn(&mut self, mmu: &mut MMU) {self.opcode_call_cc_nn(mmu, "NC");}
	// CALL C, nn
	fn opcode_call_c_nn(&mut self, mmu: &mut MMU) {self.opcode_call_cc_nn(mmu, "C");}

	// RET: Return from subroutine
	fn opcode_ret(&mut self, mmu: &mut MMU) {
		self.pc = self.pop_stack(mmu);
		self.mcycles = 4;
	}

	// RET cc: Retrun from subroutine if condition is met
	fn opcode_ret_cc(&mut self, mmu: &mut MMU, cc: &str) {
		let condition = match cc {
			"NZ" => self.get_flag('z') == 0,
			"Z"  => self.get_flag('z') == 1,
			"NC" => self.get_flag('c') == 0,
			"C"  => self.get_flag('c')== 1,
			_ => panic!("CALL cc, nn"),
		};

		if condition == true {
			self.pc = self.pop_stack(mmu);
			self.mcycles = 5;
		}
		else {
			self.mcycles = 2;
		}
	}
	// RET Z
	fn opcode_ret_z(&mut self, mmu: &mut MMU) {self.opcode_ret_cc(mmu, "Z");}
	// RET NZ
	fn opcode_ret_nz(&mut self, mmu: &mut MMU) {self.opcode_ret_cc(mmu, "NZ");}
	// RET NC
	fn opcode_ret_nc(&mut self, mmu: &mut MMU) {self.opcode_ret_cc(mmu, "NC");}
	// RET C
	fn opcode_ret_c(&mut self, mmu: &mut MMU) {self.opcode_ret_cc(mmu, "C");}


	// RETI: Return and enable interrupts
	fn opcode_reti(&mut self, mmu: &mut MMU) {
		self.ime = 1;
		self.pc = self.pop_stack(mmu);
		self.mcycles = 4;
	}

	// RST n: Call specific addresses
	// TODO: Check PC+2
	fn opcode_rst_n(&mut self, mmu: &mut MMU, n: u8) {
		self.push_stack(mmu, self.pc);
		self.pc = n as u16;
	}
	// RST 0
	fn opcode_rst_0(&mut self, mmu: &mut MMU) {self.opcode_rst_n(mmu, 0x00);}
	// RST 1
	fn opcode_rst_1(&mut self, mmu: &mut MMU) {self.opcode_rst_n(mmu, 0x08);}
	// RST 2
	fn opcode_rst_2(&mut self, mmu: &mut MMU) {self.opcode_rst_n(mmu, 0x10);}
	// RST 3
	fn opcode_rst_3(&mut self, mmu: &mut MMU) {self.opcode_rst_n(mmu, 0x18);}
	// RST 4
	fn opcode_rst_4(&mut self, mmu: &mut MMU) {self.opcode_rst_n(mmu, 0x20);}
	// RST 5
	fn opcode_rst_5(&mut self, mmu: &mut MMU) {self.opcode_rst_n(mmu, 0x28);}
	// RST 6
	fn opcode_rst_6(&mut self, mmu: &mut MMU) {self.opcode_rst_n(mmu, 0x30);}
	// RST 7
	fn opcode_rst_7(&mut self, mmu: &mut MMU) {self.opcode_rst_n(mmu, 0x38);}

	// // Unitiliazed opcode
	// fn opcode_unitialized(&mut self, _mmu: &mut MMU) {
	// 	println!("Unitialized opcode");
	// }
}
