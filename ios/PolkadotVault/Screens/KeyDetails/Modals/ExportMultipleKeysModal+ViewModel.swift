//
//  ExportMultipleKeysModal+ViewModel.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 24/10/2022.
//

import SwiftUI

extension ExportMultipleKeysModal {
    final class ViewModel: ObservableObject {
        private let keyExportService: ExportKeySetService
        let viewModel: ExportMultipleKeysModalViewModel

        @Published var qrCode: AnimatedQRCodeViewModel = .init(qrCodes: [])
        @Published var isShowingKeysExportModal = false
        @Published var animateBackground: Bool = false

        @Binding var isPresented: Bool

        init(
            viewModel: ExportMultipleKeysModalViewModel,
            keyExportService: ExportKeySetService = ExportKeySetService(),
            isPresented: Binding<Bool>
        ) {
            self.viewModel = viewModel
            self.keyExportService = keyExportService
            _isPresented = isPresented
        }

        func prepareKeysExport() {
            let completion: (Result<AnimatedQRCodeViewModel, ServiceError>) -> Void = { result in
                self.qrCode = (try? result.get()) ?? .init(qrCodes: [])
            }
            keyExportService.exportRootWithDerivedKeys(
                seedName: viewModel.keyName,
                keys: viewModel.derivedKeys.map(\.keyData),
                completion
            )
        }
    }
}
