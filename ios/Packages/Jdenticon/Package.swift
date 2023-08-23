// swift-tools-version: 5.8

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
    dependencies: [],
    targets: [
        .target(
            name: "Jdenticon",
            dependencies: []
        ),
        .testTarget(
            name: "JdenticonTests",
            dependencies: ["Jdenticon"]
        )
    ]
)
