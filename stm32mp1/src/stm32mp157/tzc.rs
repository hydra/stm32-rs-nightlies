///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Provides information about TZC configuration.
    pub tzc_build_config: TZC_BUILD_CONFIG,
    ///0x04 - Controls interrupt and bus error response behavior when regions permission failures occur.
    pub tzc_action: TZC_ACTION,
    ///0x08 - Provides control and status for the gate keeper in each filter unit implemented.
    pub tzc_gate_keeper: TZC_GATE_KEEPER,
    ///0x0c - Controls read and write access speculation.
    pub tzc_speculation_ctrl: TZC_SPECULATION_CTRL,
    ///0x10 - Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.
    pub tzc_int_status: TZC_INT_STATUS,
    ///0x14 - Interrupt clear for each filter.
    pub tzc_int_clear: TZC_INT_CLEAR,
    _reserved6: [u8; 0x08],
    ///0x20 - Address low bits of the first failed access in the associated filter (0 to 1).
    pub tzc_fail_address_low0: TZC_FAIL_ADDRESS_LOW0,
    ///0x24 - Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
    pub tzc_fail_address_high0: TZC_FAIL_ADDRESS_HIGH0,
    ///0x28 - Status information about the first access that failed a region permission check in the associated filter (0 to 1).
    pub tzc_fail_control0: TZC_FAIL_CONTROL0,
    ///0x2c - Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
    pub tzc_fail_id0: TZC_FAIL_ID0,
    ///0x30 - Address low bits of the first failed access in the associated filter (0 to 1).
    pub tzc_fail_address_low1: TZC_FAIL_ADDRESS_LOW1,
    ///0x34 - Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
    pub tzc_fail_address_high1: TZC_FAIL_ADDRESS_HIGH1,
    ///0x38 - Status information about the first access that failed a region permission check in the associated filter (0 to 1).
    pub tzc_fail_control1: TZC_FAIL_CONTROL1,
    ///0x3c - Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
    pub tzc_fail_id1: TZC_FAIL_ID1,
    _reserved14: [u8; 0xc4],
    ///0x104 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high0: TZC_REGION_BASE_HIGH0,
    ///0x108 - Top address bits \[31:12\]
    ///for region 0.
    pub tzc_region_top_low0: TZC_REGION_TOP_LOW0,
    ///0x10c - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high0: TZC_REGION_TOP_HIGH0,
    ///0x110 - Region 0 attributes.
    pub tzc_region_attribute0: TZC_REGION_ATTRIBUTE0,
    ///0x114 - Region non-secure access based on NSAID.
    pub tzc_region_id_access0: TZC_REGION_ID_ACCESS0,
    _reserved19: [u8; 0x08],
    ///0x120 - Base address low for regions 1 to 8.
    pub tzc_region_base_low1: TZC_REGION_BASE_LOW1,
    ///0x124 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high1: TZC_REGION_BASE_HIGH1,
    ///0x128 - Top address bits \[31:12\]
    ///for region x.
    pub tzc_region_top_low1: TZC_REGION_TOP_LOW1,
    ///0x12c - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high1: TZC_REGION_TOP_HIGH1,
    ///0x130 - Region x attributes.
    pub tzc_region_attribute1: TZC_REGION_ATTRIBUTE1,
    ///0x134 - Region non-secure access based on NSAID.
    pub tzc_region_id_access1: TZC_REGION_ID_ACCESS1,
    _reserved25: [u8; 0x08],
    ///0x140 - Base address low for regions 1 to 8.
    pub tzc_region_base_low2: TZC_REGION_BASE_LOW2,
    ///0x144 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high2: TZC_REGION_BASE_HIGH2,
    ///0x148 - Top address bits \[31:12\]
    ///for region x.
    pub tzc_region_top_low2: TZC_REGION_TOP_LOW2,
    ///0x14c - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high2: TZC_REGION_TOP_HIGH2,
    ///0x150 - Region x attributes.
    pub tzc_region_attribute2: TZC_REGION_ATTRIBUTE2,
    ///0x154 - Region non-secure access based on NSAID.
    pub tzc_region_id_access2: TZC_REGION_ID_ACCESS2,
    _reserved31: [u8; 0x08],
    ///0x160 - Base address low for regions 1 to 8.
    pub tzc_region_base_low3: TZC_REGION_BASE_LOW3,
    ///0x164 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high3: TZC_REGION_BASE_HIGH3,
    ///0x168 - Top address bits \[31:12\]
    ///for region x.
    pub tzc_region_top_low3: TZC_REGION_TOP_LOW3,
    ///0x16c - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high3: TZC_REGION_TOP_HIGH3,
    ///0x170 - Region x attributes.
    pub tzc_region_attribute3: TZC_REGION_ATTRIBUTE3,
    ///0x174 - Region non-secure access based on NSAID.
    pub tzc_region_id_access3: TZC_REGION_ID_ACCESS3,
    _reserved37: [u8; 0x08],
    ///0x180 - Base address low for regions 1 to 8.
    pub tzc_region_base_low4: TZC_REGION_BASE_LOW4,
    ///0x184 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high4: TZC_REGION_BASE_HIGH4,
    ///0x188 - Top address bits \[31:12\]
    ///for region x.
    pub tzc_region_top_low4: TZC_REGION_TOP_LOW4,
    ///0x18c - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high4: TZC_REGION_TOP_HIGH4,
    ///0x190 - Region x attributes.
    pub tzc_region_attribute4: TZC_REGION_ATTRIBUTE4,
    ///0x194 - Region non-secure access based on NSAID.
    pub tzc_region_id_access4: TZC_REGION_ID_ACCESS4,
    _reserved43: [u8; 0x08],
    ///0x1a0 - Base address low for regions 1 to 8.
    pub tzc_region_base_low5: TZC_REGION_BASE_LOW5,
    ///0x1a4 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high5: TZC_REGION_BASE_HIGH5,
    ///0x1a8 - Top address bits \[31:12\]
    ///for region x.
    pub tzc_region_top_low5: TZC_REGION_TOP_LOW5,
    ///0x1ac - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high5: TZC_REGION_TOP_HIGH5,
    ///0x1b0 - Region x attributes.
    pub tzc_region_attribute5: TZC_REGION_ATTRIBUTE5,
    ///0x1b4 - Region non-secure access based on NSAID.
    pub tzc_region_id_access5: TZC_REGION_ID_ACCESS5,
    _reserved49: [u8; 0x08],
    ///0x1c0 - Base address low for regions 1 to 8.
    pub tzc_region_base_low6: TZC_REGION_BASE_LOW6,
    ///0x1c4 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high6: TZC_REGION_BASE_HIGH6,
    ///0x1c8 - Top address bits \[31:12\]
    ///for region x.
    pub tzc_region_top_low6: TZC_REGION_TOP_LOW6,
    ///0x1cc - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high6: TZC_REGION_TOP_HIGH6,
    ///0x1d0 - Region x attributes.
    pub tzc_region_attribute6: TZC_REGION_ATTRIBUTE6,
    ///0x1d4 - Region non-secure access based on NSAID.
    pub tzc_region_id_access6: TZC_REGION_ID_ACCESS6,
    _reserved55: [u8; 0x10],
    ///0x1e8 - Top address bits \[31:12\]
    ///for region x.
    pub tzc_region_top_low7: TZC_REGION_TOP_LOW7,
    _reserved56: [u8; 0x04],
    ///0x1f0 - Region x attributes.
    pub tzc_region_attribute7: TZC_REGION_ATTRIBUTE7,
    _reserved57: [u8; 0x0c],
    ///0x200 - Base address low for regions 1 to 8.
    pub tzc_region_base_low8: TZC_REGION_BASE_LOW8,
    ///0x204 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high8: TZC_REGION_BASE_HIGH8,
    _reserved59: [u8; 0x08],
    ///0x210 - Region x attributes.
    pub tzc_region_attribute8: TZC_REGION_ATTRIBUTE8,
    _reserved60: [u8; 0xcc],
    ///0x2e0 - Base address low for regions 1 to 8.
    pub tzc_region_base_low7: TZC_REGION_BASE_LOW7,
    ///0x2e4 - Base address high are not used with 32-bit address.
    pub tzc_region_base_high7: TZC_REGION_BASE_HIGH7,
    _reserved62: [u8; 0x04],
    ///0x2ec - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high7: TZC_REGION_TOP_HIGH7,
    _reserved63: [u8; 0x04],
    ///0x2f4 - Region non-secure access based on NSAID.
    pub tzc_region_id_access7: TZC_REGION_ID_ACCESS7,
    _reserved64: [u8; 0x10],
    ///0x308 - Top address bits \[31:12\]
    ///for region x.
    pub tzc_region_top_low8: TZC_REGION_TOP_LOW8,
    ///0x30c - Top address high of region are not used with 32-bit address.
    pub tzc_region_top_high8: TZC_REGION_TOP_HIGH8,
    _reserved66: [u8; 0x04],
    ///0x314 - Region non-secure access based on NSAID.
    pub tzc_region_id_access8: TZC_REGION_ID_ACCESS8,
    _reserved67: [u8; 0x0cb8],
    ///0xfd0 - Peripheral ID 4.
    pub tzc_pid4: TZC_PID4,
    ///0xfd4 - Peripheral ID 5.
    pub tzc_pid5: TZC_PID5,
    ///0xfd8 - Peripheral ID 6.
    pub tzc_pid6: TZC_PID6,
    ///0xfdc - Peripheral ID 7.
    pub tzc_pid7: TZC_PID7,
    ///0xfe0 - Peripheral ID 0.
    pub tzc_pid0: TZC_PID0,
    ///0xfe4 - Peripheral ID 1.
    pub tzc_pid1: TZC_PID1,
    ///0xfe8 - Peripheral ID 2.
    pub tzc_pid2: TZC_PID2,
    ///0xfec - Peripheral ID 3.
    pub tzc_pid3: TZC_PID3,
    ///0xff0 - Component ID 0.
    pub tzc_cid0: TZC_CID0,
    ///0xff4 - Component ID 1.
    pub tzc_cid1: TZC_CID1,
    ///0xff8 - Component ID 2.
    pub tzc_cid2: TZC_CID2,
    ///0xffc - Component ID 3.
    pub tzc_cid3: TZC_CID3,
}
///TZC_BUILD_CONFIG (r) register accessor: an alias for `Reg<TZC_BUILD_CONFIG_SPEC>`
pub type TZC_BUILD_CONFIG = crate::Reg<tzc_build_config::TZC_BUILD_CONFIG_SPEC>;
///Provides information about TZC configuration.
pub mod tzc_build_config;
///TZC_ACTION (rw) register accessor: an alias for `Reg<TZC_ACTION_SPEC>`
pub type TZC_ACTION = crate::Reg<tzc_action::TZC_ACTION_SPEC>;
///Controls interrupt and bus error response behavior when regions permission failures occur.
pub mod tzc_action;
///TZC_GATE_KEEPER (rw) register accessor: an alias for `Reg<TZC_GATE_KEEPER_SPEC>`
pub type TZC_GATE_KEEPER = crate::Reg<tzc_gate_keeper::TZC_GATE_KEEPER_SPEC>;
///Provides control and status for the gate keeper in each filter unit implemented.
pub mod tzc_gate_keeper;
///TZC_SPECULATION_CTRL (rw) register accessor: an alias for `Reg<TZC_SPECULATION_CTRL_SPEC>`
pub type TZC_SPECULATION_CTRL = crate::Reg<tzc_speculation_ctrl::TZC_SPECULATION_CTRL_SPEC>;
///Controls read and write access speculation.
pub mod tzc_speculation_ctrl;
///TZC_INT_STATUS (r) register accessor: an alias for `Reg<TZC_INT_STATUS_SPEC>`
pub type TZC_INT_STATUS = crate::Reg<tzc_int_status::TZC_INT_STATUS_SPEC>;
///Contains the status of the interrupt signal, TZCINT, that reports access security violations or region overlap errors.
pub mod tzc_int_status;
///TZC_INT_CLEAR (rw) register accessor: an alias for `Reg<TZC_INT_CLEAR_SPEC>`
pub type TZC_INT_CLEAR = crate::Reg<tzc_int_clear::TZC_INT_CLEAR_SPEC>;
///Interrupt clear for each filter.
pub mod tzc_int_clear;
///TZC_FAIL_CONTROL0 (r) register accessor: an alias for `Reg<TZC_FAIL_CONTROL0_SPEC>`
pub type TZC_FAIL_CONTROL0 = crate::Reg<tzc_fail_control0::TZC_FAIL_CONTROL0_SPEC>;
///Status information about the first access that failed a region permission check in the associated filter (0 to 1).
pub mod tzc_fail_control0;
///TZC_FAIL_ID0 (r) register accessor: an alias for `Reg<TZC_FAIL_ID0_SPEC>`
pub type TZC_FAIL_ID0 = crate::Reg<tzc_fail_id0::TZC_FAIL_ID0_SPEC>;
///Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
pub mod tzc_fail_id0;
///TZC_FAIL_CONTROL1 (r) register accessor: an alias for `Reg<TZC_FAIL_CONTROL1_SPEC>`
pub type TZC_FAIL_CONTROL1 = crate::Reg<tzc_fail_control1::TZC_FAIL_CONTROL1_SPEC>;
///Status information about the first access that failed a region permission check in the associated filter (0 to 1).
pub mod tzc_fail_control1;
///TZC_FAIL_ID1 (r) register accessor: an alias for `Reg<TZC_FAIL_ID1_SPEC>`
pub type TZC_FAIL_ID1 = crate::Reg<tzc_fail_id1::TZC_FAIL_ID1_SPEC>;
///Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).
pub mod tzc_fail_id1;
///TZC_REGION_ATTRIBUTE0 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE0_SPEC>`
pub type TZC_REGION_ATTRIBUTE0 = crate::Reg<tzc_region_attribute0::TZC_REGION_ATTRIBUTE0_SPEC>;
///Region 0 attributes.
pub mod tzc_region_attribute0;
///TZC_REGION_ATTRIBUTE1 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE1_SPEC>`
pub type TZC_REGION_ATTRIBUTE1 = crate::Reg<tzc_region_attribute1::TZC_REGION_ATTRIBUTE1_SPEC>;
///Region x attributes.
pub mod tzc_region_attribute1;
///TZC_REGION_ATTRIBUTE2 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE2_SPEC>`
pub type TZC_REGION_ATTRIBUTE2 = crate::Reg<tzc_region_attribute2::TZC_REGION_ATTRIBUTE2_SPEC>;
///Region x attributes.
pub mod tzc_region_attribute2;
///TZC_REGION_ATTRIBUTE3 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE3_SPEC>`
pub type TZC_REGION_ATTRIBUTE3 = crate::Reg<tzc_region_attribute3::TZC_REGION_ATTRIBUTE3_SPEC>;
///Region x attributes.
pub mod tzc_region_attribute3;
///TZC_REGION_ATTRIBUTE4 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE4_SPEC>`
pub type TZC_REGION_ATTRIBUTE4 = crate::Reg<tzc_region_attribute4::TZC_REGION_ATTRIBUTE4_SPEC>;
///Region x attributes.
pub mod tzc_region_attribute4;
///TZC_REGION_ATTRIBUTE5 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE5_SPEC>`
pub type TZC_REGION_ATTRIBUTE5 = crate::Reg<tzc_region_attribute5::TZC_REGION_ATTRIBUTE5_SPEC>;
///Region x attributes.
pub mod tzc_region_attribute5;
///TZC_REGION_ATTRIBUTE6 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE6_SPEC>`
pub type TZC_REGION_ATTRIBUTE6 = crate::Reg<tzc_region_attribute6::TZC_REGION_ATTRIBUTE6_SPEC>;
///Region x attributes.
pub mod tzc_region_attribute6;
///TZC_REGION_ATTRIBUTE7 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE7_SPEC>`
pub type TZC_REGION_ATTRIBUTE7 = crate::Reg<tzc_region_attribute7::TZC_REGION_ATTRIBUTE7_SPEC>;
///Region x attributes.
pub mod tzc_region_attribute7;
///TZC_REGION_ATTRIBUTE8 (rw) register accessor: an alias for `Reg<TZC_REGION_ATTRIBUTE8_SPEC>`
pub type TZC_REGION_ATTRIBUTE8 = crate::Reg<tzc_region_attribute8::TZC_REGION_ATTRIBUTE8_SPEC>;
///Region x attributes.
pub mod tzc_region_attribute8;
///TZC_PID4 (r) register accessor: an alias for `Reg<TZC_PID4_SPEC>`
pub type TZC_PID4 = crate::Reg<tzc_pid4::TZC_PID4_SPEC>;
///Peripheral ID 4.
pub mod tzc_pid4;
///TZC_PID5 (r) register accessor: an alias for `Reg<TZC_PID5_SPEC>`
pub type TZC_PID5 = crate::Reg<tzc_pid5::TZC_PID5_SPEC>;
///Peripheral ID 5.
pub mod tzc_pid5;
///TZC_PID6 (r) register accessor: an alias for `Reg<TZC_PID6_SPEC>`
pub type TZC_PID6 = crate::Reg<tzc_pid6::TZC_PID6_SPEC>;
///Peripheral ID 6.
pub mod tzc_pid6;
///TZC_PID7 (r) register accessor: an alias for `Reg<TZC_PID7_SPEC>`
pub type TZC_PID7 = crate::Reg<tzc_pid7::TZC_PID7_SPEC>;
///Peripheral ID 7.
pub mod tzc_pid7;
///TZC_PID0 (r) register accessor: an alias for `Reg<TZC_PID0_SPEC>`
pub type TZC_PID0 = crate::Reg<tzc_pid0::TZC_PID0_SPEC>;
///Peripheral ID 0.
pub mod tzc_pid0;
///TZC_PID1 (r) register accessor: an alias for `Reg<TZC_PID1_SPEC>`
pub type TZC_PID1 = crate::Reg<tzc_pid1::TZC_PID1_SPEC>;
///Peripheral ID 1.
pub mod tzc_pid1;
///TZC_PID2 (r) register accessor: an alias for `Reg<TZC_PID2_SPEC>`
pub type TZC_PID2 = crate::Reg<tzc_pid2::TZC_PID2_SPEC>;
///Peripheral ID 2.
pub mod tzc_pid2;
///TZC_PID3 (r) register accessor: an alias for `Reg<TZC_PID3_SPEC>`
pub type TZC_PID3 = crate::Reg<tzc_pid3::TZC_PID3_SPEC>;
///Peripheral ID 3.
pub mod tzc_pid3;
///TZC_CID0 (r) register accessor: an alias for `Reg<TZC_CID0_SPEC>`
pub type TZC_CID0 = crate::Reg<tzc_cid0::TZC_CID0_SPEC>;
///Component ID 0.
pub mod tzc_cid0;
///TZC_CID1 (r) register accessor: an alias for `Reg<TZC_CID1_SPEC>`
pub type TZC_CID1 = crate::Reg<tzc_cid1::TZC_CID1_SPEC>;
///Component ID 1.
pub mod tzc_cid1;
///TZC_CID2 (r) register accessor: an alias for `Reg<TZC_CID2_SPEC>`
pub type TZC_CID2 = crate::Reg<tzc_cid2::TZC_CID2_SPEC>;
///Component ID 2.
pub mod tzc_cid2;
///TZC_CID3 (r) register accessor: an alias for `Reg<TZC_CID3_SPEC>`
pub type TZC_CID3 = crate::Reg<tzc_cid3::TZC_CID3_SPEC>;
///Component ID 3.
pub mod tzc_cid3;
///TZC_FAIL_ADDRESS_LOW0 (r) register accessor: an alias for `Reg<TZC_FAIL_ADDRESS_LOW0_SPEC>`
pub type TZC_FAIL_ADDRESS_LOW0 = crate::Reg<tzc_fail_address_low0::TZC_FAIL_ADDRESS_LOW0_SPEC>;
///Address low bits of the first failed access in the associated filter (0 to 1).
pub mod tzc_fail_address_low0;
///TZC_FAIL_ADDRESS_HIGH0 (r) register accessor: an alias for `Reg<TZC_FAIL_ADDRESS_HIGH0_SPEC>`
pub type TZC_FAIL_ADDRESS_HIGH0 = crate::Reg<tzc_fail_address_high0::TZC_FAIL_ADDRESS_HIGH0_SPEC>;
///Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
pub mod tzc_fail_address_high0;
///TZC_FAIL_ADDRESS_LOW1 (r) register accessor: an alias for `Reg<TZC_FAIL_ADDRESS_LOW1_SPEC>`
pub type TZC_FAIL_ADDRESS_LOW1 = crate::Reg<tzc_fail_address_low1::TZC_FAIL_ADDRESS_LOW1_SPEC>;
///Address low bits of the first failed access in the associated filter (0 to 1).
pub mod tzc_fail_address_low1;
///TZC_FAIL_ADDRESS_HIGH1 (r) register accessor: an alias for `Reg<TZC_FAIL_ADDRESS_HIGH1_SPEC>`
pub type TZC_FAIL_ADDRESS_HIGH1 = crate::Reg<tzc_fail_address_high1::TZC_FAIL_ADDRESS_HIGH1_SPEC>;
///Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.
pub mod tzc_fail_address_high1;
///TZC_REGION_BASE_HIGH0 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH0_SPEC>`
pub type TZC_REGION_BASE_HIGH0 = crate::Reg<tzc_region_base_high0::TZC_REGION_BASE_HIGH0_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high0;
///TZC_REGION_TOP_LOW0 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW0_SPEC>`
pub type TZC_REGION_TOP_LOW0 = crate::Reg<tzc_region_top_low0::TZC_REGION_TOP_LOW0_SPEC>;
///Top address bits \[31:12\]
///for region 0.
pub mod tzc_region_top_low0;
///TZC_REGION_TOP_HIGH0 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH0_SPEC>`
pub type TZC_REGION_TOP_HIGH0 = crate::Reg<tzc_region_top_high0::TZC_REGION_TOP_HIGH0_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high0;
///TZC_REGION_ID_ACCESS0 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS0_SPEC>`
pub type TZC_REGION_ID_ACCESS0 = crate::Reg<tzc_region_id_access0::TZC_REGION_ID_ACCESS0_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access0;
///TZC_REGION_BASE_LOW1 (rw) register accessor: an alias for `Reg<TZC_REGION_BASE_LOW1_SPEC>`
pub type TZC_REGION_BASE_LOW1 = crate::Reg<tzc_region_base_low1::TZC_REGION_BASE_LOW1_SPEC>;
///Base address low for regions 1 to 8.
pub mod tzc_region_base_low1;
///TZC_REGION_BASE_HIGH1 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH1_SPEC>`
pub type TZC_REGION_BASE_HIGH1 = crate::Reg<tzc_region_base_high1::TZC_REGION_BASE_HIGH1_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high1;
///TZC_REGION_TOP_LOW1 (rw) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW1_SPEC>`
pub type TZC_REGION_TOP_LOW1 = crate::Reg<tzc_region_top_low1::TZC_REGION_TOP_LOW1_SPEC>;
///Top address bits \[31:12\]
///for region x.
pub mod tzc_region_top_low1;
///TZC_REGION_TOP_HIGH1 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH1_SPEC>`
pub type TZC_REGION_TOP_HIGH1 = crate::Reg<tzc_region_top_high1::TZC_REGION_TOP_HIGH1_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high1;
///TZC_REGION_ID_ACCESS1 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS1_SPEC>`
pub type TZC_REGION_ID_ACCESS1 = crate::Reg<tzc_region_id_access1::TZC_REGION_ID_ACCESS1_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access1;
///TZC_REGION_BASE_LOW2 (rw) register accessor: an alias for `Reg<TZC_REGION_BASE_LOW2_SPEC>`
pub type TZC_REGION_BASE_LOW2 = crate::Reg<tzc_region_base_low2::TZC_REGION_BASE_LOW2_SPEC>;
///Base address low for regions 1 to 8.
pub mod tzc_region_base_low2;
///TZC_REGION_BASE_HIGH2 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH2_SPEC>`
pub type TZC_REGION_BASE_HIGH2 = crate::Reg<tzc_region_base_high2::TZC_REGION_BASE_HIGH2_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high2;
///TZC_REGION_TOP_LOW2 (rw) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW2_SPEC>`
pub type TZC_REGION_TOP_LOW2 = crate::Reg<tzc_region_top_low2::TZC_REGION_TOP_LOW2_SPEC>;
///Top address bits \[31:12\]
///for region x.
pub mod tzc_region_top_low2;
///TZC_REGION_TOP_HIGH2 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH2_SPEC>`
pub type TZC_REGION_TOP_HIGH2 = crate::Reg<tzc_region_top_high2::TZC_REGION_TOP_HIGH2_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high2;
///TZC_REGION_ID_ACCESS2 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS2_SPEC>`
pub type TZC_REGION_ID_ACCESS2 = crate::Reg<tzc_region_id_access2::TZC_REGION_ID_ACCESS2_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access2;
///TZC_REGION_BASE_LOW3 (rw) register accessor: an alias for `Reg<TZC_REGION_BASE_LOW3_SPEC>`
pub type TZC_REGION_BASE_LOW3 = crate::Reg<tzc_region_base_low3::TZC_REGION_BASE_LOW3_SPEC>;
///Base address low for regions 1 to 8.
pub mod tzc_region_base_low3;
///TZC_REGION_BASE_HIGH3 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH3_SPEC>`
pub type TZC_REGION_BASE_HIGH3 = crate::Reg<tzc_region_base_high3::TZC_REGION_BASE_HIGH3_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high3;
///TZC_REGION_TOP_LOW3 (rw) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW3_SPEC>`
pub type TZC_REGION_TOP_LOW3 = crate::Reg<tzc_region_top_low3::TZC_REGION_TOP_LOW3_SPEC>;
///Top address bits \[31:12\]
///for region x.
pub mod tzc_region_top_low3;
///TZC_REGION_TOP_HIGH3 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH3_SPEC>`
pub type TZC_REGION_TOP_HIGH3 = crate::Reg<tzc_region_top_high3::TZC_REGION_TOP_HIGH3_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high3;
///TZC_REGION_ID_ACCESS3 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS3_SPEC>`
pub type TZC_REGION_ID_ACCESS3 = crate::Reg<tzc_region_id_access3::TZC_REGION_ID_ACCESS3_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access3;
///TZC_REGION_BASE_LOW4 (rw) register accessor: an alias for `Reg<TZC_REGION_BASE_LOW4_SPEC>`
pub type TZC_REGION_BASE_LOW4 = crate::Reg<tzc_region_base_low4::TZC_REGION_BASE_LOW4_SPEC>;
///Base address low for regions 1 to 8.
pub mod tzc_region_base_low4;
///TZC_REGION_BASE_HIGH4 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH4_SPEC>`
pub type TZC_REGION_BASE_HIGH4 = crate::Reg<tzc_region_base_high4::TZC_REGION_BASE_HIGH4_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high4;
///TZC_REGION_TOP_LOW4 (rw) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW4_SPEC>`
pub type TZC_REGION_TOP_LOW4 = crate::Reg<tzc_region_top_low4::TZC_REGION_TOP_LOW4_SPEC>;
///Top address bits \[31:12\]
///for region x.
pub mod tzc_region_top_low4;
///TZC_REGION_TOP_HIGH4 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH4_SPEC>`
pub type TZC_REGION_TOP_HIGH4 = crate::Reg<tzc_region_top_high4::TZC_REGION_TOP_HIGH4_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high4;
///TZC_REGION_ID_ACCESS4 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS4_SPEC>`
pub type TZC_REGION_ID_ACCESS4 = crate::Reg<tzc_region_id_access4::TZC_REGION_ID_ACCESS4_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access4;
///TZC_REGION_BASE_LOW5 (rw) register accessor: an alias for `Reg<TZC_REGION_BASE_LOW5_SPEC>`
pub type TZC_REGION_BASE_LOW5 = crate::Reg<tzc_region_base_low5::TZC_REGION_BASE_LOW5_SPEC>;
///Base address low for regions 1 to 8.
pub mod tzc_region_base_low5;
///TZC_REGION_BASE_HIGH5 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH5_SPEC>`
pub type TZC_REGION_BASE_HIGH5 = crate::Reg<tzc_region_base_high5::TZC_REGION_BASE_HIGH5_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high5;
///TZC_REGION_TOP_LOW5 (rw) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW5_SPEC>`
pub type TZC_REGION_TOP_LOW5 = crate::Reg<tzc_region_top_low5::TZC_REGION_TOP_LOW5_SPEC>;
///Top address bits \[31:12\]
///for region x.
pub mod tzc_region_top_low5;
///TZC_REGION_TOP_HIGH5 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH5_SPEC>`
pub type TZC_REGION_TOP_HIGH5 = crate::Reg<tzc_region_top_high5::TZC_REGION_TOP_HIGH5_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high5;
///TZC_REGION_ID_ACCESS5 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS5_SPEC>`
pub type TZC_REGION_ID_ACCESS5 = crate::Reg<tzc_region_id_access5::TZC_REGION_ID_ACCESS5_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access5;
///TZC_REGION_BASE_LOW6 (rw) register accessor: an alias for `Reg<TZC_REGION_BASE_LOW6_SPEC>`
pub type TZC_REGION_BASE_LOW6 = crate::Reg<tzc_region_base_low6::TZC_REGION_BASE_LOW6_SPEC>;
///Base address low for regions 1 to 8.
pub mod tzc_region_base_low6;
///TZC_REGION_BASE_HIGH6 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH6_SPEC>`
pub type TZC_REGION_BASE_HIGH6 = crate::Reg<tzc_region_base_high6::TZC_REGION_BASE_HIGH6_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high6;
///TZC_REGION_TOP_LOW6 (rw) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW6_SPEC>`
pub type TZC_REGION_TOP_LOW6 = crate::Reg<tzc_region_top_low6::TZC_REGION_TOP_LOW6_SPEC>;
///Top address bits \[31:12\]
///for region x.
pub mod tzc_region_top_low6;
///TZC_REGION_TOP_HIGH6 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH6_SPEC>`
pub type TZC_REGION_TOP_HIGH6 = crate::Reg<tzc_region_top_high6::TZC_REGION_TOP_HIGH6_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high6;
///TZC_REGION_ID_ACCESS6 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS6_SPEC>`
pub type TZC_REGION_ID_ACCESS6 = crate::Reg<tzc_region_id_access6::TZC_REGION_ID_ACCESS6_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access6;
///TZC_REGION_BASE_LOW7 (rw) register accessor: an alias for `Reg<TZC_REGION_BASE_LOW7_SPEC>`
pub type TZC_REGION_BASE_LOW7 = crate::Reg<tzc_region_base_low7::TZC_REGION_BASE_LOW7_SPEC>;
///Base address low for regions 1 to 8.
pub mod tzc_region_base_low7;
///TZC_REGION_BASE_HIGH7 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH7_SPEC>`
pub type TZC_REGION_BASE_HIGH7 = crate::Reg<tzc_region_base_high7::TZC_REGION_BASE_HIGH7_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high7;
///TZC_REGION_TOP_LOW7 (rw) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW7_SPEC>`
pub type TZC_REGION_TOP_LOW7 = crate::Reg<tzc_region_top_low7::TZC_REGION_TOP_LOW7_SPEC>;
///Top address bits \[31:12\]
///for region x.
pub mod tzc_region_top_low7;
///TZC_REGION_TOP_HIGH7 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH7_SPEC>`
pub type TZC_REGION_TOP_HIGH7 = crate::Reg<tzc_region_top_high7::TZC_REGION_TOP_HIGH7_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high7;
///TZC_REGION_ID_ACCESS7 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS7_SPEC>`
pub type TZC_REGION_ID_ACCESS7 = crate::Reg<tzc_region_id_access7::TZC_REGION_ID_ACCESS7_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access7;
///TZC_REGION_BASE_LOW8 (rw) register accessor: an alias for `Reg<TZC_REGION_BASE_LOW8_SPEC>`
pub type TZC_REGION_BASE_LOW8 = crate::Reg<tzc_region_base_low8::TZC_REGION_BASE_LOW8_SPEC>;
///Base address low for regions 1 to 8.
pub mod tzc_region_base_low8;
///TZC_REGION_BASE_HIGH8 (r) register accessor: an alias for `Reg<TZC_REGION_BASE_HIGH8_SPEC>`
pub type TZC_REGION_BASE_HIGH8 = crate::Reg<tzc_region_base_high8::TZC_REGION_BASE_HIGH8_SPEC>;
///Base address high are not used with 32-bit address.
pub mod tzc_region_base_high8;
///TZC_REGION_TOP_LOW8 (rw) register accessor: an alias for `Reg<TZC_REGION_TOP_LOW8_SPEC>`
pub type TZC_REGION_TOP_LOW8 = crate::Reg<tzc_region_top_low8::TZC_REGION_TOP_LOW8_SPEC>;
///Top address bits \[31:12\]
///for region x.
pub mod tzc_region_top_low8;
///TZC_REGION_TOP_HIGH8 (r) register accessor: an alias for `Reg<TZC_REGION_TOP_HIGH8_SPEC>`
pub type TZC_REGION_TOP_HIGH8 = crate::Reg<tzc_region_top_high8::TZC_REGION_TOP_HIGH8_SPEC>;
///Top address high of region are not used with 32-bit address.
pub mod tzc_region_top_high8;
///TZC_REGION_ID_ACCESS8 (rw) register accessor: an alias for `Reg<TZC_REGION_ID_ACCESS8_SPEC>`
pub type TZC_REGION_ID_ACCESS8 = crate::Reg<tzc_region_id_access8::TZC_REGION_ID_ACCESS8_SPEC>;
///Region non-secure access based on NSAID.
pub mod tzc_region_id_access8;
