///Register `HSEM_R20` reader
pub struct R(crate::R<HSEM_R20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_R20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_R20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_R20_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HSEM_R20` writer
pub struct W(crate::W<HSEM_R20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEM_R20_SPEC>;
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
impl From<crate::W<HSEM_R20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEM_R20_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PROCID` reader - PROCID
pub type PROCID_R = crate::FieldReader<u8, u8>;
///Field `PROCID` writer - PROCID
pub type PROCID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSEM_R20_SPEC, u8, u8, 8, O>;
///Field `COREID` reader - COREID
pub type COREID_R = crate::FieldReader<u8, u8>;
///Field `COREID` writer - COREID
pub type COREID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSEM_R20_SPEC, u8, u8, 4, O>;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader<bool>;
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSEM_R20_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - PROCID
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - COREID
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - PROCID
    #[inline(always)]
    #[must_use]
    pub fn procid(&mut self) -> PROCID_W<0> {
        PROCID_W::new(self)
    }
    ///Bits 8:11 - COREID
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<8> {
        COREID_W::new(self)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hsem_r20](index.html) module
pub struct HSEM_R20_SPEC;
impl crate::RegisterSpec for HSEM_R20_SPEC {
    type Ux = u32;
}
///`read()` method returns [hsem_r20::R](R) reader structure
impl crate::Readable for HSEM_R20_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hsem_r20::W](W) writer structure
impl crate::Writable for HSEM_R20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HSEM_R20 to value 0
impl crate::Resettable for HSEM_R20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
