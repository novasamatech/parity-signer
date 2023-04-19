//
//  ApplicationStatePublisher.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 20/01/2023.
//

import Combine
import UIKit

enum ApplicationState: Equatable {
    case active
    case inactive
}

final class ApplicationStatePublisher: ObservableObject {
    @Published var applicationState: ApplicationState = .active

    private let cancelBag = CancelBag()

    init() {
        subscribe()
    }

    private func subscribe() {
        NotificationCenter.default
            .publisher(for: UIApplication.willResignActiveNotification)
            .sink { [weak self] _ in
                self?.applicationState = .inactive
            }
            .store(in: cancelBag)
        NotificationCenter.default
            .publisher(for: UIApplication.didBecomeActiveNotification)
            .sink { [weak self] _ in
                self?.applicationState = .active
            }
            .store(in: cancelBag)
    }
}
