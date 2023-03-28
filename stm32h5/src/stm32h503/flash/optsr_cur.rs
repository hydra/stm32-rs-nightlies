///Register `OPTSR_CUR` reader
pub struct R(crate::R<OPTSR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTSR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTSR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTSR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BOR_LEV` reader - Brownout level option status bit These bits reflects the power level that generates a system reset.
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
///Field `BORH_EN` reader - Brownout high enable status bit
pub type BORH_EN_R = crate::BitReader<bool>;
///Field `IWDG_SW` reader - IWDG control mode option status bit
pub type IWDG_SW_R = crate::BitReader<bool>;
///Field `WWDG_SW` reader - WWDG control mode option status bit
pub type WWDG_SW_R = crate::BitReader<bool>;
///Field `NRST_SHDW` reader - Core domain Shutdown entry reset option status bit
pub type NRST_SHDW_R = crate::BitReader<bool>;
///Field `NRST_STOP` reader - Core domain Stop entry reset option status bit
pub type NRST_STOP_R = crate::BitReader<bool>;
///Field `NRST_STDBY` reader - Core domain Standby entry reset option status bit
pub type NRST_STDBY_R = crate::BitReader<bool>;
///Field `PRODUCT_STATE` reader - Life state code (based on Hamming 8,4). More information in .
pub type PRODUCT_STATE_R = crate::FieldReader<u8, u8>;
///Field `IO_VDD_HSLV` reader - High-speed IO at low VDD voltage status bit. This bit can be set only with VDD below 2.5 V.
pub type IO_VDD_HSLV_R = crate::BitReader<bool>;
///Field `IO_VDDIO2_HSLV` reader - High-speed IO at low VDDIO2 voltage status bit. This bit can be set only with VDDIO2 below 2.5 V.
pub type IO_VDDIO2_HSLV_R = crate::BitReader<bool>;
///Field `IWDG_STOP` reader - IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode.
pub type IWDG_STOP_R = crate::BitReader<bool>;
///Field `IWDG_STDBY` reader - IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode.
pub type IWDG_STDBY_R = crate::BitReader<bool>;
///Field `SWAP_BANK` reader - Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset.
pub type SWAP_BANK_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:1 - Brownout level option status bit These bits reflects the power level that generates a system reset.
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Brownout high enable status bit
    #[inline(always)]
    pub fn borh_en(&self) -> BORH_EN_R {
        BORH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IWDG control mode option status bit
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WWDG control mode option status bit
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Core domain Shutdown entry reset option status bit
    #[inline(always)]
    pub fn nrst_shdw(&self) -> NRST_SHDW_R {
        NRST_SHDW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Core domain Stop entry reset option status bit
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Core domain Standby entry reset option status bit
    #[inline(always)]
    pub fn nrst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Life state code (based on Hamming 8,4). More information in .
    #[inline(always)]
    pub fn product_state(&self) -> PRODUCT_STATE_R {
        PRODUCT_STATE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - High-speed IO at low VDD voltage status bit. This bit can be set only with VDD below 2.5 V.
    #[inline(always)]
    pub fn io_vdd_hslv(&self) -> IO_VDD_HSLV_R {
        IO_VDD_HSLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - High-speed IO at low VDDIO2 voltage status bit. This bit can be set only with VDDIO2 below 2.5 V.
    #[inline(always)]
    pub fn io_vddio2_hslv(&self) -> IO_VDDIO2_HSLV_R {
        IO_VDDIO2_HSLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode.
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode.
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 31 - Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset.
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///FLASH option status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optsr_cur](index.html) module
pub struct OPTSR_CUR_SPEC;
impl crate::RegisterSpec for OPTSR_CUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [optsr_cur::R](R) reader structure
impl crate::Readable for OPTSR_CUR_SPEC {
    type Reader = R;
}
///`reset()` method sets OPTSR_CUR to value 0
impl crate::Resettable for OPTSR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
