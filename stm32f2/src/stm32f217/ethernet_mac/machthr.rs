///Register `MACHTHR` reader
pub struct R(crate::R<MACHTHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHTHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHTHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHTHR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACHTHR` writer
pub struct W(crate::W<MACHTHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACHTHR_SPEC>;
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
impl From<crate::W<MACHTHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACHTHR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HTH` reader - Hash table high
pub type HTH_R = crate::FieldReader<u32, u32>;
///Field `HTH` writer - Hash table high
pub type HTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACHTHR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Hash table high
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Hash table high
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HTH_W<0> {
        HTH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC hash table high register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [machthr](index.html) module
pub struct MACHTHR_SPEC;
impl crate::RegisterSpec for MACHTHR_SPEC {
    type Ux = u32;
}
///`read()` method returns [machthr::R](R) reader structure
impl crate::Readable for MACHTHR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [machthr::W](W) writer structure
impl crate::Writable for MACHTHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACHTHR to value 0
impl crate::Resettable for MACHTHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
