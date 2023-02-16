//
//  CancelBag.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import Combine

final class CancelBag {
    fileprivate(set) var subscriptions = Set<AnyCancellable>()

    func cancel() {
        subscriptions.removeAll()
    }

    func collect(@Builder _ cancellables: () -> [AnyCancellable]) {
        subscriptions.formUnion(cancellables())
    }

    @resultBuilder
    enum Builder {
        static func buildBlock(_ cancellables: AnyCancellable...) -> [AnyCancellable] {
            cancellables
        }
    }
}

extension AnyCancellable {
    func store(in cancelBag: CancelBag) {
        cancelBag.subscriptions.insert(self)
    }
}
