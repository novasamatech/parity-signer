//
//  PasswordProtectionStatePublisher.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 25/01/2023.
//

import Combine
import LocalAuthentication
import UIKit

final class PasswordProtectionStatePublisher: ObservableObject {
    @Published var isProtected: Bool = true
    private var cancelBag = CancelBag()

    init() {
        updateProtectionStatus()
        NotificationCenter.default
            .publisher(for: UIApplication.willEnterForegroundNotification)
            .sink { [weak self] _ in self?.updateProtectionStatus() }
            .store(in: cancelBag)
    }

    private func updateProtectionStatus() {
        Future { promise in
            let context = LAContext()
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
