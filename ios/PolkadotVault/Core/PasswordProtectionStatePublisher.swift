//
//  PasswordProtectionStatePublisher.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 25/01/2023.
//

import Combine
import LocalAuthentication
import UIKit

protocol LAContextProtocol {
    func canEvaluatePolicy(_ policy: LAPolicy, error: NSErrorPointer) -> Bool
}

extension LAContext: LAContextProtocol {}

final class PasswordProtectionStatePublisher: ObservableObject {
    @Published var isProtected: Bool = true
    private var cancelBag = CancelBag()
    private let context: LAContextProtocol

    init(context: LAContextProtocol = LAContext()) {
        self.context = context
        updateProtectionStatus()
        NotificationCenter.default
            .publisher(for: UIApplication.willEnterForegroundNotification)
            .sink { [weak self] _ in self?.updateProtectionStatus() }
            .store(in: cancelBag)
    }

    private func updateProtectionStatus() {
        Future { promise in
            let isPasswordProtected = self.context.canEvaluatePolicy(.deviceOwnerAuthentication, error: nil)
            promise(.success(isPasswordProtected))
        }
        .receive(on: DispatchQueue.main)
        .sink(receiveValue: { [weak self] success in
            self?.isProtected = success
        })
        .store(in: cancelBag)
    }
}
