// swift-tools-version: 5.6
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Signer",
    platforms: [
        .iOS(.v15)
    ],
    products: [
        .library(
            name: "Signer",
            targets: ["Signer"]
        )
    ],
    dependencies: [],
    targets: [
        .target(
            name: "Signer",
            dependencies: ["signer"],
            exclude: [
                "Frameworks"
            ],
            publicHeadersPath: "Signer/Generated/**",
            cSettings: [
                .headerSearchPath("Signer/Generated/**")
            ],
            linkerSettings: [
                .linkedLibrary("libresolv.tbd")
            ],
            plugins: ["GenerateSigner"]
        ),
        .binaryTarget(name: "signer", path: "Signer/Frameworks/signer.xcframework"),
        .plugin(
            name: "GenerateRustSigner",
            capability: .command(
                intent: .custom(
                    verb: "rust-code-generation",
                    description: "Generates Swift files from a given set of inputs."
                ),
                permissions: [
                    .writeToPackageDirectory(reason: "Need access to the package directory to generate files.")
                ]
            ),
            dependencies: []
        ),
        .plugin(
            name: "GenerateSigner",
            capability: .buildTool(),
            dependencies: []
        )
    ]
)
