///Register `MACTXTSSNR` reader
pub struct R(crate::R<MACTXTSSNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTXTSSNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTXTSSNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTXTSSNR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACTXTSSNR` writer
pub struct W(crate::W<MACTXTSSNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACTXTSSNR_SPEC>;
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
impl From<crate::W<MACTXTSSNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACTXTSSNR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXTSSLO` reader - Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp.
///
///The field is **cleared** (set to zero) following a read operation.
pub type TXTSSLO_R = crate::FieldReader<u32, u32>;
///Field `TXTSSLO` writer - Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp.
pub type TXTSSLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACTXTSSNR_SPEC, u32, u32, 31, O>;
///Field `TXTSSMIS` reader - Transmit Timestamp Status Missed When this bit is set, it indicates one of the following: The timestamp of the current packet is ignored if TXTSSTSM bit of the Timestamp control Register (ETH_MACTSCR) is reset The timestamp of the previous packet is overwritten with timestamp of the current packet if TXTSSTSM bit of the Timestamp control Register (ETH_MACTSCR) is set.
pub type TXTSSMIS_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:30 - Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp.
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - Transmit Timestamp Status Missed When this bit is set, it indicates one of the following: The timestamp of the current packet is ignored if TXTSSTSM bit of the Timestamp control Register (ETH_MACTSCR) is reset The timestamp of the previous packet is overwritten with timestamp of the current packet if TXTSSTSM bit of the Timestamp control Register (ETH_MACTSCR) is set.
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:30 - Transmit Timestamp Status Low This field contains the 31 bits of the Nanoseconds field of the Transmit packet's captured timestamp.
    #[inline(always)]
    #[must_use]
    pub fn txtsslo(&mut self) -> TXTSSLO_W<0> {
        TXTSSLO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Tx timestamp status nanoseconds register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mactxtssnr](index.html) module
pub struct MACTXTSSNR_SPEC;
impl crate::RegisterSpec for MACTXTSSNR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mactxtssnr::R](R) reader structure
impl crate::Readable for MACTXTSSNR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mactxtssnr::W](W) writer structure
impl crate::Writable for MACTXTSSNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACTXTSSNR to value 0
impl crate::Resettable for MACTXTSSNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
