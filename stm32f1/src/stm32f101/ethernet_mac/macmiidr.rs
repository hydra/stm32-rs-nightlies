///Register `MACMIIDR` reader
pub struct R(crate::R<MACMIIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMIIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMIIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMIIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACMIIDR` writer
pub struct W(crate::W<MACMIIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMIIDR_SPEC>;
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
impl From<crate::W<MACMIIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMIIDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MD` reader - MII data
pub type MD_R = crate::FieldReader<u16, u16>;
///Field `MD` writer - MII data
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACMIIDR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - MII data
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - MII data
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC MII data register (ETH_MACMIIDR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macmiidr](index.html) module
pub struct MACMIIDR_SPEC;
impl crate::RegisterSpec for MACMIIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macmiidr::R](R) reader structure
impl crate::Readable for MACMIIDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macmiidr::W](W) writer structure
impl crate::Writable for MACMIIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACMIIDR to value 0
impl crate::Resettable for MACMIIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
