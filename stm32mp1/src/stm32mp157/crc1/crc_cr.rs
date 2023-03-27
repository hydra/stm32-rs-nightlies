///Register `CRC_CR` reader
pub struct R(crate::R<CRC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRC_CR` writer
pub struct W(crate::W<CRC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CR_SPEC>;
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
impl From<crate::W<CRC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RESET` reader - RESET
pub type RESET_R = crate::BitReader<bool>;
///Field `RESET` writer - RESET
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_CR_SPEC, bool, O>;
///Field `POLYSIZE` reader - POLYSIZE
pub type POLYSIZE_R = crate::FieldReader<u8, u8>;
///Field `POLYSIZE` writer - POLYSIZE
pub type POLYSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_CR_SPEC, u8, u8, 2, O>;
///Field `REV_IN` reader - REV_IN
pub type REV_IN_R = crate::FieldReader<u8, u8>;
///Field `REV_IN` writer - REV_IN
pub type REV_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_CR_SPEC, u8, u8, 2, O>;
///Field `REV_OUT` reader - REV_OUT
pub type REV_OUT_R = crate::BitReader<bool>;
///Field `REV_OUT` writer - REV_OUT
pub type REV_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRC_CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - RESET
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    ///Bits 3:4 - POLYSIZE
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - REV_IN
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - REV_OUT
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RESET
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    ///Bits 3:4 - POLYSIZE
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> POLYSIZE_W<3> {
        POLYSIZE_W::new(self)
    }
    ///Bits 5:6 - REV_IN
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> REV_IN_W<5> {
        REV_IN_W::new(self)
    }
    ///Bit 7 - REV_OUT
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> REV_OUT_W<7> {
        REV_OUT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CRC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crc_cr](index.html) module
pub struct CRC_CR_SPEC;
impl crate::RegisterSpec for CRC_CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [crc_cr::R](R) reader structure
impl crate::Readable for CRC_CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crc_cr::W](W) writer structure
impl crate::Writable for CRC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRC_CR to value 0
impl crate::Resettable for CRC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
