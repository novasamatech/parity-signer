//
//  DerivedKeyExportModel.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 14/12/2022.
//

import Foundation

struct DerivedKeyExportModel: Equatable, Identifiable {
    let id = UUID()
    let viewModel: DerivedKeyRowViewModel
    let keyData: MKeyAndNetworkCard
}
