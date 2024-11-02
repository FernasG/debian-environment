pub enum PackageType {
    UTILITY,
    STYLE,
    CODE
}

pub struct Package {
    pub name: &'static str,
    pub filename: &'static str,
    pub package_type: PackageType
}

pub const PACKAGES: [Package; 3] = [
    Package {
        name: "Visual Studio Code",
        filename: "vscode.sh",
        package_type: PackageType::CODE
    },
    Package {
        name: "BitWarden",
        filename: "bitwarden.sh",
        package_type: PackageType::UTILITY
    },
    Package {
        name: "GNOME Tweaks",
        filename: "tweaks.sh",
        package_type: PackageType::STYLE
    }
];
