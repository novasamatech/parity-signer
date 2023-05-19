//
//  GenerateSigner.swift
//
//
//  Created by Krzysztof Rodak on 18/05/2023.
//

import Foundation
import PackagePlugin

@main
struct GenerateSigner: BuildToolPlugin {
    func createBuildCommands(context: PluginContext, target: Target) throws -> [Command] {
        print("TEST TEST")
        let fileManager = FileManager.default
        let currentDirectory = fileManager.currentDirectoryPath
        let inputFilePath = URL(fileURLWithPath: currentDirectory)
            .appendingPathComponent("../../../rust/signer/src/signer.udl").path

        guard FileManager.default.fileExists(atPath: inputFilePath) else {
            Diagnostics.error("Could not find .udl at: \(inputFilePath)")
            return []
        }
        let command: Command = .prebuildCommand(
            displayName: "UnifiBindgen BuildTool Plugin",
            executable: Path(URL(fileURLWithPath: "\(NSHomeDirectory())/.cargo/bin/uniffi-bindgen").relativePath),
            arguments: [
                "generate",
                inputFilePath,
                "--language",
                "swift",
                "--out-dir",
                "\(target.directory)/Generated"
            ],
            environment: [
                "PROJECT_DIR": context.package.directory,
                "TARGET_NAME": target.name,
                "PRODUCT_MODULE_NAME": target.moduleName,
                "DERIVED_SOURCES_DIR": context.pluginWorkDirectory
            ],
            outputFilesDirectory: Path("\(target.directory)/Generated")
        )
        print("MY COMMAND: \(command)")
        return [command]
    }
}

extension Target {
    /// Try to access the underlying `moduleName` property
    /// Falls back to target's name
    var moduleName: String {
        switch self {
        case let target as SourceModuleTarget:
            return target.moduleName
        default:
            return ""
        }
    }
}
