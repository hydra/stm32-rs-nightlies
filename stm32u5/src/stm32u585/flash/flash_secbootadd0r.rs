///Register `FLASH_SECBOOTADD0R` reader
pub struct R(crate::R<FLASH_SECBOOTADD0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SECBOOTADD0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SECBOOTADD0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SECBOOTADD0R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SECBOOTADD0R` writer
pub struct W(crate::W<FLASH_SECBOOTADD0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SECBOOTADD0R_SPEC>;
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
impl From<crate::W<FLASH_SECBOOTADD0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SECBOOTADD0R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BOOT_LOCK` reader - Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\[24:0\]
///option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0.
pub type BOOT_LOCK_R = crate::BitReader<bool>;
///Field `BOOT_LOCK` writer - Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\[24:0\]
///option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0.
pub type BOOT_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECBOOTADD0R_SPEC, bool, O>;
///Field `SECBOOTADD0` reader - Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \[31:7\]
///The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\[24:0\]
///= 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\[24:0\]
///= 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\[24:0\]
///= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)
pub type SECBOOTADD0_R = crate::FieldReader<u32, u32>;
///Field `SECBOOTADD0` writer - Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \[31:7\]
///The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\[24:0\]
///= 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\[24:0\]
///= 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\[24:0\]
///= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)
pub type SECBOOTADD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SECBOOTADD0R_SPEC, u32, u32, 25, O>;
impl R {
    ///Bit 0 - Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\[24:0\]
    ///option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0.
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bits 7:31 - Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \[31:7\]
    ///The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\[24:0\]
    ///= 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\[24:0\]
    ///= 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\[24:0\]
    ///= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)
    #[inline(always)]
    pub fn secbootadd0(&self) -> SECBOOTADD0_R {
        SECBOOTADD0_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    ///Bit 0 - Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\[24:0\]
    ///option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0.
    #[inline(always)]
    #[must_use]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<0> {
        BOOT_LOCK_W::new(self)
    }
    ///Bits 7:31 - Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \[31:7\]
    ///The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\[24:0\]
    ///= 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\[24:0\]
    ///= 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\[24:0\]
    ///= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)
    #[inline(always)]
    #[must_use]
    pub fn secbootadd0(&mut self) -> SECBOOTADD0_W<7> {
        SECBOOTADD0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure boot address 0 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_secbootadd0r](index.html) module
pub struct FLASH_SECBOOTADD0R_SPEC;
impl crate::RegisterSpec for FLASH_SECBOOTADD0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_secbootadd0r::R](R) reader structure
impl crate::Readable for FLASH_SECBOOTADD0R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_secbootadd0r::W](W) writer structure
impl crate::Writable for FLASH_SECBOOTADD0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SECBOOTADD0R to value 0
impl crate::Resettable for FLASH_SECBOOTADD0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
