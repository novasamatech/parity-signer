// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "Jdenticon",
    platforms: [
        .iOS(.v15)
    ],
    products: [
        .library(
            name: "Jdenticon",
            targets: ["Jdenticon"]
        )
    ],
    dependencies: [
        .package(url: "https://github.com/exyte/SVGView.git", branch: "main")
    ],
    targets: [
        .target(
            name: "Jdenticon",
            dependencies: ["SVGView"]
        )
    ]
)
