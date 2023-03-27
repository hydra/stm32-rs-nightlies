///Register `MACA3HR` reader
pub struct R(crate::R<MACA3HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA3HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA3HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA3HR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACA3HR` writer
pub struct W(crate::W<MACA3HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA3HR_SPEC>;
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
impl From<crate::W<MACA3HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA3HR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MACA3H` reader - MACA3H
pub type MACA3H_R = crate::FieldReader<u16, u16>;
///Field `MACA3H` writer - MACA3H
pub type MACA3H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA3HR_SPEC, u16, u16, 16, O>;
///Field `MBC` reader - MBC
pub type MBC_R = crate::FieldReader<u8, u8>;
///Field `MBC` writer - MBC
pub type MBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA3HR_SPEC, u8, u8, 6, O>;
///Field `SA` reader - SA
pub type SA_R = crate::BitReader<bool>;
///Field `SA` writer - SA
pub type SA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACA3HR_SPEC, bool, O>;
///Field `AE` reader - AE
pub type AE_R = crate::BitReader<bool>;
///Field `AE` writer - AE
pub type AE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACA3HR_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - MACA3H
    #[inline(always)]
    pub fn maca3h(&self) -> MACA3H_R {
        MACA3H_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - SA
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AE
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - MACA3H
    #[inline(always)]
    #[must_use]
    pub fn maca3h(&mut self) -> MACA3H_W<0> {
        MACA3H_W::new(self)
    }
    ///Bits 24:29 - MBC
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<24> {
        MBC_W::new(self)
    }
    ///Bit 30 - SA
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<30> {
        SA_W::new(self)
    }
    ///Bit 31 - AE
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<31> {
        AE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC address 3 high register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca3hr](index.html) module
pub struct MACA3HR_SPEC;
impl crate::RegisterSpec for MACA3HR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maca3hr::R](R) reader structure
impl crate::Readable for MACA3HR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maca3hr::W](W) writer structure
impl crate::Writable for MACA3HR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACA3HR to value 0xffff
impl crate::Resettable for MACA3HR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
