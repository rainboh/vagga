use std::fmt;
use std::rc::Rc;
use std::path::PathBuf;

use builder::commands as cmd;
use libc::{uid_t, gid_t};
use quire::validate as V;
use serde::de::{self, Deserializer, Deserialize};
use serde::de::{VariantAccess, Visitor, EnumAccess};

use build_step::{Step, BuildStep};

const COMMANDS: &'static [&'static str] = &[
    "Alpine",
    "AlpineRepo",
    "Ubuntu",
    "UbuntuRepo",
    "UbuntuRelease",
    "UbuntuPPA",
    "UbuntuUniverse",
    "AptTrust",
    "Repo",
    "Install",
    "BuildDeps",
    "Git",
    "GitInstall",
    "GitDescribe",
    "PipConfig",
    "Py2Install",
    "Py2Requirements",
    "Py3Install",
    "Py3Requirements",
    "Tar",
    "TarInstall",
    "Unzip",
    "Sh",
    "Cmd",
    "RunAs",
    "Env",
    "Text",
    "Copy",
    "Download",
    "EnsureDir",
    "CacheDirs",
    "EmptyDir",
    "Remove",
    "Depends",
    "Container",
    "Build",
    "SubConfig",
    "NpmConfig",
    "NpmDependencies",
    "YarnDependencies",
    "NpmInstall",
    "GemInstall",
    "GemBundle",
    "GemConfig",
    "ComposerInstall",
    "ComposerDependencies",
    "ComposerConfig",
];

pub enum CommandName {
    Alpine,
    AlpineRepo,
    Ubuntu,
    UbuntuRepo,
    UbuntuRelease,
    UbuntuPPA,
    UbuntuUniverse,
    AptTrust,
    Repo,
    Install,
    BuildDeps,
    Git,
    GitInstall,
    GitDescribe,
    PipConfig,
    Py2Install,
    Py2Requirements,
    Py3Install,
    Py3Requirements,
    Tar,
    TarInstall,
    Unzip,
    Sh,
    Cmd,
    RunAs,
    Env,
    Text,
    Copy,
    Download,
    EnsureDir,
    CacheDirs,
    EmptyDir,
    Remove,
    Depends,
    Container,
    Build,
    SubConfig,
    NpmConfig,
    NpmDependencies,
    YarnDependencies,
    NpmInstall,
    GemInstall,
    GemBundle,
    GemConfig,
    ComposerInstall,
    ComposerDependencies,
    ComposerConfig,
}

pub struct NameVisitor;
pub struct StepVisitor;

pub fn builder_validator<'x>() -> V::Enum<'x> {
    V::Enum::new()
    .option("Alpine", cmd::alpine::Alpine::config())
    .option("AlpineRepo", cmd::alpine::AlpineRepo::config())
    .option("Ubuntu", cmd::ubuntu::Ubuntu::config())
    .option("UbuntuRelease", cmd::ubuntu::UbuntuRelease::config())
    .option("UbuntuRepo", cmd::ubuntu::UbuntuRepo::config())
    .option("UbuntuPPA", cmd::ubuntu::UbuntuPPA::config())
    .option("UbuntuUniverse", cmd::ubuntu::UbuntuUniverse::config())
    .option("AptTrust", cmd::ubuntu::AptTrust::config())
    .option("Repo", cmd::packaging::Repo::config())
    .option("Install", cmd::packaging::Install::config())
    .option("BuildDeps", cmd::packaging::BuildDeps::config())
    .option("Container", cmd::subcontainer::Container::config())
    .option("SubConfig", cmd::subcontainer::SubConfig::config())
    .option("Build", cmd::subcontainer::Build::config())
    .option("Text", cmd::text::Text::config())
    .option("Copy", cmd::copy::Copy::config())

    .option("Sh", cmd::generic::Sh::config())
    .option("Cmd", cmd::generic::Cmd::config())
    .option("RunAs", cmd::generic::RunAs::config())
    .option("Remove", cmd::dirs::Remove::config())
    .option("EnsureDir", cmd::dirs::EnsureDir::config())
    .option("EmptyDir", cmd::dirs::EmptyDir::config())
    .option("CacheDirs", cmd::dirs::CacheDirs::config())
    .option("Env", cmd::generic::Env::config())
    .option("Depends", cmd::copy::Depends::config())
    .option("Git", cmd::vcs::Git::config())
    .option("GitInstall", cmd::vcs::GitInstall::config())
    .option("GitDescribe", cmd::vcs::GitDescribe::config())
    .option("Tar", cmd::tarcmd::Tar::config())
    .option("TarInstall", cmd::tarcmd::TarInstall::config())
    .option("Unzip", cmd::unzip::Unzip::config())
    .option("Download", cmd::download::Download::config())

    // Python
    .option("PipConfig", cmd::pip::PipConfig::config())
    .option("Py2Install", cmd::pip::Py2Install::config())
    .option("Py2Requirements", cmd::pip::Py2Requirements::config())
    .option("Py3Install", cmd::pip::Py3Install::config())
    .option("Py3Requirements", cmd::pip::Py3Requirements::config())

    // Node.js
    .option("NpmConfig", cmd::npm::NpmConfig::config())
    .option("NpmInstall", cmd::npm::NpmInstall::config())
    .option("NpmDependencies", cmd::npm::NpmDependencies::config())
    .option("YarnDependencies", cmd::npm::YarnDependencies::config())

    // Composer
    .option("ComposerConfig", cmd::composer::ComposerConfig::config())
    .option("ComposerInstall", cmd::composer::ComposerInstall::config())
    .option("ComposerDependencies",
        cmd::composer::ComposerDependencies::config())

    // Ruby
    .option("GemConfig", cmd::gem::GemConfig::config())
    .option("GemInstall", cmd::gem::GemInstall::config())
    .option("GemBundle", cmd::gem::GemBundle::config())
}

fn step<T: BuildStep + 'static, E>(val: Result<T, E>)
    -> Result<Step, E>
{
    val.map(|x| Step(Rc::new(x) as Rc<BuildStep>))
}

impl<'a> Visitor<'a> for NameVisitor {
    type Value = CommandName;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "build step is one of {}", COMMANDS.join(", "))
    }
    fn visit_str<E: de::Error>(self, val: &str) -> Result<CommandName, E> {
        use self::CommandName::*;
        let res = match val {
            "Alpine" => Alpine,
            "AlpineRepo" => AlpineRepo,
            "Ubuntu" => Ubuntu,
            "UbuntuRepo" => UbuntuRepo,
            "UbuntuRelease" => UbuntuRelease,
            "UbuntuPPA" => UbuntuPPA,
            "UbuntuUniverse" => UbuntuUniverse,
            "AptTrust" => AptTrust,
            "Repo" => Repo,
            "Install" => Install,
            "BuildDeps" => BuildDeps,
            "Git" => Git,
            "GitInstall" => GitInstall,
            "GitDescribe" => GitDescribe,
            "PipConfig" => PipConfig,
            "Py2Install" => Py2Install,
            "Py2Requirements" => Py2Requirements,
            "Py3Install" => Py3Install,
            "Py3Requirements" => Py3Requirements,
            "Tar" => Tar,
            "TarInstall" => TarInstall,
            "Unzip" => Unzip,
            "Sh" => Sh,
            "Cmd" => Cmd,
            "RunAs" => RunAs,
            "Env" => Env,
            "Text" => Text,
            "Copy" => Copy,
            "Download" => Download,
            "EnsureDir" => EnsureDir,
            "CacheDirs" => CacheDirs,
            "EmptyDir" => EmptyDir,
            "Remove" => Remove,
            "Depends" => Depends,
            "Container" => Container,
            "Build" => Build,
            "SubConfig" => SubConfig,
            "NpmConfig" => NpmConfig,
            "NpmDependencies" => NpmDependencies,
            "YarnDependencies" => YarnDependencies,
            "NpmInstall" => NpmInstall,
            "GemInstall" => GemInstall,
            "GemBundle" => GemBundle,
            "GemConfig" => GemConfig,
            "ComposerInstall" => ComposerInstall,
            "ComposerDependencies" => ComposerDependencies,
            "ComposerConfig" => ComposerConfig,
            _ => return Err(E::custom("invalid build step")),
        };
        Ok(res)
    }
}

impl<'a> Visitor<'a> for StepVisitor {
    type Value = Step;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "build step is one of {}", COMMANDS.join(", "))
    }
    fn visit_enum<A>(self, data: A) -> Result<Step, A::Error>
        where A: EnumAccess<'a>,
    {
        use self::CommandName::*;
        let (tag, v) = data.variant()?;
        match tag {
            Alpine => step(v.newtype_variant::<cmd::alpine::Alpine>()),
            AlpineRepo => step(v.newtype_variant::<cmd::alpine::AlpineRepo>()),
            Ubuntu => step(v.newtype_variant::<cmd::ubuntu::Ubuntu>()),
            /*
            UbuntuRepo => step(cmd::ubuntu::UbuntuRepo::deserialize(d)),
            UbuntuRelease => step(cmd::ubuntu::UbuntuRelease::deserialize(d)),
            UbuntuPPA => step(cmd::ubuntu::UbuntuPPA::deserialize(d)),
            UbuntuUniverse
            => step(cmd::ubuntu::UbuntuUniverse::deserialize(d)),
            AptTrust => step(cmd::ubuntu::AptTrust::deserialize(d)),
            Repo => step(cmd::packaging::Repo::deserialize(d)),
            Install => step(cmd::packaging::Install::deserialize(d)),
            BuildDeps => step(cmd::packaging::BuildDeps::deserialize(d)),
            Git => step(cmd::vcs::Git::deserialize(d)),
            GitInstall => step(cmd::vcs::GitInstall::deserialize(d)),
            GitDescribe => step(cmd::vcs::GitDescribe::deserialize(d)),
            PipConfig => step(cmd::pip::PipConfig::deserialize(d)),
            Py2Install => step(cmd::pip::Py2Install::deserialize(d)),
            Py2Requirements => step(cmd::pip::Py2Requirements::deserialize(d)),
            Py3Install => step(cmd::pip::Py3Install::deserialize(d)),
            Py3Requirements => step(cmd::pip::Py3Requirements::deserialize(d)),
            Tar => step(cmd::tarcmd::Tar::deserialize(d)),
            TarInstall => step(cmd::tarcmd::TarInstall::deserialize(d)),
            Unzip => step(cmd::unzip::Unzip::deserialize(d)),
            Sh => step(cmd::generic::Sh::deserialize(d)),
            Cmd => step(cmd::generic::Cmd::deserialize(d)),
            RunAs => step(cmd::generic::RunAs::deserialize(d)),
            Env => step(cmd::generic::Env::deserialize(d)),
            Text => step(cmd::text::Text::deserialize(d)),
            Copy => step(cmd::copy::Copy::deserialize(d)),
            Download => step(cmd::download::Download::deserialize(d)),
            EnsureDir => step(cmd::dirs::EnsureDir::deserialize(d)),
            CacheDirs => step(cmd::dirs::CacheDirs::deserialize(d)),
            EmptyDir => step(cmd::dirs::EmptyDir::deserialize(d)),
            Remove => step(cmd::dirs::Remove::deserialize(d)),
            Depends => step(cmd::copy::Depends::deserialize(d)),
            Container => step(cmd::subcontainer::Container::deserialize(d)),
            Build => step(cmd::subcontainer::Build::deserialize(d)),
            SubConfig => step(cmd::subcontainer::SubConfig::deserialize(d)),
            NpmConfig => step(cmd::npm::NpmConfig::deserialize(d)),
            NpmDependencies => step(cmd::npm::NpmDependencies::deserialize(d)),
            YarnDependencies
            => step(cmd::npm::YarnDependencies::deserialize(d)),
            NpmInstall => step(cmd::npm::NpmInstall::deserialize(d)),
            GemInstall => step(cmd::gem::GemInstall::deserialize(d)),
            GemBundle => step(cmd::gem::GemBundle::deserialize(d)),
            GemConfig => step(cmd::gem::GemConfig::deserialize(d)),
            ComposerInstall
            => step(cmd::composer::ComposerInstall::deserialize(d)),
            ComposerDependencies
            => step(cmd::composer::ComposerDependencies::deserialize(d)),
            ComposerConfig
            => step(cmd::composer::ComposerConfig::deserialize(d)),
            */
            _ => unimplemented!(),
        }
    }
}

impl<'a> Deserialize<'a> for CommandName {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<CommandName, D::Error>
    {
        d.deserialize_identifier(NameVisitor)
    }
}

impl<'a> Deserialize<'a> for Step {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<Step, D::Error> {
        d.deserialize_enum("BuildStep", COMMANDS, StepVisitor)
    }
}
