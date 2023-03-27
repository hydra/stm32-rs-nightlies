///Register `IDMABASER` reader
pub struct R(crate::R<IDMABASER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDMABASER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDMABASER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDMABASER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IDMABASER` writer
pub struct W(crate::W<IDMABASER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDMABASER_SPEC>;
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
impl From<crate::W<IDMABASER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDMABASER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IDMABASE` reader - Buffer memory base address bits \[31:2\], must be word aligned (bit \[1:0\]
///are always 0 and read only) This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1).
pub type IDMABASE_R = crate::FieldReader<u32, u32>;
///Field `IDMABASE` writer - Buffer memory base address bits \[31:2\], must be word aligned (bit \[1:0\]
///are always 0 and read only) This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1).
pub type IDMABASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDMABASER_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Buffer memory base address bits \[31:2\], must be word aligned (bit \[1:0\]
    ///are always 0 and read only) This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1).
    #[inline(always)]
    pub fn idmabase(&self) -> IDMABASE_R {
        IDMABASE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Buffer memory base address bits \[31:2\], must be word aligned (bit \[1:0\]
    ///are always 0 and read only) This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can dynamically be written by firmware when DPSM active (DPSMACT = 1).
    #[inline(always)]
    #[must_use]
    pub fn idmabase(&mut self) -> IDMABASE_W<0> {
        IDMABASE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDMMC IDMA buffer base address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idmabaser](index.html) module
pub struct IDMABASER_SPEC;
impl crate::RegisterSpec for IDMABASER_SPEC {
    type Ux = u32;
}
///`read()` method returns [idmabaser::R](R) reader structure
impl crate::Readable for IDMABASER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [idmabaser::W](W) writer structure
impl crate::Writable for IDMABASER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IDMABASER to value 0
impl crate::Resettable for IDMABASER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
