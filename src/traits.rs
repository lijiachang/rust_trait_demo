/// 所有的角色都是人类。可以说话进行自我介绍
pub trait Speak {
    fn introduce(&self) -> ();
}

/// 除了患者，每个人都有临床技能（clinical skills）
pub trait ClinicalSkills {
    // 能开处方
    fn can_prescribe(&self) -> bool {
        false
    }
    // 能诊断
    fn can_diagnose(&self) -> bool {
        false
    }
    // 能给药
    fn can_administer_medication(&self) -> bool {
        true
    }
}

/// 医生和高级职业护士
pub trait AdvanceMedical {}
/// 患者
pub trait PatientRole{
    // 获取患者名字
    fn get_name(&self) -> String;
}