///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - BSEC OTP configuration register
    pub bsec_otp_config: BSEC_OTP_CONFIG,
    ///0x04 - BSEC OTP control register
    pub bsec_otp_control: BSEC_OTP_CONTROL,
    ///0x08 - BSEC OTP write data register
    pub bsec_otp_wrdata: BSEC_OTP_WRDATA,
    ///0x0c - BSEC OTP status register
    pub bsec_otp_status: BSEC_OTP_STATUS,
    ///0x10 - BSEC OTP lock configuration register
    pub bsec_otp_lock: BSEC_OTP_LOCK,
    ///0x14 - reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.
    pub bsec_denable: BSEC_DENABLE,
    _reserved6: [u8; 0x04],
    ///0x1c - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    pub bsec_otp_disturbed0: BSEC_OTP_DISTURBED0,
    ///0x20 - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    pub bsec_otp_disturbed1: BSEC_OTP_DISTURBED1,
    ///0x24 - BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    pub bsec_otp_disturbed2: BSEC_OTP_DISTURBED2,
    _reserved9: [u8; 0x0c],
    ///0x34 - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    pub bsec_otp_error0: BSEC_OTP_ERROR0,
    ///0x38 - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    pub bsec_otp_error1: BSEC_OTP_ERROR1,
    ///0x3c - BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    pub bsec_otp_error2: BSEC_OTP_ERROR2,
    _reserved12: [u8; 0x0c],
    ///0x4c - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    pub bsec_otp_wrlock0: BSEC_OTP_WRLOCK0,
    ///0x50 - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    pub bsec_otp_wrlock1: BSEC_OTP_WRLOCK1,
    ///0x54 - BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    pub bsec_otp_wrlock2: BSEC_OTP_WRLOCK2,
    _reserved15: [u8; 0x0c],
    ///0x64 - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    pub bsec_otp_splock0: BSEC_OTP_SPLOCK0,
    ///0x68 - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    pub bsec_otp_splock1: BSEC_OTP_SPLOCK1,
    ///0x6c - BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    pub bsec_otp_splock2: BSEC_OTP_SPLOCK2,
    _reserved18: [u8; 0x0c],
    ///0x7c - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    pub bsec_otp_swlock0: BSEC_OTP_SWLOCK0,
    ///0x80 - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    pub bsec_otp_swlock1: BSEC_OTP_SWLOCK1,
    ///0x84 - BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    pub bsec_otp_swlock2: BSEC_OTP_SWLOCK2,
    _reserved21: [u8; 0x0c],
    ///0x94 - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    pub bsec_otp_srlock0: BSEC_OTP_SRLOCK0,
    ///0x98 - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    pub bsec_otp_srlock1: BSEC_OTP_SRLOCK1,
    ///0x9c - BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    pub bsec_otp_srlock2: BSEC_OTP_SRLOCK2,
    _reserved24: [u8; 0x0c],
    ///0xac - BSEC JTAG input register
    pub bsec_jtagin: BSEC_JTAGIN,
    ///0xb0 - BSEC JTAG output register
    pub bsec_jtagout: BSEC_JTAGOUT,
    ///0xb4 - BSEC scratch register
    pub bsec_scratch: BSEC_SCRATCH,
    _reserved27: [u8; 0x0148],
    ///0x200 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data0: BSEC_OTP_DATA0,
    ///0x204 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data1: BSEC_OTP_DATA1,
    ///0x208 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data2: BSEC_OTP_DATA2,
    ///0x20c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data3: BSEC_OTP_DATA3,
    ///0x210 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data4: BSEC_OTP_DATA4,
    ///0x214 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data5: BSEC_OTP_DATA5,
    ///0x218 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data6: BSEC_OTP_DATA6,
    ///0x21c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data7: BSEC_OTP_DATA7,
    ///0x220 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data8: BSEC_OTP_DATA8,
    ///0x224 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data9: BSEC_OTP_DATA9,
    ///0x228 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data10: BSEC_OTP_DATA10,
    ///0x22c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data11: BSEC_OTP_DATA11,
    ///0x230 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data12: BSEC_OTP_DATA12,
    ///0x234 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data13: BSEC_OTP_DATA13,
    ///0x238 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data14: BSEC_OTP_DATA14,
    ///0x23c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data15: BSEC_OTP_DATA15,
    ///0x240 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data16: BSEC_OTP_DATA16,
    ///0x244 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data17: BSEC_OTP_DATA17,
    ///0x248 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data18: BSEC_OTP_DATA18,
    ///0x24c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data19: BSEC_OTP_DATA19,
    ///0x250 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data20: BSEC_OTP_DATA20,
    ///0x254 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data21: BSEC_OTP_DATA21,
    ///0x258 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data22: BSEC_OTP_DATA22,
    ///0x25c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data23: BSEC_OTP_DATA23,
    ///0x260 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data24: BSEC_OTP_DATA24,
    ///0x264 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data25: BSEC_OTP_DATA25,
    ///0x268 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data26: BSEC_OTP_DATA26,
    ///0x26c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data27: BSEC_OTP_DATA27,
    ///0x270 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data28: BSEC_OTP_DATA28,
    ///0x274 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data29: BSEC_OTP_DATA29,
    ///0x278 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data30: BSEC_OTP_DATA30,
    ///0x27c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data31: BSEC_OTP_DATA31,
    ///0x280 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data32: BSEC_OTP_DATA32,
    ///0x284 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data33: BSEC_OTP_DATA33,
    ///0x288 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data34: BSEC_OTP_DATA34,
    ///0x28c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data35: BSEC_OTP_DATA35,
    ///0x290 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data36: BSEC_OTP_DATA36,
    ///0x294 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data37: BSEC_OTP_DATA37,
    ///0x298 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data38: BSEC_OTP_DATA38,
    ///0x29c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data39: BSEC_OTP_DATA39,
    ///0x2a0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data40: BSEC_OTP_DATA40,
    ///0x2a4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data41: BSEC_OTP_DATA41,
    ///0x2a8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data42: BSEC_OTP_DATA42,
    ///0x2ac - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data43: BSEC_OTP_DATA43,
    ///0x2b0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data44: BSEC_OTP_DATA44,
    ///0x2b4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data45: BSEC_OTP_DATA45,
    ///0x2b8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data46: BSEC_OTP_DATA46,
    ///0x2bc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data47: BSEC_OTP_DATA47,
    ///0x2c0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data48: BSEC_OTP_DATA48,
    ///0x2c4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data49: BSEC_OTP_DATA49,
    ///0x2c8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data50: BSEC_OTP_DATA50,
    ///0x2cc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data51: BSEC_OTP_DATA51,
    ///0x2d0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data52: BSEC_OTP_DATA52,
    ///0x2d4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data53: BSEC_OTP_DATA53,
    ///0x2d8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data54: BSEC_OTP_DATA54,
    ///0x2dc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data55: BSEC_OTP_DATA55,
    ///0x2e0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data56: BSEC_OTP_DATA56,
    ///0x2e4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data57: BSEC_OTP_DATA57,
    ///0x2e8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data58: BSEC_OTP_DATA58,
    ///0x2ec - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data59: BSEC_OTP_DATA59,
    ///0x2f0 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data60: BSEC_OTP_DATA60,
    ///0x2f4 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data61: BSEC_OTP_DATA61,
    ///0x2f8 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data62: BSEC_OTP_DATA62,
    ///0x2fc - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data63: BSEC_OTP_DATA63,
    ///0x300 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data64: BSEC_OTP_DATA64,
    ///0x304 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data65: BSEC_OTP_DATA65,
    ///0x308 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data66: BSEC_OTP_DATA66,
    ///0x30c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data67: BSEC_OTP_DATA67,
    ///0x310 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data68: BSEC_OTP_DATA68,
    ///0x314 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data69: BSEC_OTP_DATA69,
    ///0x318 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data70: BSEC_OTP_DATA70,
    ///0x31c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data71: BSEC_OTP_DATA71,
    ///0x320 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data72: BSEC_OTP_DATA72,
    ///0x324 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data73: BSEC_OTP_DATA73,
    ///0x328 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data74: BSEC_OTP_DATA74,
    ///0x32c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data75: BSEC_OTP_DATA75,
    ///0x330 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data76: BSEC_OTP_DATA76,
    ///0x334 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data77: BSEC_OTP_DATA77,
    ///0x338 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data78: BSEC_OTP_DATA78,
    ///0x33c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data79: BSEC_OTP_DATA79,
    ///0x340 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data80: BSEC_OTP_DATA80,
    ///0x344 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data81: BSEC_OTP_DATA81,
    ///0x348 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data82: BSEC_OTP_DATA82,
    ///0x34c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data83: BSEC_OTP_DATA83,
    ///0x350 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data84: BSEC_OTP_DATA84,
    ///0x354 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data85: BSEC_OTP_DATA85,
    ///0x358 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data86: BSEC_OTP_DATA86,
    ///0x35c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data87: BSEC_OTP_DATA87,
    ///0x360 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data88: BSEC_OTP_DATA88,
    ///0x364 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data89: BSEC_OTP_DATA89,
    ///0x368 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data90: BSEC_OTP_DATA90,
    ///0x36c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data91: BSEC_OTP_DATA91,
    ///0x370 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data92: BSEC_OTP_DATA92,
    ///0x374 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data93: BSEC_OTP_DATA93,
    ///0x378 - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data94: BSEC_OTP_DATA94,
    ///0x37c - Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
    ///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub bsec_otp_data95: BSEC_OTP_DATA95,
    _reserved123: [u8; 0x0c70],
    ///0xff0 - BSEC hardware configuration register
    pub bsec_hwcfgr: BSEC_HWCFGR,
    ///0xff4 - BSEC version register
    pub bsec_verr: BSEC_VERR,
    ///0xff8 - BSEC identification register
    pub bsec_ipidr: BSEC_IPIDR,
    ///0xffc - BSEC size identification register
    pub bsec_sidr: BSEC_SIDR,
}
///BSEC_OTP_CONFIG (rw) register accessor: an alias for `Reg<BSEC_OTP_CONFIG_SPEC>`
pub type BSEC_OTP_CONFIG = crate::Reg<bsec_otp_config::BSEC_OTP_CONFIG_SPEC>;
///BSEC OTP configuration register
pub mod bsec_otp_config;
///BSEC_OTP_CONTROL (rw) register accessor: an alias for `Reg<BSEC_OTP_CONTROL_SPEC>`
pub type BSEC_OTP_CONTROL = crate::Reg<bsec_otp_control::BSEC_OTP_CONTROL_SPEC>;
///BSEC OTP control register
pub mod bsec_otp_control;
///BSEC_OTP_WRDATA (rw) register accessor: an alias for `Reg<BSEC_OTP_WRDATA_SPEC>`
pub type BSEC_OTP_WRDATA = crate::Reg<bsec_otp_wrdata::BSEC_OTP_WRDATA_SPEC>;
///BSEC OTP write data register
pub mod bsec_otp_wrdata;
///BSEC_OTP_STATUS (r) register accessor: an alias for `Reg<BSEC_OTP_STATUS_SPEC>`
pub type BSEC_OTP_STATUS = crate::Reg<bsec_otp_status::BSEC_OTP_STATUS_SPEC>;
///BSEC OTP status register
pub mod bsec_otp_status;
///BSEC_OTP_LOCK (rw) register accessor: an alias for `Reg<BSEC_OTP_LOCK_SPEC>`
pub type BSEC_OTP_LOCK = crate::Reg<bsec_otp_lock::BSEC_OTP_LOCK_SPEC>;
///BSEC OTP lock configuration register
pub mod bsec_otp_lock;
///BSEC_DENABLE (rw) register accessor: an alias for `Reg<BSEC_DENABLE_SPEC>`
pub type BSEC_DENABLE = crate::Reg<bsec_denable::BSEC_DENABLE_SPEC>;
///reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.
pub mod bsec_denable;
///BSEC_OTP_DISTURBED0 (r) register accessor: an alias for `Reg<BSEC_OTP_DISTURBED0_SPEC>`
pub type BSEC_OTP_DISTURBED0 = crate::Reg<bsec_otp_disturbed0::BSEC_OTP_DISTURBED0_SPEC>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod bsec_otp_disturbed0;
///BSEC_OTP_DISTURBED1 (r) register accessor: an alias for `Reg<BSEC_OTP_DISTURBED1_SPEC>`
pub type BSEC_OTP_DISTURBED1 = crate::Reg<bsec_otp_disturbed1::BSEC_OTP_DISTURBED1_SPEC>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod bsec_otp_disturbed1;
///BSEC_OTP_DISTURBED2 (r) register accessor: an alias for `Reg<BSEC_OTP_DISTURBED2_SPEC>`
pub type BSEC_OTP_DISTURBED2 = crate::Reg<bsec_otp_disturbed2::BSEC_OTP_DISTURBED2_SPEC>;
///BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod bsec_otp_disturbed2;
///BSEC_OTP_ERROR0 (r) register accessor: an alias for `Reg<BSEC_OTP_ERROR0_SPEC>`
pub type BSEC_OTP_ERROR0 = crate::Reg<bsec_otp_error0::BSEC_OTP_ERROR0_SPEC>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod bsec_otp_error0;
///BSEC_OTP_ERROR1 (r) register accessor: an alias for `Reg<BSEC_OTP_ERROR1_SPEC>`
pub type BSEC_OTP_ERROR1 = crate::Reg<bsec_otp_error1::BSEC_OTP_ERROR1_SPEC>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod bsec_otp_error1;
///BSEC_OTP_ERROR2 (r) register accessor: an alias for `Reg<BSEC_OTP_ERROR2_SPEC>`
pub type BSEC_OTP_ERROR2 = crate::Reg<bsec_otp_error2::BSEC_OTP_ERROR2_SPEC>;
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod bsec_otp_error2;
///BSEC_OTP_WRLOCK0 (r) register accessor: an alias for `Reg<BSEC_OTP_WRLOCK0_SPEC>`
pub type BSEC_OTP_WRLOCK0 = crate::Reg<bsec_otp_wrlock0::BSEC_OTP_WRLOCK0_SPEC>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod bsec_otp_wrlock0;
///BSEC_OTP_WRLOCK1 (r) register accessor: an alias for `Reg<BSEC_OTP_WRLOCK1_SPEC>`
pub type BSEC_OTP_WRLOCK1 = crate::Reg<bsec_otp_wrlock1::BSEC_OTP_WRLOCK1_SPEC>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod bsec_otp_wrlock1;
///BSEC_OTP_WRLOCK2 (r) register accessor: an alias for `Reg<BSEC_OTP_WRLOCK2_SPEC>`
pub type BSEC_OTP_WRLOCK2 = crate::Reg<bsec_otp_wrlock2::BSEC_OTP_WRLOCK2_SPEC>;
///BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod bsec_otp_wrlock2;
///BSEC_OTP_SPLOCK0 (rw) register accessor: an alias for `Reg<BSEC_OTP_SPLOCK0_SPEC>`
pub type BSEC_OTP_SPLOCK0 = crate::Reg<bsec_otp_splock0::BSEC_OTP_SPLOCK0_SPEC>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod bsec_otp_splock0;
///BSEC_OTP_SPLOCK1 (rw) register accessor: an alias for `Reg<BSEC_OTP_SPLOCK1_SPEC>`
pub type BSEC_OTP_SPLOCK1 = crate::Reg<bsec_otp_splock1::BSEC_OTP_SPLOCK1_SPEC>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod bsec_otp_splock1;
///BSEC_OTP_SPLOCK2 (rw) register accessor: an alias for `Reg<BSEC_OTP_SPLOCK2_SPEC>`
pub type BSEC_OTP_SPLOCK2 = crate::Reg<bsec_otp_splock2::BSEC_OTP_SPLOCK2_SPEC>;
///BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod bsec_otp_splock2;
///BSEC_OTP_SWLOCK0 (rw) register accessor: an alias for `Reg<BSEC_OTP_SWLOCK0_SPEC>`
pub type BSEC_OTP_SWLOCK0 = crate::Reg<bsec_otp_swlock0::BSEC_OTP_SWLOCK0_SPEC>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod bsec_otp_swlock0;
///BSEC_OTP_SWLOCK1 (rw) register accessor: an alias for `Reg<BSEC_OTP_SWLOCK1_SPEC>`
pub type BSEC_OTP_SWLOCK1 = crate::Reg<bsec_otp_swlock1::BSEC_OTP_SWLOCK1_SPEC>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod bsec_otp_swlock1;
///BSEC_OTP_SWLOCK2 (rw) register accessor: an alias for `Reg<BSEC_OTP_SWLOCK2_SPEC>`
pub type BSEC_OTP_SWLOCK2 = crate::Reg<bsec_otp_swlock2::BSEC_OTP_SWLOCK2_SPEC>;
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod bsec_otp_swlock2;
///BSEC_OTP_SRLOCK0 (rw) register accessor: an alias for `Reg<BSEC_OTP_SRLOCK0_SPEC>`
pub type BSEC_OTP_SRLOCK0 = crate::Reg<bsec_otp_srlock0::BSEC_OTP_SRLOCK0_SPEC>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod bsec_otp_srlock0;
///BSEC_OTP_SRLOCK1 (rw) register accessor: an alias for `Reg<BSEC_OTP_SRLOCK1_SPEC>`
pub type BSEC_OTP_SRLOCK1 = crate::Reg<bsec_otp_srlock1::BSEC_OTP_SRLOCK1_SPEC>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod bsec_otp_srlock1;
///BSEC_OTP_SRLOCK2 (rw) register accessor: an alias for `Reg<BSEC_OTP_SRLOCK2_SPEC>`
pub type BSEC_OTP_SRLOCK2 = crate::Reg<bsec_otp_srlock2::BSEC_OTP_SRLOCK2_SPEC>;
///BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod bsec_otp_srlock2;
///BSEC_JTAGIN (r) register accessor: an alias for `Reg<BSEC_JTAGIN_SPEC>`
pub type BSEC_JTAGIN = crate::Reg<bsec_jtagin::BSEC_JTAGIN_SPEC>;
///BSEC JTAG input register
pub mod bsec_jtagin;
///BSEC_JTAGOUT (rw) register accessor: an alias for `Reg<BSEC_JTAGOUT_SPEC>`
pub type BSEC_JTAGOUT = crate::Reg<bsec_jtagout::BSEC_JTAGOUT_SPEC>;
///BSEC JTAG output register
pub mod bsec_jtagout;
///BSEC_SCRATCH (rw) register accessor: an alias for `Reg<BSEC_SCRATCH_SPEC>`
pub type BSEC_SCRATCH = crate::Reg<bsec_scratch::BSEC_SCRATCH_SPEC>;
///BSEC scratch register
pub mod bsec_scratch;
///BSEC_OTP_DATA0 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA0_SPEC>`
pub type BSEC_OTP_DATA0 = crate::Reg<bsec_otp_data0::BSEC_OTP_DATA0_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data0;
///BSEC_OTP_DATA1 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA1_SPEC>`
pub type BSEC_OTP_DATA1 = crate::Reg<bsec_otp_data1::BSEC_OTP_DATA1_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data1;
///BSEC_OTP_DATA2 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA2_SPEC>`
pub type BSEC_OTP_DATA2 = crate::Reg<bsec_otp_data2::BSEC_OTP_DATA2_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data2;
///BSEC_OTP_DATA3 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA3_SPEC>`
pub type BSEC_OTP_DATA3 = crate::Reg<bsec_otp_data3::BSEC_OTP_DATA3_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data3;
///BSEC_OTP_DATA4 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA4_SPEC>`
pub type BSEC_OTP_DATA4 = crate::Reg<bsec_otp_data4::BSEC_OTP_DATA4_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data4;
///BSEC_OTP_DATA5 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA5_SPEC>`
pub type BSEC_OTP_DATA5 = crate::Reg<bsec_otp_data5::BSEC_OTP_DATA5_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data5;
///BSEC_OTP_DATA6 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA6_SPEC>`
pub type BSEC_OTP_DATA6 = crate::Reg<bsec_otp_data6::BSEC_OTP_DATA6_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data6;
///BSEC_OTP_DATA7 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA7_SPEC>`
pub type BSEC_OTP_DATA7 = crate::Reg<bsec_otp_data7::BSEC_OTP_DATA7_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data7;
///BSEC_OTP_DATA8 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA8_SPEC>`
pub type BSEC_OTP_DATA8 = crate::Reg<bsec_otp_data8::BSEC_OTP_DATA8_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data8;
///BSEC_OTP_DATA9 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA9_SPEC>`
pub type BSEC_OTP_DATA9 = crate::Reg<bsec_otp_data9::BSEC_OTP_DATA9_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data9;
///BSEC_OTP_DATA10 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA10_SPEC>`
pub type BSEC_OTP_DATA10 = crate::Reg<bsec_otp_data10::BSEC_OTP_DATA10_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data10;
///BSEC_OTP_DATA11 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA11_SPEC>`
pub type BSEC_OTP_DATA11 = crate::Reg<bsec_otp_data11::BSEC_OTP_DATA11_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data11;
///BSEC_OTP_DATA12 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA12_SPEC>`
pub type BSEC_OTP_DATA12 = crate::Reg<bsec_otp_data12::BSEC_OTP_DATA12_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data12;
///BSEC_OTP_DATA13 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA13_SPEC>`
pub type BSEC_OTP_DATA13 = crate::Reg<bsec_otp_data13::BSEC_OTP_DATA13_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data13;
///BSEC_OTP_DATA14 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA14_SPEC>`
pub type BSEC_OTP_DATA14 = crate::Reg<bsec_otp_data14::BSEC_OTP_DATA14_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data14;
///BSEC_OTP_DATA15 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA15_SPEC>`
pub type BSEC_OTP_DATA15 = crate::Reg<bsec_otp_data15::BSEC_OTP_DATA15_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data15;
///BSEC_OTP_DATA16 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA16_SPEC>`
pub type BSEC_OTP_DATA16 = crate::Reg<bsec_otp_data16::BSEC_OTP_DATA16_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data16;
///BSEC_OTP_DATA17 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA17_SPEC>`
pub type BSEC_OTP_DATA17 = crate::Reg<bsec_otp_data17::BSEC_OTP_DATA17_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data17;
///BSEC_OTP_DATA18 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA18_SPEC>`
pub type BSEC_OTP_DATA18 = crate::Reg<bsec_otp_data18::BSEC_OTP_DATA18_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data18;
///BSEC_OTP_DATA19 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA19_SPEC>`
pub type BSEC_OTP_DATA19 = crate::Reg<bsec_otp_data19::BSEC_OTP_DATA19_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data19;
///BSEC_OTP_DATA20 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA20_SPEC>`
pub type BSEC_OTP_DATA20 = crate::Reg<bsec_otp_data20::BSEC_OTP_DATA20_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data20;
///BSEC_OTP_DATA21 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA21_SPEC>`
pub type BSEC_OTP_DATA21 = crate::Reg<bsec_otp_data21::BSEC_OTP_DATA21_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data21;
///BSEC_OTP_DATA22 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA22_SPEC>`
pub type BSEC_OTP_DATA22 = crate::Reg<bsec_otp_data22::BSEC_OTP_DATA22_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data22;
///BSEC_OTP_DATA23 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA23_SPEC>`
pub type BSEC_OTP_DATA23 = crate::Reg<bsec_otp_data23::BSEC_OTP_DATA23_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data23;
///BSEC_OTP_DATA24 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA24_SPEC>`
pub type BSEC_OTP_DATA24 = crate::Reg<bsec_otp_data24::BSEC_OTP_DATA24_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data24;
///BSEC_OTP_DATA25 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA25_SPEC>`
pub type BSEC_OTP_DATA25 = crate::Reg<bsec_otp_data25::BSEC_OTP_DATA25_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data25;
///BSEC_OTP_DATA26 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA26_SPEC>`
pub type BSEC_OTP_DATA26 = crate::Reg<bsec_otp_data26::BSEC_OTP_DATA26_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data26;
///BSEC_OTP_DATA27 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA27_SPEC>`
pub type BSEC_OTP_DATA27 = crate::Reg<bsec_otp_data27::BSEC_OTP_DATA27_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data27;
///BSEC_OTP_DATA28 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA28_SPEC>`
pub type BSEC_OTP_DATA28 = crate::Reg<bsec_otp_data28::BSEC_OTP_DATA28_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data28;
///BSEC_OTP_DATA29 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA29_SPEC>`
pub type BSEC_OTP_DATA29 = crate::Reg<bsec_otp_data29::BSEC_OTP_DATA29_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data29;
///BSEC_OTP_DATA30 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA30_SPEC>`
pub type BSEC_OTP_DATA30 = crate::Reg<bsec_otp_data30::BSEC_OTP_DATA30_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data30;
///BSEC_OTP_DATA31 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA31_SPEC>`
pub type BSEC_OTP_DATA31 = crate::Reg<bsec_otp_data31::BSEC_OTP_DATA31_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data31;
///BSEC_OTP_DATA32 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA32_SPEC>`
pub type BSEC_OTP_DATA32 = crate::Reg<bsec_otp_data32::BSEC_OTP_DATA32_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data32;
///BSEC_OTP_DATA33 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA33_SPEC>`
pub type BSEC_OTP_DATA33 = crate::Reg<bsec_otp_data33::BSEC_OTP_DATA33_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data33;
///BSEC_OTP_DATA34 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA34_SPEC>`
pub type BSEC_OTP_DATA34 = crate::Reg<bsec_otp_data34::BSEC_OTP_DATA34_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data34;
///BSEC_OTP_DATA35 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA35_SPEC>`
pub type BSEC_OTP_DATA35 = crate::Reg<bsec_otp_data35::BSEC_OTP_DATA35_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data35;
///BSEC_OTP_DATA36 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA36_SPEC>`
pub type BSEC_OTP_DATA36 = crate::Reg<bsec_otp_data36::BSEC_OTP_DATA36_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data36;
///BSEC_OTP_DATA37 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA37_SPEC>`
pub type BSEC_OTP_DATA37 = crate::Reg<bsec_otp_data37::BSEC_OTP_DATA37_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data37;
///BSEC_OTP_DATA38 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA38_SPEC>`
pub type BSEC_OTP_DATA38 = crate::Reg<bsec_otp_data38::BSEC_OTP_DATA38_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data38;
///BSEC_OTP_DATA39 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA39_SPEC>`
pub type BSEC_OTP_DATA39 = crate::Reg<bsec_otp_data39::BSEC_OTP_DATA39_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data39;
///BSEC_OTP_DATA40 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA40_SPEC>`
pub type BSEC_OTP_DATA40 = crate::Reg<bsec_otp_data40::BSEC_OTP_DATA40_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data40;
///BSEC_OTP_DATA41 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA41_SPEC>`
pub type BSEC_OTP_DATA41 = crate::Reg<bsec_otp_data41::BSEC_OTP_DATA41_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data41;
///BSEC_OTP_DATA42 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA42_SPEC>`
pub type BSEC_OTP_DATA42 = crate::Reg<bsec_otp_data42::BSEC_OTP_DATA42_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data42;
///BSEC_OTP_DATA43 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA43_SPEC>`
pub type BSEC_OTP_DATA43 = crate::Reg<bsec_otp_data43::BSEC_OTP_DATA43_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data43;
///BSEC_OTP_DATA44 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA44_SPEC>`
pub type BSEC_OTP_DATA44 = crate::Reg<bsec_otp_data44::BSEC_OTP_DATA44_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data44;
///BSEC_OTP_DATA45 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA45_SPEC>`
pub type BSEC_OTP_DATA45 = crate::Reg<bsec_otp_data45::BSEC_OTP_DATA45_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data45;
///BSEC_OTP_DATA46 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA46_SPEC>`
pub type BSEC_OTP_DATA46 = crate::Reg<bsec_otp_data46::BSEC_OTP_DATA46_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data46;
///BSEC_OTP_DATA47 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA47_SPEC>`
pub type BSEC_OTP_DATA47 = crate::Reg<bsec_otp_data47::BSEC_OTP_DATA47_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data47;
///BSEC_OTP_DATA48 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA48_SPEC>`
pub type BSEC_OTP_DATA48 = crate::Reg<bsec_otp_data48::BSEC_OTP_DATA48_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data48;
///BSEC_OTP_DATA49 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA49_SPEC>`
pub type BSEC_OTP_DATA49 = crate::Reg<bsec_otp_data49::BSEC_OTP_DATA49_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data49;
///BSEC_OTP_DATA50 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA50_SPEC>`
pub type BSEC_OTP_DATA50 = crate::Reg<bsec_otp_data50::BSEC_OTP_DATA50_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data50;
///BSEC_OTP_DATA51 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA51_SPEC>`
pub type BSEC_OTP_DATA51 = crate::Reg<bsec_otp_data51::BSEC_OTP_DATA51_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data51;
///BSEC_OTP_DATA52 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA52_SPEC>`
pub type BSEC_OTP_DATA52 = crate::Reg<bsec_otp_data52::BSEC_OTP_DATA52_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data52;
///BSEC_OTP_DATA53 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA53_SPEC>`
pub type BSEC_OTP_DATA53 = crate::Reg<bsec_otp_data53::BSEC_OTP_DATA53_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data53;
///BSEC_OTP_DATA54 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA54_SPEC>`
pub type BSEC_OTP_DATA54 = crate::Reg<bsec_otp_data54::BSEC_OTP_DATA54_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data54;
///BSEC_OTP_DATA55 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA55_SPEC>`
pub type BSEC_OTP_DATA55 = crate::Reg<bsec_otp_data55::BSEC_OTP_DATA55_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data55;
///BSEC_OTP_DATA56 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA56_SPEC>`
pub type BSEC_OTP_DATA56 = crate::Reg<bsec_otp_data56::BSEC_OTP_DATA56_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data56;
///BSEC_OTP_DATA57 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA57_SPEC>`
pub type BSEC_OTP_DATA57 = crate::Reg<bsec_otp_data57::BSEC_OTP_DATA57_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data57;
///BSEC_OTP_DATA58 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA58_SPEC>`
pub type BSEC_OTP_DATA58 = crate::Reg<bsec_otp_data58::BSEC_OTP_DATA58_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data58;
///BSEC_OTP_DATA59 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA59_SPEC>`
pub type BSEC_OTP_DATA59 = crate::Reg<bsec_otp_data59::BSEC_OTP_DATA59_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data59;
///BSEC_OTP_DATA60 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA60_SPEC>`
pub type BSEC_OTP_DATA60 = crate::Reg<bsec_otp_data60::BSEC_OTP_DATA60_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data60;
///BSEC_OTP_DATA61 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA61_SPEC>`
pub type BSEC_OTP_DATA61 = crate::Reg<bsec_otp_data61::BSEC_OTP_DATA61_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data61;
///BSEC_OTP_DATA62 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA62_SPEC>`
pub type BSEC_OTP_DATA62 = crate::Reg<bsec_otp_data62::BSEC_OTP_DATA62_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data62;
///BSEC_OTP_DATA63 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA63_SPEC>`
pub type BSEC_OTP_DATA63 = crate::Reg<bsec_otp_data63::BSEC_OTP_DATA63_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data63;
///BSEC_OTP_DATA64 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA64_SPEC>`
pub type BSEC_OTP_DATA64 = crate::Reg<bsec_otp_data64::BSEC_OTP_DATA64_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data64;
///BSEC_OTP_DATA65 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA65_SPEC>`
pub type BSEC_OTP_DATA65 = crate::Reg<bsec_otp_data65::BSEC_OTP_DATA65_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data65;
///BSEC_OTP_DATA66 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA66_SPEC>`
pub type BSEC_OTP_DATA66 = crate::Reg<bsec_otp_data66::BSEC_OTP_DATA66_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data66;
///BSEC_OTP_DATA67 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA67_SPEC>`
pub type BSEC_OTP_DATA67 = crate::Reg<bsec_otp_data67::BSEC_OTP_DATA67_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data67;
///BSEC_OTP_DATA68 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA68_SPEC>`
pub type BSEC_OTP_DATA68 = crate::Reg<bsec_otp_data68::BSEC_OTP_DATA68_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data68;
///BSEC_OTP_DATA69 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA69_SPEC>`
pub type BSEC_OTP_DATA69 = crate::Reg<bsec_otp_data69::BSEC_OTP_DATA69_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data69;
///BSEC_OTP_DATA70 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA70_SPEC>`
pub type BSEC_OTP_DATA70 = crate::Reg<bsec_otp_data70::BSEC_OTP_DATA70_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data70;
///BSEC_OTP_DATA71 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA71_SPEC>`
pub type BSEC_OTP_DATA71 = crate::Reg<bsec_otp_data71::BSEC_OTP_DATA71_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data71;
///BSEC_OTP_DATA72 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA72_SPEC>`
pub type BSEC_OTP_DATA72 = crate::Reg<bsec_otp_data72::BSEC_OTP_DATA72_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data72;
///BSEC_OTP_DATA73 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA73_SPEC>`
pub type BSEC_OTP_DATA73 = crate::Reg<bsec_otp_data73::BSEC_OTP_DATA73_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data73;
///BSEC_OTP_DATA74 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA74_SPEC>`
pub type BSEC_OTP_DATA74 = crate::Reg<bsec_otp_data74::BSEC_OTP_DATA74_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data74;
///BSEC_OTP_DATA75 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA75_SPEC>`
pub type BSEC_OTP_DATA75 = crate::Reg<bsec_otp_data75::BSEC_OTP_DATA75_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data75;
///BSEC_OTP_DATA76 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA76_SPEC>`
pub type BSEC_OTP_DATA76 = crate::Reg<bsec_otp_data76::BSEC_OTP_DATA76_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data76;
///BSEC_OTP_DATA77 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA77_SPEC>`
pub type BSEC_OTP_DATA77 = crate::Reg<bsec_otp_data77::BSEC_OTP_DATA77_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data77;
///BSEC_OTP_DATA78 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA78_SPEC>`
pub type BSEC_OTP_DATA78 = crate::Reg<bsec_otp_data78::BSEC_OTP_DATA78_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data78;
///BSEC_OTP_DATA79 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA79_SPEC>`
pub type BSEC_OTP_DATA79 = crate::Reg<bsec_otp_data79::BSEC_OTP_DATA79_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data79;
///BSEC_OTP_DATA80 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA80_SPEC>`
pub type BSEC_OTP_DATA80 = crate::Reg<bsec_otp_data80::BSEC_OTP_DATA80_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data80;
///BSEC_OTP_DATA81 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA81_SPEC>`
pub type BSEC_OTP_DATA81 = crate::Reg<bsec_otp_data81::BSEC_OTP_DATA81_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data81;
///BSEC_OTP_DATA82 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA82_SPEC>`
pub type BSEC_OTP_DATA82 = crate::Reg<bsec_otp_data82::BSEC_OTP_DATA82_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data82;
///BSEC_OTP_DATA83 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA83_SPEC>`
pub type BSEC_OTP_DATA83 = crate::Reg<bsec_otp_data83::BSEC_OTP_DATA83_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data83;
///BSEC_OTP_DATA84 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA84_SPEC>`
pub type BSEC_OTP_DATA84 = crate::Reg<bsec_otp_data84::BSEC_OTP_DATA84_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data84;
///BSEC_OTP_DATA85 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA85_SPEC>`
pub type BSEC_OTP_DATA85 = crate::Reg<bsec_otp_data85::BSEC_OTP_DATA85_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data85;
///BSEC_OTP_DATA86 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA86_SPEC>`
pub type BSEC_OTP_DATA86 = crate::Reg<bsec_otp_data86::BSEC_OTP_DATA86_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data86;
///BSEC_OTP_DATA87 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA87_SPEC>`
pub type BSEC_OTP_DATA87 = crate::Reg<bsec_otp_data87::BSEC_OTP_DATA87_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data87;
///BSEC_OTP_DATA88 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA88_SPEC>`
pub type BSEC_OTP_DATA88 = crate::Reg<bsec_otp_data88::BSEC_OTP_DATA88_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data88;
///BSEC_OTP_DATA89 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA89_SPEC>`
pub type BSEC_OTP_DATA89 = crate::Reg<bsec_otp_data89::BSEC_OTP_DATA89_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data89;
///BSEC_OTP_DATA90 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA90_SPEC>`
pub type BSEC_OTP_DATA90 = crate::Reg<bsec_otp_data90::BSEC_OTP_DATA90_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data90;
///BSEC_OTP_DATA91 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA91_SPEC>`
pub type BSEC_OTP_DATA91 = crate::Reg<bsec_otp_data91::BSEC_OTP_DATA91_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data91;
///BSEC_OTP_DATA92 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA92_SPEC>`
pub type BSEC_OTP_DATA92 = crate::Reg<bsec_otp_data92::BSEC_OTP_DATA92_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data92;
///BSEC_OTP_DATA93 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA93_SPEC>`
pub type BSEC_OTP_DATA93 = crate::Reg<bsec_otp_data93::BSEC_OTP_DATA93_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data93;
///BSEC_OTP_DATA94 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA94_SPEC>`
pub type BSEC_OTP_DATA94 = crate::Reg<bsec_otp_data94::BSEC_OTP_DATA94_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data94;
///BSEC_OTP_DATA95 (rw) register accessor: an alias for `Reg<BSEC_OTP_DATA95_SPEC>`
pub type BSEC_OTP_DATA95 = crate::Reg<bsec_otp_data95::BSEC_OTP_DATA95_SPEC>;
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod bsec_otp_data95;
///BSEC_HWCFGR (r) register accessor: an alias for `Reg<BSEC_HWCFGR_SPEC>`
pub type BSEC_HWCFGR = crate::Reg<bsec_hwcfgr::BSEC_HWCFGR_SPEC>;
///BSEC hardware configuration register
pub mod bsec_hwcfgr;
///BSEC_VERR (r) register accessor: an alias for `Reg<BSEC_VERR_SPEC>`
pub type BSEC_VERR = crate::Reg<bsec_verr::BSEC_VERR_SPEC>;
///BSEC version register
pub mod bsec_verr;
///BSEC_IPIDR (r) register accessor: an alias for `Reg<BSEC_IPIDR_SPEC>`
pub type BSEC_IPIDR = crate::Reg<bsec_ipidr::BSEC_IPIDR_SPEC>;
///BSEC identification register
pub mod bsec_ipidr;
///BSEC_SIDR (r) register accessor: an alias for `Reg<BSEC_SIDR_SPEC>`
pub type BSEC_SIDR = crate::Reg<bsec_sidr::BSEC_SIDR_SPEC>;
///BSEC size identification register
pub mod bsec_sidr;
