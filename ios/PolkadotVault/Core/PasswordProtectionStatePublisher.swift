//
//  PasswordProtectionStatePublisher.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 25/01/2023.
//

import Combine
import LocalAuthentication
import UIKit

// sourcery: AutoMockable
protocol LAContextProtocol {
    func canEvaluatePolicy(_ policy: LAPolicy, error: NSErrorPointer) -> Bool
}

extension LAContext: LAContextProtocol {}

final class PasswordProtectionStatePublisher: ObservableObject {
    @Published var isProtected: Bool = true
    private var cancelBag = CancelBag()
    private let notificationCenter: NotificationCenter

    init(
        context: LAContextProtocol = LAContext(),
        notificationCenter: NotificationCenter = NotificationCenter.default
    ) {
        self.notificationCenter = notificationCenter
        updateProtectionStatus(context: context)

        notificationCenter.publisher(for: UIApplication.willEnterForegroundNotification)
            .sink { [weak self] _ in self?.updateProtectionStatus(context: context) }
            .store(in: cancelBag)
    }

    private func updateProtectionStatus(context: LAContextProtocol) {
        Future { promise in
            let isPasswordProtected = context.canEvaluatePolicy(.deviceOwnerAuthentication, error: nil)
            promise(.success(isPasswordProtected))
        }
        .receive(on: DispatchQueue.main)
        .sink(receiveValue: { [weak self] success in
            self?.isProtected = success
        })
        .store(in: cancelBag)
    }
}
