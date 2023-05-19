//
//  GenerateRustSigner.swift
//
//
//  Created by Krzysztof Rodak on 18/05/2023.
//

import Foundation
import PackagePlugin

@main
struct GenerateRustSigner: CommandPlugin {
    func performCommand(context: PluginContext, arguments _: [String]) throws {
        let toolUrl = URL(fileURLWithPath: "\(NSHomeDirectory())/.cargo/bin/uniffi-bindgen")

        for target in context.package.targets {
            guard let target = target as? SourceModuleTarget else { continue }
            let fileManager = FileManager.default
            let currentDirectory = fileManager.currentDirectoryPath
            let inputFilePath = URL(fileURLWithPath: currentDirectory)
                .appendingPathComponent("rust/signer/src/signer.udl").relativePath

            guard FileManager.default.fileExists(atPath: inputFilePath) else {
                Diagnostics.error("Could not find .udl at: \(inputFilePath)")
                return
            }
            let outputDir = "\(target.directory)/Generated"

            print("Generating Swift files from \(inputFilePath) to \(outputDir).")

            let process = Process()
            process.executableURL = toolUrl
            process.arguments = [
                "generate",
                inputFilePath,
                "--language",
                "swift",
                "--out-dir",
                outputDir
            ]

            try process.run()
            process.waitUntilExit()

            if process.terminationReason == .exit, process.terminationStatus == 0 {
                print("Generated Swift files from \(inputFilePath) to \(outputDir).")
            } else {
                let problem = "\(process.terminationReason):\(process.terminationStatus)"
                Diagnostics.error("uniffi-bindgen invocation failed: \(problem)")
            }
        }
    }
}
