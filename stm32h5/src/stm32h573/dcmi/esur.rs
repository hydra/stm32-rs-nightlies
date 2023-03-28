///Register `ESUR` reader
pub struct R(crate::R<ESUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESUR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ESUR` writer
pub struct W(crate::W<ESUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESUR_SPEC>;
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
impl From<crate::W<ESUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESUR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FSU` reader - Frame start delimiter unmask This byte specifies the mask to be applied to the code of the frame start delimiter.
pub type FSU_R = crate::FieldReader<u8, u8>;
///Field `FSU` writer - Frame start delimiter unmask This byte specifies the mask to be applied to the code of the frame start delimiter.
pub type FSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESUR_SPEC, u8, u8, 8, O>;
///Field `LSU` reader - Line start delimiter unmask This byte specifies the mask to be applied to the code of the line start delimiter.
pub type LSU_R = crate::FieldReader<u8, u8>;
///Field `LSU` writer - Line start delimiter unmask This byte specifies the mask to be applied to the code of the line start delimiter.
pub type LSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESUR_SPEC, u8, u8, 8, O>;
///Field `LEU` reader - Line end delimiter unmask This byte specifies the mask to be applied to the code of the line end delimiter.
pub type LEU_R = crate::FieldReader<u8, u8>;
///Field `LEU` writer - Line end delimiter unmask This byte specifies the mask to be applied to the code of the line end delimiter.
pub type LEU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESUR_SPEC, u8, u8, 8, O>;
///Field `FEU` reader - Frame end delimiter unmask This byte specifies the mask to be applied to the code of the frame end delimiter.
pub type FEU_R = crate::FieldReader<u8, u8>;
///Field `FEU` writer - Frame end delimiter unmask This byte specifies the mask to be applied to the code of the frame end delimiter.
pub type FEU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ESUR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Frame start delimiter unmask This byte specifies the mask to be applied to the code of the frame start delimiter.
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Line start delimiter unmask This byte specifies the mask to be applied to the code of the line start delimiter.
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Line end delimiter unmask This byte specifies the mask to be applied to the code of the line end delimiter.
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Frame end delimiter unmask This byte specifies the mask to be applied to the code of the frame end delimiter.
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Frame start delimiter unmask This byte specifies the mask to be applied to the code of the frame start delimiter.
    #[inline(always)]
    #[must_use]
    pub fn fsu(&mut self) -> FSU_W<0> {
        FSU_W::new(self)
    }
    ///Bits 8:15 - Line start delimiter unmask This byte specifies the mask to be applied to the code of the line start delimiter.
    #[inline(always)]
    #[must_use]
    pub fn lsu(&mut self) -> LSU_W<8> {
        LSU_W::new(self)
    }
    ///Bits 16:23 - Line end delimiter unmask This byte specifies the mask to be applied to the code of the line end delimiter.
    #[inline(always)]
    #[must_use]
    pub fn leu(&mut self) -> LEU_W<16> {
        LEU_W::new(self)
    }
    ///Bits 24:31 - Frame end delimiter unmask This byte specifies the mask to be applied to the code of the frame end delimiter.
    #[inline(always)]
    #[must_use]
    pub fn feu(&mut self) -> FEU_W<24> {
        FEU_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DCMI embedded synchronization unmask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [esur](index.html) module
pub struct ESUR_SPEC;
impl crate::RegisterSpec for ESUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [esur::R](R) reader structure
impl crate::Readable for ESUR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [esur::W](W) writer structure
impl crate::Writable for ESUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ESUR to value 0
impl crate::Resettable for ESUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
