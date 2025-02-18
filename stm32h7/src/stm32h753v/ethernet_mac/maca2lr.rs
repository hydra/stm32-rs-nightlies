///Register `MACA2LR` reader
pub struct R(crate::R<MACA2LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA2LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA2LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA2LR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACA2LR` writer
pub struct W(crate::W<MACA2LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA2LR_SPEC>;
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
impl From<crate::W<MACA2LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA2LR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDRLO` reader - MAC Address 2 \[31:0\]
pub type ADDRLO_R = crate::FieldReader<u32, u32>;
///Field `ADDRLO` writer - MAC Address 2 \[31:0\]
pub type ADDRLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA2LR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - MAC Address 2 \[31:0\]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - MAC Address 2 \[31:0\]
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> ADDRLO_W<0> {
        ADDRLO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Address 2 low register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca2lr](index.html) module
pub struct MACA2LR_SPEC;
impl crate::RegisterSpec for MACA2LR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maca2lr::R](R) reader structure
impl crate::Readable for MACA2LR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maca2lr::W](W) writer structure
impl crate::Writable for MACA2LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACA2LR to value 0xffff_ffff
impl crate::Resettable for MACA2LR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
