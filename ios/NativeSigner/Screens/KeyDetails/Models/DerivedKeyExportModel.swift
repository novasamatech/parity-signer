//
//  DerivedKeyExportModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 14/12/2022.
//

import Foundation

struct DerivedKeyExportModel: Equatable {
    let viewModel: DerivedKeyRowViewModel
    let keyData: MKeyAndNetworkCard
}
