//
//  DerivedKeyExportModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/10/2022.
//

import Foundation

struct DerivedKeyExportModel: Equatable {
    let viewModel: DerivedKeyRowViewModel
    let keyData: MKeyAndNetworkCard
}
