//
//  JailbreakDetectionPublisher.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 20/01/2023.
//

import Combine
import Foundation
import UIKit

// sourcery: AutoMockable
protocol DeviceProtocol: AnyObject {
    var isSimulator: Bool { get }
}

final class JailbreakDetectionPublisher: ObservableObject {
    @Published var isJailbroken = false

    private let cancelBag = CancelBag()
    private let device: DeviceProtocol
    private let runtimePropertiesProvider: RuntimePropertiesProviding
    private let fileManager: FileManagingProtocol
    private let urlOpener: URLOpening
    private let processInfo: ProcessInfoProtocol

    init(
        runtimePropertiesProvider: RuntimePropertiesProviding = RuntimePropertiesProvider(),
        device: DeviceProtocol = UIDevice.current,
        fileManager: FileManagingProtocol = FileManager.default,
        urlOpener: URLOpening = UIApplication.shared,
        processInfo: ProcessInfoProtocol = ProcessInfo.processInfo
    ) {
        self.runtimePropertiesProvider = runtimePropertiesProvider
        self.device = device
        self.fileManager = fileManager
        self.urlOpener = urlOpener
        self.processInfo = processInfo
        guard runtimePropertiesProvider.runtimeMode == .production else { return }
        NotificationCenter.default
            .publisher(for: UIApplication.didBecomeActiveNotification)
            .map { _ in self.detectJailbreak() }
            .switchToLatest()
            .sink { self.isJailbroken = $0 }
            .store(in: cancelBag)
    }
}

private extension JailbreakDetectionPublisher {
    func detectJailbreak() -> AnyPublisher<Bool, Never> {
        Future { promise in
            let isJailbroken = [
                self.checkJailbreakFilesAndDirectories(),
                self.checkJailbreakTools(),
                self.checkSystemModifications(),
                self.checkEnvironmentVariables()
            ].contains(true)
            promise(.success(isJailbroken || self.device.isSimulator))
        }.eraseToAnyPublisher()
    }

    func checkJailbreakFilesAndDirectories() -> Bool {
        Constants.jailbreakApplicationPaths
            .contains { self.fileManager.fileExists(atPath: $0) }
    }

    func checkSystemModifications() -> Bool {
        Constants.inaccessibleSystemPaths
            .contains { self.fileManager.fileExists(atPath: $0) }
    }

    func checkJailbreakTools() -> Bool {
        let jailbreakTools = [
            Constants.jailbreakToolCydia,
            Constants.jailbreakToolIcy,
            Constants.jailbreakToolInstaller
        ]
        // swiftlint: disable:next force_unwrapping
        return jailbreakTools.contains { self.urlOpener.canOpenURL($0!) }
    }

    func checkEnvironmentVariables() -> Bool {
        let environmentVariables = [
            Constants.environmentVariableDyldInsertLibraries,
            Constants.environmentVariableDyldPrintToFile,
            Constants.environmentVariableDyldPrintOpts
        ]
        return environmentVariables.contains { self.processInfo.environment[$0] != nil }
    }
}

extension UIDevice: DeviceProtocol {
    var isSimulator: Bool {
        #if targetEnvironment(simulator)
            true
        #else
            false
        #endif
    }
}

private enum Constants {
    static let jailbreakToolCydia: URL! = URL(string: "cydia://")
    static let jailbreakToolIcy: URL! = URL(string: "icy://")
    static let jailbreakToolInstaller: URL! = URL(string: "installer://")
    static let environmentVariableDyldInsertLibraries = "DYLD_INSERT_LIBRARIES"
    static let environmentVariableDyldPrintToFile = "DYLD_PRINT_TO_FILE"
    static let environmentVariableDyldPrintOpts = "DYLD_PRINT_OPTS"
    static let jailbreakApplicationPaths: [String] = [
        "/Applications/Cydia.app",
        "/Applications/blackra1n.app",
        "/Applications/FakeCarrier.app",
        "/Applications/Icy.app",
        "/Applications/IntelliScreen.app",
        "/Applications/MxTube.app",
        "/Applications/RockApp.app",
        "/Applications/SBSettings.app",
        "/Applications/WinterBoard.app"
    ]

    static let inaccessibleSystemPaths: [String] = [
        "/Library/MobileSubstrate/DynamicLibraries/LiveClock.plist",
        "/Library/MobileSubstrate/DynamicLibraries/Veency.plist",
        "/private/var/lib/apt",
        "/private/var/lib/cydia",
        "/private/var/mobile/Library/SBSettings/Themes",
        "/private/var/stash",
        "/private/var/tmp/cydia.log",
        "/System/Library/LaunchDaemons/com.ikey.bbot.plist",
        "/System/Library/LaunchDaemons/com.saurik.Cydia.Startup.plist",
        "/usr/bin/sshd",
        "/bin/bash",
        "/usr/libexec/ssh-keysign",
        "/usr/libexec/sftp-server",
        "/usr/sbin/sshd",
        "/etc/apt",
        "/bin/sh",
        "/bin/su",
        "/etc/ssh/sshd_config",
        "/Library/MobileSubstrate/MobileSubstrate.dylib",
        "/pguntether",
        "/usr/bin/cycript",
        "/usr/bin/ssh",
        "/usr/sbin/frida-server",
        "/var/cache/apt",
        "/var/lib/cydia",
        "/var/log/syslog",
        "/var/mobile/Media/.evasi0n7_installed",
        "/var/tmp/cydia.log"
    ]
}
