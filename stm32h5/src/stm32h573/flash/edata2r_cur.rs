///Register `EDATA2R_CUR` reader
pub struct R(crate::R<EDATA2R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDATA2R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDATA2R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDATA2R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EDATA2_STRT` reader - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data
pub type EDATA2_STRT_R = crate::FieldReader<u8, u8>;
///Field `EDATA2_EN` reader - Bank2 flash high-cycle data enable
pub type EDATA2_EN_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - EDATA2_STRT contains the start sectors of the flash high-cycle data area in Bank 2 There is no hardware effect to those bits. They shall be managed by ST tools in Flasher. ... Note: 111: The eight last sectors of the Bank 2 are reserved for flash high-cycle data
    #[inline(always)]
    pub fn edata2_strt(&self) -> EDATA2_STRT_R {
        EDATA2_STRT_R::new((self.bits & 7) as u8)
    }
    ///Bit 15 - Bank2 flash high-cycle data enable
    #[inline(always)]
    pub fn edata2_en(&self) -> EDATA2_EN_R {
        EDATA2_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///FLASH data sectors configuration Bank 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [edata2r_cur](index.html) module
pub struct EDATA2R_CUR_SPEC;
impl crate::RegisterSpec for EDATA2R_CUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [edata2r_cur::R](R) reader structure
impl crate::Readable for EDATA2R_CUR_SPEC {
    type Reader = R;
}
///`reset()` method sets EDATA2R_CUR to value 0
impl crate::Resettable for EDATA2R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
