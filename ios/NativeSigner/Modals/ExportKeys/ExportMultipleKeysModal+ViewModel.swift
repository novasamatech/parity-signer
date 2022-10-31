//
//  ExportMultipleKeysModal+ViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 24/10/2022.
//

import SwiftUI

extension ExportMultipleKeysModal {
    final class ViewModel: ObservableObject {
        private let keysExportService: ExportMultipleKeysService
        let viewModel: ExportMultipleKeysModalViewModel

        @Published var qrCode: AnimatedQRCodeViewModel = .init(qrCodes: [])
        @Published var isShowingKeysExportModal = false
        @Published var animateBackground: Bool = false

        @Binding var isPresented: Bool

        init(
            viewModel: ExportMultipleKeysModalViewModel,
            keysExportService: ExportMultipleKeysService = ExportMultipleKeysService(),
            isPresented: Binding<Bool>
        ) {
            self.viewModel = viewModel
            self.keysExportService = keysExportService
            _isPresented = isPresented
        }

        func prepareKeysExport() {
            let completion: (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void = { result in
                self.qrCode = (try? result.get()) ?? .init(qrCodes: [])
            }
            switch viewModel.selectedItems {
            case .keySets:
                keysExportService.exportMultipleKeySets(seedNames: viewModel.seedNames, completion)
            case let .keys(key, derivedKeys):
                keysExportService.exportRootWithDerivedKeys(
                    seedName: key.keyName,
                    keys: derivedKeys.map(\.keyData),
                    completion
                )
            }
        }
    }
}
