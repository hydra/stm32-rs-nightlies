///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CRYP control register
    pub cryp_cr: CRYP_CR,
    ///0x04 - CRYP status register
    pub cryp_sr: CRYP_SR,
    ///0x08 - The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.
    pub cryp_din: CRYP_DIN,
    ///0x0c - The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.
    pub cryp_dout: CRYP_DOUT,
    ///0x10 - CRYP DMA control register
    pub cryp_dmacr: CRYP_DMACR,
    ///0x14 - The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.
    pub cryp_imscr: CRYP_IMSCR,
    ///0x18 - The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.
    pub cryp_risr: CRYP_RISR,
    ///0x1c - The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.
    pub cryp_misr: CRYP_MISR,
    ///0x20 - CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)
    pub cryp_k0lr: CRYP_K0LR,
    ///0x24 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    pub cryp_k0rr: CRYP_K0RR,
    ///0x28 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    pub cryp_k1lr: CRYP_K1LR,
    ///0x2c - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    pub cryp_k1rr: CRYP_K1RR,
    ///0x30 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    pub cryp_k2lr: CRYP_K2LR,
    ///0x34 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    pub cryp_k2rr: CRYP_K2RR,
    ///0x38 - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    pub cryp_k3lr: CRYP_K3LR,
    ///0x3c - Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
    pub cryp_k3rr: CRYP_K3RR,
    ///0x40 - The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).
    pub cryp_iv0lr: CRYP_IV0LR,
    ///0x44 - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
    pub cryp_iv0rr: CRYP_IV0RR,
    ///0x48 - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
    pub cryp_iv1lr: CRYP_IV1LR,
    ///0x4c - Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
    pub cryp_iv1rr: CRYP_IV1RR,
    ///0x50 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    pub cryp_csgcmccm0r: CRYP_CSGCMCCM0R,
    ///0x54 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    pub cryp_csgcmccm1r: CRYP_CSGCMCCM1R,
    ///0x58 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    pub cryp_csgcmccm2r: CRYP_CSGCMCCM2R,
    ///0x5c - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    pub cryp_csgcmccm3r: CRYP_CSGCMCCM3R,
    ///0x60 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    pub cryp_csgcmccm4r: CRYP_CSGCMCCM4R,
    ///0x64 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    pub cryp_csgcmccm5r: CRYP_CSGCMCCM5R,
    ///0x68 - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    pub cryp_csgcmccm6r: CRYP_CSGCMCCM6R,
    ///0x6c - These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
    pub cryp_csgcmccm7r: CRYP_CSGCMCCM7R,
    ///0x70 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    pub cryp_csgcm0r: CRYP_CSGCM0R,
    ///0x74 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    pub cryp_csgcm1r: CRYP_CSGCM1R,
    ///0x78 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    pub cryp_csgcm2r: CRYP_CSGCM2R,
    ///0x7c - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    pub cryp_csgcm3r: CRYP_CSGCM3R,
    ///0x80 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    pub cryp_csgcm4r: CRYP_CSGCM4R,
    ///0x84 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    pub cryp_csgcm5r: CRYP_CSGCM5R,
    ///0x88 - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    pub cryp_csgcm6r: CRYP_CSGCM6R,
    ///0x8c - Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
    pub cryp_csgcm7r: CRYP_CSGCM7R,
    _reserved36: [u8; 0x0360],
    ///0x3f0 - CRYP hardware configuration register
    pub cryp_hwcfgr: CRYP_HWCFGR,
    ///0x3f4 - CRYP HW Version Register
    pub cryp_verr: CRYP_VERR,
    ///0x3f8 - CRYP Identification
    pub cryp_ipidr: CRYP_IPIDR,
    ///0x3fc - CRYP HW Magic ID
    pub cryp_mid: CRYP_MID,
}
///CRYP_CR (rw) register accessor: an alias for `Reg<CRYP_CR_SPEC>`
pub type CRYP_CR = crate::Reg<cryp_cr::CRYP_CR_SPEC>;
///CRYP control register
pub mod cryp_cr;
///CRYP_SR (r) register accessor: an alias for `Reg<CRYP_SR_SPEC>`
pub type CRYP_SR = crate::Reg<cryp_sr::CRYP_SR_SPEC>;
///CRYP status register
pub mod cryp_sr;
///CRYP_DIN (rw) register accessor: an alias for `Reg<CRYP_DIN_SPEC>`
pub type CRYP_DIN = crate::Reg<cryp_din::CRYP_DIN_SPEC>;
///The CRYP_DIN register is the data input register. It is 32-bit wide. It is used to enter into the input FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DIN register is written to the data are pushed into the input FIFO. If CRYPEN = 1, when at least two 32-bit words in the DES/TDES mode have been pushed into the input FIFO (four words in the AES mode), and when at least two words are free in the output FIFO (four words in the AES mode), the CRYP engine starts an encrypting or decrypting process. When CRYP_DIN register is read: If CRYPEN = 0, the FIFO is popped, and then the data present in the Input FIFO are returned, from the oldest one (first reading) to the newest one (last reading). The IFEM flag must be checked before each read operation to make sure that the FIFO is not empty. if CRYPEN = 1, an undefined value is returned. After the CRYP_DIN register has been read once or several times, the FIFO must be flushed by setting the FFLUSH bit prior to processing new data.
pub mod cryp_din;
///CRYP_DOUT (r) register accessor: an alias for `Reg<CRYP_DOUT_SPEC>`
pub type CRYP_DOUT = crate::Reg<cryp_dout::CRYP_DOUT_SPEC>;
///The CRYP_DOUT register is the data output register. It is read-only and 32-bit wide. It is used to retrieve from the output FIFO up to four 64-bit blocks (TDES) or two 128-bit blocks (AES) of plaintext (when encrypting) or ciphertext (when decrypting), one 32-bit word at a time. To fit different data sizes, the data can be swapped after processing by configuring the DATATYPE bits in the CRYP_CR register. Refer to Section39.3.16: CRYP data registers and data swapping for more details. When CRYP_DOUT register is read, the last data entered into the output FIFO (pointed to by the read pointer) is returned.
pub mod cryp_dout;
///CRYP_DMACR (rw) register accessor: an alias for `Reg<CRYP_DMACR_SPEC>`
pub type CRYP_DMACR = crate::Reg<cryp_dmacr::CRYP_DMACR_SPEC>;
///CRYP DMA control register
pub mod cryp_dmacr;
///CRYP_IMSCR (rw) register accessor: an alias for `Reg<CRYP_IMSCR_SPEC>`
pub type CRYP_IMSCR = crate::Reg<cryp_imscr::CRYP_IMSCR_SPEC>;
///The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.
pub mod cryp_imscr;
///CRYP_RISR (r) register accessor: an alias for `Reg<CRYP_RISR_SPEC>`
pub type CRYP_RISR = crate::Reg<cryp_risr::CRYP_RISR_SPEC>;
///The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.
pub mod cryp_risr;
///CRYP_MISR (r) register accessor: an alias for `Reg<CRYP_MISR_SPEC>`
pub type CRYP_MISR = crate::Reg<cryp_misr::CRYP_MISR_SPEC>;
///The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.
pub mod cryp_misr;
///CRYP_K0LR (w) register accessor: an alias for `Reg<CRYP_K0LR_SPEC>`
pub type CRYP_K0LR = crate::Reg<cryp_k0lr::CRYP_K0LR_SPEC>;
///CRYP key registers contain the cryptographic keys. In DES/TDES mode, the keys are 64-bit binary values (number from left to right, that is the leftmost bit is bit 1) and named K1, K2 and K3 (K0 is not used). Each key consists of 56 information bits and 8 parity bits. In AES mode, the key is considered as a single 128, 192 or 256 bits long sequence K0K1K2...K127/191/255. The AES key is entered into the registers as follows: for AES-128: K0..K127 corresponds to b127..b0 (b255..b128 are not used), for AES-192: K0..K191 corresponds to b191..b0 (b255..b192 are not used), for AES-256: K0..K255 corresponds to b255..b0. In all cases key bit K0 is the leftmost bit in CRYP inner memory and register bit b0 is the rightmost bit in corresponding CRYP_KxLR key register. For more information refer to Section39.3.17: CRYP key registers. Write accesses to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register)
pub mod cryp_k0lr;
///CRYP_K0RR (w) register accessor: an alias for `Reg<CRYP_K0RR_SPEC>`
pub type CRYP_K0RR = crate::Reg<cryp_k0rr::CRYP_K0RR_SPEC>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod cryp_k0rr;
///CRYP_K1LR (w) register accessor: an alias for `Reg<CRYP_K1LR_SPEC>`
pub type CRYP_K1LR = crate::Reg<cryp_k1lr::CRYP_K1LR_SPEC>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod cryp_k1lr;
///CRYP_K1RR (w) register accessor: an alias for `Reg<CRYP_K1RR_SPEC>`
pub type CRYP_K1RR = crate::Reg<cryp_k1rr::CRYP_K1RR_SPEC>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod cryp_k1rr;
///CRYP_K2LR (w) register accessor: an alias for `Reg<CRYP_K2LR_SPEC>`
pub type CRYP_K2LR = crate::Reg<cryp_k2lr::CRYP_K2LR_SPEC>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod cryp_k2lr;
///CRYP_K2RR (w) register accessor: an alias for `Reg<CRYP_K2RR_SPEC>`
pub type CRYP_K2RR = crate::Reg<cryp_k2rr::CRYP_K2RR_SPEC>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod cryp_k2rr;
///CRYP_K3LR (w) register accessor: an alias for `Reg<CRYP_K3LR_SPEC>`
pub type CRYP_K3LR = crate::Reg<cryp_k3lr::CRYP_K3LR_SPEC>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod cryp_k3lr;
///CRYP_K3RR (w) register accessor: an alias for `Reg<CRYP_K3RR_SPEC>`
pub type CRYP_K3RR = crate::Reg<cryp_k3rr::CRYP_K3RR_SPEC>;
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
pub mod cryp_k3rr;
///CRYP_IV0LR (rw) register accessor: an alias for `Reg<CRYP_IV0LR_SPEC>`
pub type CRYP_IV0LR = crate::Reg<cryp_iv0lr::CRYP_IV0LR_SPEC>;
///The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).
pub mod cryp_iv0lr;
///CRYP_IV0RR (rw) register accessor: an alias for `Reg<CRYP_IV0RR_SPEC>`
pub type CRYP_IV0RR = crate::Reg<cryp_iv0rr::CRYP_IV0RR_SPEC>;
///Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
pub mod cryp_iv0rr;
///CRYP_IV1LR (rw) register accessor: an alias for `Reg<CRYP_IV1LR_SPEC>`
pub type CRYP_IV1LR = crate::Reg<cryp_iv1lr::CRYP_IV1LR_SPEC>;
///Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
pub mod cryp_iv1lr;
///CRYP_IV1RR (rw) register accessor: an alias for `Reg<CRYP_IV1RR_SPEC>`
pub type CRYP_IV1RR = crate::Reg<cryp_iv1rr::CRYP_IV1RR_SPEC>;
///Refer to Section39.6.17: CRYP initialization vector register 0L (CRYP_IV0LR) for details.
pub mod cryp_iv1rr;
///CRYP_CSGCMCCM0R (rw) register accessor: an alias for `Reg<CRYP_CSGCMCCM0R_SPEC>`
pub type CRYP_CSGCMCCM0R = crate::Reg<cryp_csgcmccm0r::CRYP_CSGCMCCM0R_SPEC>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod cryp_csgcmccm0r;
///CRYP_CSGCMCCM1R (rw) register accessor: an alias for `Reg<CRYP_CSGCMCCM1R_SPEC>`
pub type CRYP_CSGCMCCM1R = crate::Reg<cryp_csgcmccm1r::CRYP_CSGCMCCM1R_SPEC>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod cryp_csgcmccm1r;
///CRYP_CSGCMCCM2R (rw) register accessor: an alias for `Reg<CRYP_CSGCMCCM2R_SPEC>`
pub type CRYP_CSGCMCCM2R = crate::Reg<cryp_csgcmccm2r::CRYP_CSGCMCCM2R_SPEC>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod cryp_csgcmccm2r;
///CRYP_CSGCMCCM3R (rw) register accessor: an alias for `Reg<CRYP_CSGCMCCM3R_SPEC>`
pub type CRYP_CSGCMCCM3R = crate::Reg<cryp_csgcmccm3r::CRYP_CSGCMCCM3R_SPEC>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod cryp_csgcmccm3r;
///CRYP_CSGCMCCM4R (rw) register accessor: an alias for `Reg<CRYP_CSGCMCCM4R_SPEC>`
pub type CRYP_CSGCMCCM4R = crate::Reg<cryp_csgcmccm4r::CRYP_CSGCMCCM4R_SPEC>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod cryp_csgcmccm4r;
///CRYP_CSGCMCCM5R (rw) register accessor: an alias for `Reg<CRYP_CSGCMCCM5R_SPEC>`
pub type CRYP_CSGCMCCM5R = crate::Reg<cryp_csgcmccm5r::CRYP_CSGCMCCM5R_SPEC>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod cryp_csgcmccm5r;
///CRYP_CSGCMCCM6R (rw) register accessor: an alias for `Reg<CRYP_CSGCMCCM6R_SPEC>`
pub type CRYP_CSGCMCCM6R = crate::Reg<cryp_csgcmccm6r::CRYP_CSGCMCCM6R_SPEC>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod cryp_csgcmccm6r;
///CRYP_CSGCMCCM7R (rw) register accessor: an alias for `Reg<CRYP_CSGCMCCM7R_SPEC>`
pub type CRYP_CSGCMCCM7R = crate::Reg<cryp_csgcmccm7r::CRYP_CSGCMCCM7R_SPEC>;
///These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.
pub mod cryp_csgcmccm7r;
///CRYP_CSGCM0R (rw) register accessor: an alias for `Reg<CRYP_CSGCM0R_SPEC>`
pub type CRYP_CSGCM0R = crate::Reg<cryp_csgcm0r::CRYP_CSGCM0R_SPEC>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod cryp_csgcm0r;
///CRYP_CSGCM1R (rw) register accessor: an alias for `Reg<CRYP_CSGCM1R_SPEC>`
pub type CRYP_CSGCM1R = crate::Reg<cryp_csgcm1r::CRYP_CSGCM1R_SPEC>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod cryp_csgcm1r;
///CRYP_CSGCM2R (rw) register accessor: an alias for `Reg<CRYP_CSGCM2R_SPEC>`
pub type CRYP_CSGCM2R = crate::Reg<cryp_csgcm2r::CRYP_CSGCM2R_SPEC>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod cryp_csgcm2r;
///CRYP_CSGCM3R (rw) register accessor: an alias for `Reg<CRYP_CSGCM3R_SPEC>`
pub type CRYP_CSGCM3R = crate::Reg<cryp_csgcm3r::CRYP_CSGCM3R_SPEC>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod cryp_csgcm3r;
///CRYP_CSGCM4R (rw) register accessor: an alias for `Reg<CRYP_CSGCM4R_SPEC>`
pub type CRYP_CSGCM4R = crate::Reg<cryp_csgcm4r::CRYP_CSGCM4R_SPEC>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod cryp_csgcm4r;
///CRYP_CSGCM5R (rw) register accessor: an alias for `Reg<CRYP_CSGCM5R_SPEC>`
pub type CRYP_CSGCM5R = crate::Reg<cryp_csgcm5r::CRYP_CSGCM5R_SPEC>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod cryp_csgcm5r;
///CRYP_CSGCM6R (rw) register accessor: an alias for `Reg<CRYP_CSGCM6R_SPEC>`
pub type CRYP_CSGCM6R = crate::Reg<cryp_csgcm6r::CRYP_CSGCM6R_SPEC>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod cryp_csgcm6r;
///CRYP_CSGCM7R (rw) register accessor: an alias for `Reg<CRYP_CSGCM7R_SPEC>`
pub type CRYP_CSGCM7R = crate::Reg<cryp_csgcm7r::CRYP_CSGCM7R_SPEC>;
///Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.
pub mod cryp_csgcm7r;
///CRYP_HWCFGR (r) register accessor: an alias for `Reg<CRYP_HWCFGR_SPEC>`
pub type CRYP_HWCFGR = crate::Reg<cryp_hwcfgr::CRYP_HWCFGR_SPEC>;
///CRYP hardware configuration register
pub mod cryp_hwcfgr;
///CRYP_VERR (r) register accessor: an alias for `Reg<CRYP_VERR_SPEC>`
pub type CRYP_VERR = crate::Reg<cryp_verr::CRYP_VERR_SPEC>;
///CRYP HW Version Register
pub mod cryp_verr;
///CRYP_IPIDR (r) register accessor: an alias for `Reg<CRYP_IPIDR_SPEC>`
pub type CRYP_IPIDR = crate::Reg<cryp_ipidr::CRYP_IPIDR_SPEC>;
///CRYP Identification
pub mod cryp_ipidr;
///CRYP_MID (r) register accessor: an alias for `Reg<CRYP_MID_SPEC>`
pub type CRYP_MID = crate::Reg<cryp_mid::CRYP_MID_SPEC>;
///CRYP HW Magic ID
pub mod cryp_mid;
