#[doc = "Register `CPAR4` reader"]
pub type R = crate::R<Cpar4Spec>;
#[doc = "Register `CPAR4` writer"]
pub type W = crate::W<Cpar4Spec>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PaR = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<'_, Cpar4Spec> {
        PaW::new(self, 0)
    }
}
#[doc = "DMA channel 4 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpar4Spec;
impl crate::RegisterSpec for Cpar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar4::R`](R) reader structure"]
impl crate::Readable for Cpar4Spec {}
#[doc = "`write(|w| ..)` method takes [`cpar4::W`](W) writer structure"]
impl crate::Writable for Cpar4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPAR4 to value 0"]
impl crate::Resettable for Cpar4Spec {}
