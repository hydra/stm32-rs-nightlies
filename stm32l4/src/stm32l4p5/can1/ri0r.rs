///Register `RI0R` reader
pub struct R(crate::R<RI0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RI0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RI0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RI0R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RTR` reader - RTR
pub type RTR_R = crate::BitReader<bool>;
///Field `IDE` reader - IDE
pub type IDE_R = crate::BitReader<bool>;
///Field `EXID` reader - EXID
pub type EXID_R = crate::FieldReader<u32, u32>;
///Field `STID` reader - STID
pub type STID_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
///receive FIFO mailbox identifier register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ri0r](index.html) module
pub struct RI0R_SPEC;
impl crate::RegisterSpec for RI0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [ri0r::R](R) reader structure
impl crate::Readable for RI0R_SPEC {
    type Reader = R;
}
///`reset()` method sets RI0R to value 0
impl crate::Resettable for RI0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
