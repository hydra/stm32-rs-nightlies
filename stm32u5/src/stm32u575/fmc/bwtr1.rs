///Register `BWTR1` reader
pub struct R(crate::R<BWTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BWTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BWTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BWTR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BWTR1` writer
pub struct W(crate::W<BWTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BWTR1_SPEC>;
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
impl From<crate::W<BWTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BWTR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDSET` reader - Address setup phase duration
pub type ADDSET_R = crate::FieldReader<u8, u8>;
///Field `ADDSET` writer - Address setup phase duration
pub type ADDSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 4, O>;
///Field `ADDHLD` reader - Address-hold phase duration
pub type ADDHLD_R = crate::FieldReader<u8, u8>;
///Field `ADDHLD` writer - Address-hold phase duration
pub type ADDHLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 4, O>;
///Field `DATAST` reader - Data-phase duration
pub type DATAST_R = crate::FieldReader<u8, u8>;
///Field `DATAST` writer - Data-phase duration
pub type DATAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 8, O>;
///Field `BUSTURN` reader - Bus turnaround phase duration
pub type BUSTURN_R = crate::FieldReader<u8, u8>;
///Field `BUSTURN` writer - Bus turnaround phase duration
pub type BUSTURN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 4, O>;
///Field `ACCMOD` reader - Access mode
pub type ACCMOD_R = crate::FieldReader<u8, u8>;
///Field `ACCMOD` writer - Access mode
pub type ACCMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 2, O>;
///Field `DATAHLD` reader - Data hold phase duration
pub type DATAHLD_R = crate::FieldReader<u8, u8>;
///Field `DATAHLD` writer - Data hold phase duration
pub type DATAHLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BWTR1_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:3 - Address setup phase duration
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Address-hold phase duration
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - Data-phase duration
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Bus turnaround phase duration
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 28:29 - Access mode
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Data hold phase duration
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:3 - Address setup phase duration
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<0> {
        ADDSET_W::new(self)
    }
    ///Bits 4:7 - Address-hold phase duration
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<4> {
        ADDHLD_W::new(self)
    }
    ///Bits 8:15 - Data-phase duration
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<8> {
        DATAST_W::new(self)
    }
    ///Bits 16:19 - Bus turnaround phase duration
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<16> {
        BUSTURN_W::new(self)
    }
    ///Bits 28:29 - Access mode
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<28> {
        ACCMOD_W::new(self)
    }
    ///Bits 30:31 - Data hold phase duration
    #[inline(always)]
    #[must_use]
    pub fn datahld(&mut self) -> DATAHLD_W<30> {
        DATAHLD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SRAM/NOR-Flash write timing registers 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bwtr1](index.html) module
pub struct BWTR1_SPEC;
impl crate::RegisterSpec for BWTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [bwtr1::R](R) reader structure
impl crate::Readable for BWTR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bwtr1::W](W) writer structure
impl crate::Writable for BWTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BWTR1 to value 0x0fff_ffff
impl crate::Resettable for BWTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
