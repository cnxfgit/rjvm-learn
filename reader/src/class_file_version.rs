

#[derive(Debug, PartialEq, Default, strum_macros::Display)]
#[allow(dead_code)]
pub enum ClassFileVersion {
    Jdk1_1,
    Jdk1_2,
    Jdk1_3,
    Jdk1_4,
    Jdk1_5,
    Jdk6,
    Jdk7,
    #[default]
    Jdk8,
    Jdk9,
    Jdk10,
    Jdk11,
    Jdk12,
    Jdk13,
    Jdk14,
    Jdk15,
    Jdk16,
    Jdk17,
    Jdk18,
    Jdk19,
    Jdk20,
    Jdk21,
    Jdk22,
}