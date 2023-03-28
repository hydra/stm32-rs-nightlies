///Register `FDCAN_TTTMK` reader
pub struct R(crate::R<FDCAN_TTTMK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTTMK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTTMK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTTMK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TTTMK` writer
pub struct W(crate::W<FDCAN_TTTMK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTTMK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FDCAN_TTTMK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTTMK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TM` reader - TM
pub type TM_R = crate::FieldReader<u16, u16>;
///Field `TM` writer - TM
pub type TM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTTMK_SPEC, u16, u16, 16, O>;
///Field `TICC` reader - TICC
pub type TICC_R = crate::FieldReader<u8, u8>;
///Field `TICC` writer - TICC
pub type TICC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTTMK_SPEC, u8, u8, 7, O>;
///Field `LCKM` reader - LCKM
pub type LCKM_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:15 - TM
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:22 - TICC
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - LCKM
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - TM
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<0> {
        TM_W::new(self)
    }
    ///Bits 16:22 - TICC
    #[inline(always)]
    #[must_use]
    pub fn ticc(&mut self) -> TICC_W<16> {
        TICC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///A time mark interrupt (FDCAN_TTIR.TMI = 1) is generated when the time base indicated by FDCAN_TTOCN.TMC (cycle time, local time, or global time) has the same value as TM.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_tttmk](index.html) module
pub struct FDCAN_TTTMK_SPEC;
impl crate::RegisterSpec for FDCAN_TTTMK_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_tttmk::R](R) reader structure
impl crate::Readable for FDCAN_TTTMK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_tttmk::W](W) writer structure
impl crate::Writable for FDCAN_TTTMK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TTTMK to value 0
impl crate::Resettable for FDCAN_TTTMK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
