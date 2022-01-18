#[doc = "Register `SCRATCH5` reader"]
pub struct R(crate::R<SCRATCH5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCRATCH5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCRATCH5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCRATCH5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCRATCH5` writer"]
pub struct W(crate::W<SCRATCH5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCRATCH5_SPEC>;
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
impl From<crate::W<SCRATCH5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCRATCH5_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scratch register. Information persists through soft reset of the chip.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [scratch5](index.html) module"]
pub struct SCRATCH5_SPEC;
impl crate::RegisterSpec for SCRATCH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scratch5::R](R) reader structure"]
impl crate::Readable for SCRATCH5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scratch5::W](W) writer structure"]
impl crate::Writable for SCRATCH5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCRATCH5 to value 0"]
impl crate::Resettable for SCRATCH5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
impl crate::markers::AtomicMarker for SCRATCH5_SPEC {}
