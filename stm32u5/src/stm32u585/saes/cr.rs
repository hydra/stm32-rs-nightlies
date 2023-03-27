///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - SAES enable
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - SAES enable
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DATATYPE` reader - DATATYPE
pub type DATATYPE_R = crate::FieldReader<u8, u8>;
///Field `DATATYPE` writer - DATATYPE
pub type DATATYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `MODE` reader - MODE
pub type MODE_R = crate::FieldReader<u8, u8>;
///Field `MODE` writer - MODE
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `CHMOD` reader - CHMOD
pub type CHMOD_R = crate::FieldReader<u8, u8>;
///Field `CHMOD` writer - CHMOD
pub type CHMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `DMAINEN` reader - DMAINEN
pub type DMAINEN_R = crate::BitReader<bool>;
///Field `DMAINEN` writer - DMAINEN
pub type DMAINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DMAOUTEN` reader - DMAOUTEN
pub type DMAOUTEN_R = crate::BitReader<bool>;
///Field `DMAOUTEN` writer - DMAOUTEN
pub type DMAOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `KEYSIZE` reader - KEYSIZE
pub type KEYSIZE_R = crate::BitReader<bool>;
///Field `KEYSIZE` writer - KEYSIZE
pub type KEYSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `KEYPROT` reader - KEYPROT
pub type KEYPROT_R = crate::BitReader<bool>;
///Field `KEYPROT` writer - KEYPROT
pub type KEYPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `KMOD` reader - KMOD
pub type KMOD_R = crate::FieldReader<u8, u8>;
///Field `KMOD` writer - KMOD
pub type KMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `KSHAREID` reader - KSHAREID
pub type KSHAREID_R = crate::FieldReader<u8, u8>;
///Field `KSHAREID` writer - KSHAREID
pub type KSHAREID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `KEYSEL` reader - KEYSEL
pub type KEYSEL_R = crate::FieldReader<u8, u8>;
///Field `KEYSEL` writer - KEYSEL
pub type KEYSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `IPRST` reader - IPRST
pub type IPRST_R = crate::BitReader<bool>;
///Field `IPRST` writer - IPRST
pub type IPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SAES enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DATATYPE
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - MODE
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - CHMOD
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 11 - DMAINEN
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMAOUTEN
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 18 - KEYSIZE
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - KEYPROT
    #[inline(always)]
    pub fn keyprot(&self) -> KEYPROT_R {
        KEYPROT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:25 - KMOD
    #[inline(always)]
    pub fn kmod(&self) -> KMOD_R {
        KMOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - KSHAREID
    #[inline(always)]
    pub fn kshareid(&self) -> KSHAREID_R {
        KSHAREID_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:30 - KEYSEL
    #[inline(always)]
    pub fn keysel(&self) -> KEYSEL_R {
        KEYSEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - IPRST
    #[inline(always)]
    pub fn iprst(&self) -> IPRST_R {
        IPRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SAES enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bits 1:2 - DATATYPE
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<1> {
        DATATYPE_W::new(self)
    }
    ///Bits 3:4 - MODE
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    ///Bits 5:6 - CHMOD
    #[inline(always)]
    #[must_use]
    pub fn chmod(&mut self) -> CHMOD_W<5> {
        CHMOD_W::new(self)
    }
    ///Bit 11 - DMAINEN
    #[inline(always)]
    #[must_use]
    pub fn dmainen(&mut self) -> DMAINEN_W<11> {
        DMAINEN_W::new(self)
    }
    ///Bit 12 - DMAOUTEN
    #[inline(always)]
    #[must_use]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W<12> {
        DMAOUTEN_W::new(self)
    }
    ///Bit 18 - KEYSIZE
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<18> {
        KEYSIZE_W::new(self)
    }
    ///Bit 19 - KEYPROT
    #[inline(always)]
    #[must_use]
    pub fn keyprot(&mut self) -> KEYPROT_W<19> {
        KEYPROT_W::new(self)
    }
    ///Bits 24:25 - KMOD
    #[inline(always)]
    #[must_use]
    pub fn kmod(&mut self) -> KMOD_W<24> {
        KMOD_W::new(self)
    }
    ///Bits 26:27 - KSHAREID
    #[inline(always)]
    #[must_use]
    pub fn kshareid(&mut self) -> KSHAREID_W<26> {
        KSHAREID_W::new(self)
    }
    ///Bits 28:30 - KEYSEL
    #[inline(always)]
    #[must_use]
    pub fn keysel(&mut self) -> KEYSEL_W<28> {
        KEYSEL_W::new(self)
    }
    ///Bit 31 - IPRST
    #[inline(always)]
    #[must_use]
    pub fn iprst(&mut self) -> IPRST_W<31> {
        IPRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
