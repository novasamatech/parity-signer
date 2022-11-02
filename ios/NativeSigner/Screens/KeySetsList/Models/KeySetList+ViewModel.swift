//
//  KeySetList+ViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/10/2022.
//

import SwiftUI

extension KeySetList {
    final class ViewModel: ObservableObject {
        let listViewModel: KeySetListViewModel
        let keyDetailsService: KeyDetailsService

        @Published var isShowingKeysExportModal = false

        init(
            listViewModel: KeySetListViewModel,
            keyDetailsService: KeyDetailsService = KeyDetailsService()
        ) {
            self.listViewModel = listViewModel
            self.keyDetailsService = keyDetailsService
        }

        func loadKeysInformation(
            for seedName: String,
            _ completion: @escaping (Result<MKeysNew, ServiceError>) -> Void
        ) {
            keyDetailsService.getKeys(for: seedName, completion)
        }
    }
}
