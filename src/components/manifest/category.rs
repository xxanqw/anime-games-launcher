use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentCategory {
    Translation,
    Virtualisation,
    Runtime,
    General
}

impl std::fmt::Display for ComponentCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Translation    => f.write_str("translation"),
            Self::Virtualisation => f.write_str("virtualisation"),
            Self::Runtime        => f.write_str("runtime"),
            Self::General        => f.write_str("general")
        }
    }
}

impl std::str::FromStr for ComponentCategory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "translation"    => Ok(Self::Translation),
            "virtualisation" => Ok(Self::Virtualisation),
            "runtime"        => Ok(Self::Runtime),
            "general"        => Ok(Self::General),

            _ => anyhow::bail!("Unsupported component category: {s}")
        }
    }
}

impl AsHash for ComponentCategory {
    #[inline]
    fn hash(&self) -> Hash {
        self.to_string().hash()
    }
}
