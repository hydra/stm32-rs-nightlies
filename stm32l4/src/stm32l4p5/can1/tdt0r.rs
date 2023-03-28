///Register `TDT0R` reader
pub struct R(crate::R<TDT0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDT0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDT0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDT0R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TDT0R` writer
pub struct W(crate::W<TDT0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDT0R_SPEC>;
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
impl From<crate::W<TDT0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDT0R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DLC` reader - DLC
pub type DLC_R = crate::FieldReader<u8, u8>;
///Field `DLC` writer - DLC
pub type DLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDT0R_SPEC, u8, u8, 4, O>;
///Field `TGT` reader - TGT
pub type TGT_R = crate::BitReader<bool>;
///Field `TGT` writer - TGT
pub type TGT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TDT0R_SPEC, bool, O>;
///Field `TIME` reader - TIME
pub type TIME_R = crate::FieldReader<u16, u16>;
///Field `TIME` writer - TIME
pub type TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDT0R_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:3 - DLC
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - TGT
    #[inline(always)]
    pub fn tgt(&self) -> TGT_R {
        TGT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:31 - TIME
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:3 - DLC
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<0> {
        DLC_W::new(self)
    }
    ///Bit 8 - TGT
    #[inline(always)]
    #[must_use]
    pub fn tgt(&mut self) -> TGT_W<8> {
        TGT_W::new(self)
    }
    ///Bits 16:31 - TIME
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<16> {
        TIME_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///mailbox data length control and time stamp register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tdt0r](index.html) module
pub struct TDT0R_SPEC;
impl crate::RegisterSpec for TDT0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [tdt0r::R](R) reader structure
impl crate::Readable for TDT0R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tdt0r::W](W) writer structure
impl crate::Writable for TDT0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TDT0R to value 0
impl crate::Resettable for TDT0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
